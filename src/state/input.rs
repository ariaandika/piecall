use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers as Mod};

use super::cursor::CursorState;

/// handle input field buffer and cursor
#[derive(Debug, Default)]
pub struct InputState {
    buffer: String,
    cursor: CursorState,
}

/// Creation
impl InputState {
    pub fn new(buffer: String) -> Self {
        Self { buffer, ..Default::default() }
    }

    pub fn buffer(&self) -> &str {
        &self.buffer
    }

    pub fn into_buffer(self) -> String {
        self.buffer
    }

    pub fn cursor(&self) -> CursorState {
        self.cursor
    }
}

impl InputState {
    /// return true if submited
    pub fn handle_event(&mut self, event: &Event) -> bool {
        let Event::Key(KeyEvent { code, modifiers, .. }) = event else {
            return false;
        };

        match (code,*modifiers) {
            (KeyCode::Char('u'),Mod::CONTROL) => {
                self.buffer.clear();
                self.cursor.set_len(0);
            }
            (KeyCode::Char(ch),..) => {
                self.buffer.insert(*self.cursor as usize, *ch);
                self.cursor.set_len_str(&self.buffer);
                self.cursor.next();
            },

            (KeyCode::Enter,..) => return true,

            (KeyCode::Backspace,..) => {
                let Some(idx) = self.cursor.checked_sub(1) else {
                    return false;
                };
                let Some((idx,_)) = self.buffer.char_indices().nth(idx as usize) else {
                    return false;
                };

                self.buffer.remove(idx);
                let clamped = self.cursor.set_len_str(&self.buffer);
                if !clamped {
                    self.cursor.prev();
                }
            }

            (KeyCode::Delete,..) => {
                let Some((idx,_)) = self.buffer.char_indices().nth(*self.cursor as usize) else {
                    return false;
                };
                self.buffer.remove(idx);
                self.cursor.set_len_str(&self.buffer);
            }
            _ => {}
        }

        self.cursor.handle_event(event);

        false
    }
}


