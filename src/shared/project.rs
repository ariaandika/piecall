use std::path::PathBuf;

#[derive(Debug)]
pub struct Project {
    path: PathBuf,
    name: String,
    readme: Option<String>,
    kind: ProjectKind,
}

impl Project {
    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn readme(&self) -> Option<&String> {
        self.readme.as_ref()
    }

    pub fn kind(&self) -> &ProjectKind {
        &self.kind
    }
}

#[derive(Debug)]
pub enum ProjectKind {
    Rust,
    Js,
    Other
}

pub mod io {
    use std::{fs, path::Path};
    use crate::error::BoxError;

    use super::*;

    pub fn list(
        paths: impl IntoIterator<Item = impl AsRef<Path>>,
    ) -> (Vec<Project>, Option<BoxError>) {
        let mut projects = vec![];
        let mut error = None;

        for path in paths {
            let projects_dir = match fs::read_dir(&path) {
                Ok(ok) => ok,
                Err(err) => {
                    error.get_or_insert_with(||err.into());
                    continue;
                }
            };

            for project in projects_dir {
                let project = match project {
                    Ok(ok) => ok,
                    Err(err) => {
                        error.get_or_insert_with(||err.into());
                        continue;
                    }
                };

                projects.push(Project {
                    path: project.path(),
                    name: "".into(),
                    readme: None,
                    kind: ProjectKind::Other,
                });
            }
        }

        (projects, error)
    }
}

