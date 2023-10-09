use tui::{
    backend::Backend,
    layout::{Constraint, Alignment, Direction, Layout},
    widgets::{Paragraph, Block, Borders, Wrap},
    style::{Color, Style, Modifier},
    text::{Span,Spans},
    Frame,
};
fn ui<B: Backend>(f: &mut Frame<B>) {
    let chunks = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
    [
        Constraint::Percentage(10),
                 Constraint::Percentage(60),
                 Constraint::Percentage(30)
    ].as_ref()
    )
    .split(f.size());
    let block = Block::default()
    .title("Block 1")
    .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);
    let block = Block::default()
    .title("Block 2")
    .borders(Borders::ALL);
    f.render_widget(block, chunks[1]);

    let text = vec![
        Spans::from(vec![
            Span::raw("This is a "),
            Span::styled("test", Style::default().add_modifier(Modifier::ITALIC)),
            Span::raw("."),
        ]),
        Spans::from(Span::styled("And only a test", Style::default().fg(Color::Red))),
    ];
    let final_block = Paragraph::new(text)
        .block(Block::default().title("Paragraph").borders(Borders::ALL))
        .alignment(Alignment::Center)
        .wrap(Wrap {trim: true});
    f.render_widget(final_block, chunks[2]);

}

use std::{io, thread, time::Duration};
use tui::{
    backend::CrosstermBackend,
  terminal::Terminal,
};
use crossterm:: {
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.draw(|f| {
        ui(f);
    })?;

    thread::sleep(Duration::from_millis(5000));

    //restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}
