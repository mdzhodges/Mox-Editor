pub mod buffer;
pub mod tui;

pub fn main() -> std::io::Result<()> {
    let path = std::env::args().nth(1).unwrap_or_default();
    tui::entry_point(&path);
    Ok(())
}