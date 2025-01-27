use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Rect, text::Span, widgets::{Block, BorderType, Borders}, Frame};

use super::shared::project::Project;

#[derive(Debug)]
pub struct ProjectList {
    index: u16,
}

impl ProjectList {
    pub fn new() -> Self {
        Self { index: 0 }
    }

    /// return true if submitting
    pub fn handle_event(&mut self, event: &Event, projects: &[Project]) {
        let Event::Key(KeyEvent {
            code,
            modifiers,
            kind: KeyEventKind::Press,
            ..
        }) = event else {
            return;
        };

        match (code, *modifiers) {
            (KeyCode::Up, _) => {
                let prev = self.index.checked_sub(1);
                match prev {
                    Some(prev) => self.index = prev,
                    None => self.index = projects.len().saturating_sub(1) as u16,
                }
            }
            (KeyCode::Down, _) => {
                let next = self.index.saturating_add(1);
                match next >= projects.len() as u16 {
                    true => self.index = 0,
                    false => self.index = next,
                }
            }
            _ => {}
        }
    }

    pub fn render<'a>(&self, fr: &mut Frame, area: Rect, projects: impl IntoIterator<Item = &'a Project>) {
        let block = Block::new()
            .borders(Borders::all())
            .border_type(BorderType::Thick);
        let Rect { x, y, width, height } = block.inner(area);
        let select = self.index;

        fr.render_widget(block, area);

        projects
            .into_iter()
            .take(height as usize)
            .enumerate()
            .for_each(|(i, project)| {
                let i = i as u16;
                if i == select {
                    fr.render_widget(Span::raw(">"), Rect::new(x, y + i, 1, 1));
                }
                fr.render_widget(
                    Span::raw(project.path().to_str().unwrap()),
                    Rect::new(x+1, y + i, width.saturating_sub(1), height),
                );
            });
    }

    pub fn index(&self) -> u16 {
        self.index
    }
}

