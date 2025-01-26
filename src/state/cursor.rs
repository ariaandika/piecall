use std::ops::Deref;

use crossterm::event::{Event, KeyCode, KeyEvent};

/// handle cursor position
///
/// len is inclusive
#[derive(Clone, Copy, Debug, Default)]
pub struct CursorState {
    cursor: u16,
    len: u16,
    wrap: bool,
    // is direction vertical
    ver: bool,
}

impl Deref for CursorState {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        &self.cursor
    }
}

impl CursorState {
    pub fn new(cursor: u16, len: u16, wrap: bool, ver: bool) -> Self {
        Self { cursor, len, wrap, ver }
    }

    pub fn with_wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    pub fn with_len(mut self, len: u16) -> Self {
        self.len = len;
        self
    }

    pub fn vertical(mut self) -> Self {
        self.ver = true;
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.ver = false;
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

impl CursorState {
    pub fn handle_event(&mut self, event: &Event) {
        let Event::Key(KeyEvent { code, .. }) = event else {
            return;
        };

        match (code,self.ver) {
            (KeyCode::Left,false) => self.prev(),
            (KeyCode::Right,false) => self.next(),
            (KeyCode::Up,true) => self.prev(),
            (KeyCode::Down,true) => self.next(),
            (KeyCode::Home,_) => self.start(),
            (KeyCode::End,_) => self.end(),
            _ => {}
        }
    }
}
