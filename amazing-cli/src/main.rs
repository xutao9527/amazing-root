use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyEventKind};
use ratatui::Frame;
use ratatui::layout::Layout;
use ratatui::widgets::Block;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = run_app(&mut terminal);
    ratatui::restore();
    result
}

fn run_app(terminal: &mut ratatui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| render(frame))?;
        if handle_events()? {
            break Ok(());
        }
    }
}

fn render(frame: &mut Frame) {
    use ratatui::layout::Constraint::{Fill, Length, Min};
    let vertical = Layout::vertical([Length(1), Min(0), Length(1)]);
    let [title_area, main_area, status_area] = vertical.areas(frame.area());
    let horizontal = Layout::horizontal([Fill(1); 2]);
    let [left_area, right_area] = horizontal.areas(main_area);
    frame.render_widget(Block::bordered().title("Title Bar"), title_area);
    frame.render_widget(Block::bordered().title("Status Bar"), status_area);
    frame.render_widget(Block::bordered().title("Left"), left_area);
    frame.render_widget(Block::bordered().title("Right"), right_area);
}

fn handle_events() -> std::io::Result<bool> {
    match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
            KeyCode::Char('q') => return Ok(true),
            _ => {}
        },
        _ => {}
    }
    Ok(false)
}
