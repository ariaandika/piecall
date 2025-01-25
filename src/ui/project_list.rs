use ratatui::{widgets::{List, ListItem}, Frame};

use crate::Piecall;

pub struct ProjectList;

impl ProjectList {
    pub fn render(&self, frame: &mut Frame, state: &Piecall) {
        let items = state.projects().iter().filter_map(|e| {
            let item = ListItem::new(e.path().to_str()?);
            Some(item)
        });

        let list = List::new(items);

        frame.render_widget(list, frame.area());
    }
}


