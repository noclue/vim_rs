use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;
use crate::event::EventHandler;
use crate::prop_browser::PropertyBrowserManager;
use crate::resource_browser::ResourceManager;

pub(crate) enum BodyPane {
    ResourceBrowser(ResourceManager),
    PropertyBrowser(PropertyBrowserManager),
}

impl BodyPane {
    pub fn render(&mut self, frame: &mut Frame, body_area: Rect) {
        match self {
            BodyPane::ResourceBrowser(resource_manager) => {
                resource_manager.render(frame, body_area);
            }
            BodyPane::PropertyBrowser(property_browser) => {
                property_browser.render(frame, body_area);
            }
        }
    }

    pub async fn handle_key(&mut self, key: &KeyEvent, events: &mut EventHandler) -> anyhow::Result<bool> {
        match self {
            BodyPane::ResourceBrowser(resource_manager) => {
                resource_manager.handle_key(key, events).await
            }
            BodyPane::PropertyBrowser(property_browser) => {
                property_browser.handle_key(key, events).await
            }
        }
    }
    pub fn get_hints(&self) -> (&'static [&'static str], &'static [&'static str]) {
        match self {
            BodyPane::ResourceBrowser(resource_manager) => resource_manager.get_hints(),
            BodyPane::PropertyBrowser(property_browser) => property_browser.get_hints(),
        }
    }
}