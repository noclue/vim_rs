use crate::monitor::Monitor;
use anyhow::{Context, Result};
use futures::{FutureExt, StreamExt};
use ratatui::crossterm::event::Event as CrosstermEvent;
use std::time::Duration;
use tokio::sync::mpsc;
use vim_rs::types::structs::ObjectUpdate;

/// The frequency at which tick events are emitted.
const TICK_FPS: f64 = 30.0;

/// Representation of all possible events.
#[derive(Debug)]
pub enum Event {
    /// An event that is emitted on a regular schedule.
    ///
    /// Use this event to run any code which has to run outside of being a direct response to a user
    /// event. e.g. polling exernal systems, updating animations, or rendering the UI based on a
    /// fixed frame rate.
    Tick,
    /// Crossterm events.
    ///
    /// These events are emitted by the terminal.
    Crossterm(CrosstermEvent),
    /// Application events.
    ///
    /// Use this event to emit custom events that are specific to your application.
    App(AppEvent),
}

/// Application events.
///
/// You can extend this enum with your own custom events.
#[derive(Debug)]
pub enum AppEvent {
    /// Increment the counter.
    Up,
    /// Decrement the counter.
    Down,
    /// Quit the application.
    Quit,
    /// Property collector events.
    ///
    /// These events are emitted by the property collector waiting for updates.
    PropertyCollector(Vec<ObjectUpdate>),

    StatusMessage(String),
}

/// Terminal event handler.
#[derive(Debug)]
pub struct EventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,

    event_dispatch: Option<tokio::task::JoinHandle<Result<()>>>,
}

impl EventHandler {
    /// Constructs a new instance of [`EventHandler`] and spawns a new thread to handle events.
    pub fn new(monitor: Monitor) -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let internal_sender = sender.clone();
        let event_dispatch = tokio::spawn(async move {
            let mut actor = EventTask::new(internal_sender , monitor);
            actor.run().await
        });
        Self { sender, receiver, event_dispatch: Some(event_dispatch) }
    }

    /// Receives an event from the sender.
    ///
    /// This function blocks until an event is received.
    ///
    /// # Errors
    ///
    /// This function returns an error if the sender channel is disconnected. This can happen if an
    /// error occurs in the event thread. In practice, this should not happen unless there is a
    /// problem with the underlying terminal.
    pub async fn next(&mut self) -> Result<Event> {
        self.receiver
            .recv()
            .await
            .context("Failed to receive event")
    }

    /// Queue an app event to be sent to the event receiver.
    ///
    /// This is useful for sending events to the event handler which will be processed by the next
    /// iteration of the application's event loop.
    pub fn send(&mut self, app_event: AppEvent) {
        // Ignore the result as the reciever cannot be dropped while this struct still has a
        // reference to it
        let _ = self.sender.send(Event::App(app_event));
    }

    pub async fn shutdown(&mut self) -> Result<()> {
        self.receiver.close();
        if let Some(event_dispatch) = self.event_dispatch.take() {
            event_dispatch.await??;
        }
        Ok(())
    }
}

/// A thread that handles reading crossterm events and emitting tick events on a regular schedule.
struct EventTask {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// vCenter event monitor
    monitor: Monitor,
}

impl EventTask {
    /// Constructs a new instance of [`EventThread`].
    fn new(sender: mpsc::UnboundedSender<Event>, monitor: Monitor) -> Self {
        Self { sender, monitor }
    }

    /// Runs the event thread.
    ///
    /// This function emits tick events at a fixed rate and polls for crossterm events in between.
    async fn run(&mut self) -> Result<()> {
        let tick_rate = Duration::from_secs_f64(TICK_FPS);
        let mut reader = crossterm::event::EventStream::new();
        let mut tick = tokio::time::interval(tick_rate);
        loop {
            let tick_delay = tick.tick();
            let crossterm_event = reader.next().fuse();
            let updates = self.monitor.wait_updates(100).fuse();
            tokio::select! {
                _ = self.sender.closed() => {
                    break;
                }
                _ = tick_delay => {
                    self.send(Event::Tick);
                }
                Some(Ok(evt)) = crossterm_event => {
                    self.send(Event::Crossterm(evt));
                }
                updates_result = updates => {
                    match updates_result {
                        Ok(None) => {
                            continue;
                        }
                        Err(e) => {
                            self.send(Event::App(
                                AppEvent::StatusMessage(
                                    format!("Error waiting for updates: {}", e)
                                )
                            ));
                        }
                        Ok(Some(updates)) => {
                            self.send(Event::App(AppEvent::PropertyCollector(updates)));
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Sends an event to the receiver.
    fn send(&self, event: Event) {
        // Ignores the result because shutting down the app drops the receiver, which causes the send
        // operation to fail. This is expected behavior and should not panic.
        let _ = self.sender.send(event);
    }
}
