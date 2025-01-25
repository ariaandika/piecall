use std::any::{Any, TypeId};

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers as Mod};

use crate::project::Project;

#[derive(Debug)]
pub struct Piecall {
    focus: TypeId,
    projects: Vec<Project>,
    leader: Leader,
}

impl Piecall {
    pub fn new(projects: Vec<Project>) -> Self {
        Self {
            focus: TypeId::of::<Self>(),
            leader: Leader::None,
            projects,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        let Event::Key(KeyEvent { code, modifiers: md, .. }) = event else {
            return;
        };

        match (code,*md) {
            (KeyCode::Esc,_) => self.leader = Leader::Exit(matches!(self.leader,Leader::Exit(false))),
            (KeyCode::Char('q'),Mod::CONTROL) => self.leader = Leader::Exit(true),
            _ => self.leader = Leader::None,
        }
    }
}

impl Piecall {
    pub fn projects(&self) -> &[Project] {
        &self.projects
    }

    pub fn is_focus<E: Any>(&self) -> bool {
        self.focus == TypeId::of::<E>()
    }

    pub fn set_focus<E: Any>(&mut self) {
        self.focus = TypeId::of::<E>();
    }

    pub fn is_exit(&self) -> bool {
        matches!(self.leader,Leader::Exit(true))
    }
}

#[derive(Debug)]
pub enum Leader {
    None,
    Exit(bool),
}


