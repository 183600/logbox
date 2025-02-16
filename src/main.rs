use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

mod windows;
mod linux;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化终端
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, event::EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 初始化应用状态
    let mut app = App::new();

    loop {
        terminal.draw(|f| ui(f, &app))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }

    // 恢复终端
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn ui<B: ratatui::backend::Backend>(f: &mut ratatui::Frame<B>, app: &App) {
    let chunks = Layout::default()
        .constraints([Constraint::Length(3), Constraint::Min(3)].as_ref())
        .direction(Direction::Vertical)
        .split(f.size());

    let block = Block::default()
        .title("Window Analysis")
        .borders(Borders::ALL);
    f.render_widget(block, chunks[0]);

    let text = Paragraph::new("Press 'q' to quit")
        .style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
        .alignment(ratatui::layout::Alignment::Center);
    f.render_widget(text, chunks[1]);
}

struct App {
    // 添加应用状态
}

impl App {
    fn new() -> App {
        App {
            // 初始化状态
        }
    }
}
