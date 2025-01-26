use crossterm::event::{self, Event, KeyCode, KeyEvent};
use piecall::state::input::InputState;
use ratatui::{text::Span, TerminalOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut term = ratatui::init_with_options(TerminalOptions {
        viewport: ratatui::Viewport::Inline(2),
    });

    // NOTE: save state between loop
    let mut state = InputState::default();

    loop {
        term.draw(|fr| {
            let area = fr.area();

            // NOTE: state does not handle cursor rendering
            fr.set_cursor_position((area.x + *state.cursor(), area.y));

            // NOTE: using ratatui built-in `Span`
            fr.render_widget(Span::raw(state.buffer()), area);
        })?;

        let event = event::read()?;

        if let Event::Key(KeyEvent { code: KeyCode::Esc, .. }) = &event {
            break;
        }

        // NOTE: handle event if input field is focused for example
        state.handle_event(&event);
    }

    ratatui::restore();

    Ok(())
}


