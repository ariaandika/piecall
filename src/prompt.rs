use crossterm::event::{Event, KeyCode as Key, KeyEvent, KeyEventKind, KeyModifiers as Mod};
use ratatui::{layout::{Position, Rect}, text::Span, widgets::{Block, BorderType, Borders}, Frame};

#[derive(Debug, Default)]
pub struct Prompt {
    buffer: String,
}

impl Prompt {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn render_cursor(&self, fr: &mut Frame, offset: impl Into<Position>) {
        let Position { x, y } = offset.into();
        fr.set_cursor_position((x + self.buffer.len() as u16,y));
    }

    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    pub fn handle_event(&mut self, event: &Event) {
        let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event else {
            return;
        };

        match (code,*modifiers) {
            (Key::Char('u'),Mod::CONTROL) => {
                self.buffer.clear();
            }
            (Key::Char(ch),_) => {
                self.buffer.push(*ch);
            }
            (Key::Backspace,_) => {
                self.buffer.pop();
            }
            _ => {},
        }
    }

    pub fn render(&self, fr: &mut Frame, area: Rect) {
        let block = Block::new()
            .borders(Borders::all())
            .border_type(BorderType::Thick)
            .title("Search");
        let inner = block.inner(area);

        let content = Span::raw(&self.buffer);

        fr.render_widget(block, area);
        fr.render_widget(content, inner);
    }
}


