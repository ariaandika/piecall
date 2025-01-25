use std::any::{Any, TypeId};

use crate::project::Project;

pub struct Piecall {
    focus: TypeId,
    projects: Vec<Project>,
}

impl Piecall {
    pub fn new(projects: Vec<Project>) -> Self {
        Self { focus: TypeId::of::<Self>(), projects }
    }

    pub fn projects(&self) -> &[Project] {
        &self.projects
    }

    pub fn is_focus<E: Any>(&self) -> bool {
        self.focus == TypeId::of::<E>()
    }

    pub fn set_focus<E: Any>(&mut self) {
        self.focus = TypeId::of::<E>();
    }
}

