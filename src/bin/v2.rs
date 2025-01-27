use crossterm::event::{self, Event, KeyCode, KeyEvent};
use piecall::{project_list::ProjectList, prompt::Prompt, shared::project};
use ratatui::layout::{Constraint, Layout, Rect};

fn main() {
    let mut term = ratatui::init();

    let (projects,_error) = project::io::list(std::env::args().skip(1));
    let mut project_list = ProjectList::new();
    let mut prompt = Prompt::new();

    let index = loop {
        term.draw(|fr| {
            let layout = Layout::vertical([
                Constraint::Length(3),
                Constraint::Fill(1),
            ])
            .split(fr.area());

            let Rect { x, y, width, .. } = layout[0];
            prompt.render_cursor(fr, (x + 1, y + 1));
            prompt.render(fr, Rect::new(x, y, width, 3));

            project_list.render(fr, layout[1], &projects);
        })
        .unwrap();

        let event = event::read().unwrap();

        if let Event::Key(KeyEvent { code: KeyCode::Esc, .. }) = &event {
            break None;
        }

        if let Event::Key(KeyEvent { code: KeyCode::Enter, .. }) = &event {
            break Some(project_list.index());
        }

        project_list.handle_event(&event, &projects);
        prompt.handle_event(&event);
    };

    ratatui::restore();

    if let Some(index) = index {
        if let Some(path) = projects.get(index as usize).and_then(|e|e.path().to_str()) {
            println!("{path}");
        }
    }
}
