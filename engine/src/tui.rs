use std::io::stdout;
use crossterm;
use ratatui;
use ratatui::prelude::{CrosstermBackend, Layout, Direction, Constraint};
use ratatui::style::{Style, Modifier};
use ratatui::widgets::Paragraph;
use crate::buffer;

pub fn read_file(path: &str) -> String {
    std::fs::read_to_string(path).unwrap_or_default()
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

    let file_bytes = std::fs::read(path).unwrap_or_default();
    let mut buf = buffer::create_buffer();
    buffer::load_content(&mut buf, &file_bytes);
    let filename = std::path::Path::new(path)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("[no name]")
        .to_string();

    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(1), Constraint::Length(1)])
                .split(frame.area());
                
            let content = buffer::get_content(&buf);
            let text = String::from_utf8_lossy(&content).into_owned();
            let paragraph = Paragraph::new(text);
            frame.render_widget(paragraph, chunks[0]);

            // Status bar
            let (row, col) = buffer::cursor_position(&buf);
            let status = format!("  {}  |  {}:{}  ", filename, row + 1, col + 1);
            let status_bar = Paragraph::new(status)
                .style(Style::default().add_modifier(Modifier::REVERSED));
            frame.render_widget(status_bar, chunks[1]);

            // Terminal cursor
            frame.set_cursor_position((col as u16, row as u16));
        }).expect("draw failed");

        if crossterm::event::poll(std::time::Duration::from_millis(100)).unwrap_or(false) {
            if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                match key.code {
                    crossterm::event::KeyCode::Char('q') => break,
                    crossterm::event::KeyCode::Up    | crossterm::event::KeyCode::Char('k') => {
                        let (row, col) = buffer::cursor_position(&buf);
                        if row > 0 {
                            let content = buffer::get_content(&buf);
                            let mut line_start = [0usize; 2];
                            let mut cur_line = 0;
                            let mut i = 0;
                            while i < content.len() && cur_line < row {
                                if content[i] == b'\n' {
                                    cur_line += 1;
                                    if cur_line == row - 1 { line_start[0] = i + 1; }
                                    if cur_line == row     { line_start[1] = i + 1; break; }
                                }
                                i += 1;
                            }
                            let prev_line_len = line_start[1].saturating_sub(line_start[0]).saturating_sub(1);
                            let target = line_start[0] + col.min(prev_line_len);
                            buffer::move_cursor_to(&mut buf, target);
                        }
                    }
                    crossterm::event::KeyCode::Down  | crossterm::event::KeyCode::Char('j') => {
                        let (row, col) = buffer::cursor_position(&buf);
                        let content = buffer::get_content(&buf);
                        let mut cur_line = 0;
                        let mut next_line_start = None;
                        let mut next_next_line_start = content.len();
                        let mut i = 0;
                        while i < content.len() {
                            if content[i] == b'\n' {
                                cur_line += 1;
                                if cur_line == row + 1 { next_line_start = Some(i + 1); }
                                if cur_line == row + 2 { next_next_line_start = i; break; }
                            }
                            i += 1;
                        }
                        if let Some(start) = next_line_start {
                            let next_line_len = next_next_line_start.saturating_sub(start);
                            let target = start + col.min(next_line_len);
                            buffer::move_cursor_to(&mut buf, target);
                        }
                    }
                    crossterm::event::KeyCode::Left  | crossterm::event::KeyCode::Char('h') => {
                        buffer::move_cursor_left(&mut buf);
                    }
                    crossterm::event::KeyCode::Right | crossterm::event::KeyCode::Char('l') => {
                        buffer::move_cursor_right(&mut buf);
                    }
                    _ => {}
                }
            }
        }
    }

    cleanup_terminal();
}
