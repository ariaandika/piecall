use crossterm::event::{self, Event};
use ratatui::Frame;

use crate::{
    error::{BoxError, Result},
    project,
    ui::project_list::ProjectList,
    Piecall, Terminal,
};

pub struct Renderer {
    // STATE
    state: Piecall,
    error: Option<BoxError>,

    // UI
    body: ProjectList,
}

impl Renderer {
    pub fn setup() -> Self {
        let (projects, error) = project::io::list_project(std::env::args().skip(1));
        Self {
            state: Piecall::new(projects),
            body: ProjectList::new(),
            error,
        }
    }

    pub fn run(mut self, mut term: Terminal) -> Result<()> {
        self.state.set_focus::<ProjectList>();
        term.draw(|fr|self.render(fr))?;

        loop {
            if self.state.is_exit() || self.error.is_some() {
                break;
            }

            self.handle_event(event::read()?);
            term.draw(|fr|self.render(fr))?;
        };

        if let Some(err) = self.error.take() {
            Err(err)?;
        }

        Ok(())
    }

    fn handle_event(&mut self, event: Event) {
        self.state.handle_event(&event);
        self.body.handle_event(event);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.body.render(frame, &self.state);
    }
}

