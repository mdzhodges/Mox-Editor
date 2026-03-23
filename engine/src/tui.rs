use std::io::stdout;
use crossterm;
use ratatui;
use ratatui::prelude::CrosstermBackend;

pub fn read_file(path: &str) -> String{
    let path_string: String = std::fs::read_to_string(path).unwrap_or_default();
    
    path_string
}



fn init_terminal() -> std::io::Result<ratatui::Terminal<CrosstermBackend<std::io::Stdout>>> {
    crossterm::terminal::enable_raw_mode()?;
    crossterm::execute!(stdout(), crossterm::terminal::EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    ratatui::Terminal::new(backend) 
}

fn cleanup_terminal() {
    crossterm::execute!(stdout(), crossterm::terminal::LeaveAlternateScreen).ok();
    crossterm::terminal::disable_raw_mode().ok();
}

pub fn entry_point(path: &str) {
    let mut terminal = init_terminal().expect("Failed");
    let file_contents = read_file(path);
    
    loop {
        terminal.draw(|frame| {
            let paragraph = ratatui::widgets::Paragraph::new(file_contents.as_str());
            frame.render_widget(paragraph, frame.area());
        }).expect("draw failed");
    
        if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
            if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    _ => {}
                }
            }
        }
    }
    cleanup_terminal();
}


