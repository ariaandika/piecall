use crate::project::Project;

pub struct Piecall {
    projects: Vec<Project>,
}

impl Piecall {
    pub fn new(projects: Vec<Project>) -> Self {
        Self { projects }
    }

    pub fn projects(&self) -> &[Project] {
        &self.projects
    }
}

