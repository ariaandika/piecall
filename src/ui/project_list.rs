use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{style::{Style, Stylize}, widgets::{List, ListItem, ListState}, Frame};

use crate::{project::Project, ui::{BG_SELECT, SELECT_SYMBOL}, Piecall};

pub struct ProjectList {
    list_state: ListState,
}

impl ProjectList {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default().with_selected(Some(0)).with_offset(4),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::Key(KeyEvent {
                code, modifiers: m, ..
            }) => match code {
                KeyCode::Up if m == KeyModifiers::CONTROL => {
                    let select = self.list_state.selected_mut().get_or_insert_default();
                    let s = select.saturating_sub(6);
                    *select = s;
                }

                KeyCode::Down if m == KeyModifiers::CONTROL => {
                    let select = self.list_state.selected_mut().get_or_insert_default();
                    let s = select.saturating_add(6);
                    *select = s;
                }

                KeyCode::Up => self.list_state.select_previous(),
                KeyCode::Down => self.list_state.select_next(),

                KeyCode::Home | KeyCode::PageUp => self.list_state.select_first(),
                KeyCode::End | KeyCode::PageDown => self.list_state.select_last(),
                _ => {}
            },
            _ => {}
        }
    }

    pub fn render(&mut self, frame: &mut Frame, state: &Piecall) {
        fn to_item(e: &Project) -> ListItem<'_> {
            ListItem::new(e.path().to_str().unwrap_or("<NON-UTF8>"))
        }

        let items = state.projects().iter().map(to_item);

        let list = List::new(items)
            .highlight_symbol(SELECT_SYMBOL)
            .highlight_style(Style::default().red().bg(BG_SELECT));

        frame.render_stateful_widget(&list, frame.area(), &mut self.list_state);
    }
}

