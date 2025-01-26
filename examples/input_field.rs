use crossterm::event::{self, Event, KeyCode, KeyEvent};
use piecall::state::input::InputState;
use ratatui::{text::Span, TerminalOptions};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut term = ratatui::init_with_options(TerminalOptions {
        viewport: ratatui::Viewport::Inline(2),
    });

    // NOTE: save state between loop
    let mut state = InputState::default();

    let output = loop {
        term.draw(|fr| {
            let area = fr.area();

            // NOTE: state does not handle cursor rendering
            fr.set_cursor_position((area.x + *state.cursor(), area.y));

            // NOTE: using ratatui built-in `Span`
            fr.render_widget(Span::raw(state.buffer()), area);
        })?;

        let event = event::read()?;

        if let Event::Key(KeyEvent { code: KeyCode::Esc, .. }) = &event {
            break String::new();
        }

        // NOTE: handle event if input field is focused for example
        let submit = state.handle_event(&event);

        if submit {
            break state.into_buffer();
        }
    };

    ratatui::restore();

    if !output.is_empty() {
        println!();
        println!("Output: {output}");
    }

    Ok(())
}


