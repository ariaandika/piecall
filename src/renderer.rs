use crossterm::event::{self, Event, KeyCode, KeyEvent};
use ratatui::Frame;

use crate::{
    error::{Result, StdError},
    project,
    ui::project_list::ProjectList,
    Piecall, Terminal,
};

pub struct Renderer {
    state: Piecall,
    error: Option<Box<StdError>>,
    exit: bool,

    body: ProjectList,
}

impl Renderer {
    pub fn setup() -> Self {
        let (projects, error) = project::io::list_project(std::env::args().skip(1));
        Self {
            error,
            state: Piecall::new(projects),
            body: ProjectList::new(),
            exit: false,
        }
    }

    pub fn run(mut self, mut term: Terminal) -> Result<()> {
        self.state.set_focus::<ProjectList>();
        term.draw(|fr|self.render(fr)).ok();

        loop {
            if self.exit || self.error.is_some() {
                break;
            }

            self.handle_event(event::read()?);
            term.draw(|fr|self.render(fr)).ok();
        };

        match self.error {
            None => Ok(()),
            Some(err) => Err(crate::error::Error::Custom(err)),
        }
    }

    fn handle_event(&mut self, event: Event) {
        if matches!(
            event,
            Event::Key(KeyEvent {
                code: KeyCode::Esc,
                ..
            })
        ) {
            self.exit = true;
            return;
        }

        self.body.handle_event(event);
    }

    fn render(&mut self, frame: &mut Frame) {
        self.body.render(frame, &self.state);
    }
}

