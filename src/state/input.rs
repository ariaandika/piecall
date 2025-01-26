use std::ops::Deref;

use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers as Mod};

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
            (KeyCode::Left,..) => self.cursor.prev(),
            (KeyCode::Right,..) => self.cursor.next(),
            (KeyCode::Home,..) => self.cursor.start(),
            (KeyCode::End,..) => self.cursor.end(),

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

        false
    }
}

/// handle cursor position
///
/// len is inclusive
#[derive(Clone, Copy, Debug, Default)]
pub struct CursorState {
    cursor: u16,
    len: u16,
    wrap: bool,
}

impl Deref for CursorState {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl CursorState {
    pub fn new(cursor: u16, len: u16, wrap: bool) -> Self {
        Self { cursor, len, wrap }
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }
}

impl CursorState {
    pub fn next(&mut self) {
        let next = self.saturating_add(1);
        self.cursor = next.min(self.len);
        if self.wrap && next > self.len {
            self.start();
        }
    }

    pub fn prev(&mut self) {
        if let Some(val) = self.checked_sub(1) {
            self.cursor = val;
        } else if self.wrap {
            self.end();
        }
    }

    pub fn start(&mut self) {
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.cursor = self.len;
    }

    pub fn len(&self) -> u16 {
        self.len
    }

    pub fn set_len_str<S: AsRef<str>>(&mut self, val: S) -> bool {
        self.set_len(val.as_ref().len() as u16)
    }

    pub fn set_len(&mut self, len: u16) -> bool {
        self.len = len;
        let clamped = self.cursor > len;
        if clamped {
            self.end();
        }
        clamped
    }
}

