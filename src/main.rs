use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use std::io::{self, stdout};

mod app;
mod parse;
mod ui;
use app::App;

fn main() -> io::Result<()> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    let mut should_quit = false;

    let port_name: &str = "COM26";
    let baud_rate: u32 = 115200;

    let mut app = App::new(port_name, baud_rate);
    

    while !should_quit {
        terminal.draw(|f| ui(f, &mut app))?;
        should_quit = app.handle_events()?;
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn ui(frame: &mut Frame, _app: &mut App) {
    let vertical = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(3),
        Constraint::Length(3),
    ]);
    let [title_area, servocntrl_area, alt_area] = vertical.areas(frame.size());
    frame.render_widget(Paragraph::new("Meister2024"), title_area);

    if let Some(data) = _app.servoctrl_data.as_ref() {
        frame.render_widget(Paragraph::new(format!("status:{:?}    rudder:{:>3.2}    elevator:{:>3.2}    trim:{:>3.2}    timestamp:{:>10}",data.status,data.rudder,data.elevator,data.trim,data.timestamp)).block(Block::bordered().title("ServoController").title_alignment(Alignment::Center)), servocntrl_area);
    } else {
        frame.render_widget(Paragraph::new("No data received").block(Block::default().title("ServoController").title_alignment(Alignment::Center)), servocntrl_area);
    }

    if let Some(data) = _app.alt_data.as_ref() {
        frame.render_widget(Paragraph::new(format!("altitude:{:>3.2}    timestamp:{:>10}",data.altitude,data.timestamp)).block(Block::bordered().title("Altimeter").title_alignment(Alignment::Center)), alt_area);
    } else {
        frame.render_widget(Paragraph::new("No data received").block(Block::default().title("Altimeter").title_alignment(Alignment::Center)), alt_area);
    }
}
