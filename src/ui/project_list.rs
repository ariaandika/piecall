use crossterm::event::{Event, KeyCode, KeyEvent};
use ratatui::{style::Stylize, widgets::{List, ListItem}, Frame};

use crate::Piecall;

use super::BG_SELECT;

pub struct ProjectList {
    select: usize,
}

impl ProjectList {
    pub fn new() -> Self {
        Self { select: 0 }
    }

    pub fn handle_event(&mut self, state: &Piecall, event: Event) {
        match event {
            Event::Key(
                key @ KeyEvent {
                    code: KeyCode::Up | KeyCode::Down,
                    ..
                },
            ) => {
                let projects = state.projects();
                let up = matches!(key.code, KeyCode::Up);
                let current = self.select.wrapping_add_signed(if up { -1 } else { 1 });
                let select = if current >= projects.len() {
                    if up { projects.len().saturating_sub(1) } else { 0 }
                } else {
                    current
                };

                self.select = select;
            }
            _ => {}
        }
    }

    pub fn render(&self, frame: &mut Frame, state: &Piecall) {
        let select = self.select;

        let items = state.projects().iter().enumerate().filter_map(|(i, e)| {
            let item = ListItem::new(e.path().to_str()?);
            let item = if i == select {
                item.bg(BG_SELECT)
            } else {
                item
            };
            Some(item)
        });

        let list = List::new(items);

        frame.render_widget(list, frame.area());
    }
}

