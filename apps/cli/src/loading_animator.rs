use crossterm::{cursor, execute, style, terminal, ExecutableCommand, Result};
use std::io::stdout;

// TODO: tests

/// See https://stackoverflow.com/a/59890400
pub struct LoadingAnimator<'a> {
    inner: Inner<'a>,
}

struct Inner<'a> {
    chars: &'a [char],
    current: usize,
    // flags for `display`
    aborted: bool,
    hidden_cursor: bool,
}

#[derive(Debug)]
pub struct CharsEmptyError;

impl std::fmt::Display for CharsEmptyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "`chars` cannot be empty")
    }
}

impl std::error::Error for CharsEmptyError {}

impl<'a> LoadingAnimator<'a> {
    /// `chars` must not be empty
    pub fn new(chars: &'a [char]) -> std::result::Result<Self, CharsEmptyError> {
        if chars.is_empty() {
            Err(CharsEmptyError)
        } else {
            Ok(Self {
                inner: Inner {
                    chars,
                    current: 0,
                    aborted: false,
                    hidden_cursor: false,
                },
            })
        }
    }

    fn next(&mut self) -> char {
        let next_char = self.inner.chars[self.inner.current];
        self.inner.current = (self.inner.current + 1) % self.inner.chars.len();

        next_char
    }

    /// Display the next loading character, overwriting the previous one.
    pub fn display(&mut self) -> Result<()> {
        if self.inner.aborted {
            return Ok(());
        }

        let mut stdout = stdout();

        if !self.inner.hidden_cursor {
            stdout.execute(cursor::Hide)?;
            execute!(stdout, cursor::Hide)?;
            self.inner.hidden_cursor = true;
        }

        execute!(
            stdout,
            cursor::SavePosition,
            style::Print(self.next()),
            cursor::RestorePosition
        )
    }

    /// Clear the line.
    pub fn _clear(&self) -> Result<()> {
        execute!(
            stdout(),
            cursor::RestorePosition,
            terminal::Clear(terminal::ClearType::FromCursorDown),
            cursor::Show
        )
    }
}
