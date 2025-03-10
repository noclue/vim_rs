use anyhow::Result;
use ratatui::{DefaultTerminal, Frame};
use ratatui::layout::{Constraint, Layout};
use ratatui::prelude::Widget;
use ratatui::style::{Color, Style};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders};

#[tokio::main]
async fn main() -> Result<()> {
    let terminal = ratatui::init();
    App::default().run(terminal).await
}


#[derive(Default)]
struct App {
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: DefaultTerminal) -> Result<()> {
        let mut terminal = terminal;
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area())
    }
}

impl Widget for &App {
    fn render(self, area: ratatui::layout::Rect, buf: &mut ratatui::buffer::Buffer) {
        let vertical = Layout::vertical([Constraint::Length(1), Constraint::Min(0)]);
        let [title_area, body_area] = vertical.areas(area);

        let title = Line::from("Ratatui async example").centered();
        let block = Block::default()-
            .title(title)
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::LightBlue));

        buf.set_area(title_area, block.render(area));
    }
}