use crossterm::{cursor, execute, style, terminal, ExecutableCommand, Result};
use std::io::stdout;

// TODO: tests

/// See https://stackoverflow.com/a/59890400
///
/// `chars` cannot be empty
#[derive(Debug)]
pub struct LoadingAnimator<'a> {
    chars: &'a [char],
    current: usize,
    // flags for `display`
    aborted: bool,
    hidden_cursor: bool,
}

impl<'a> LoadingAnimator<'a> {
    /// `chars` cannot be empty
    pub fn new(chars: &'a [char]) -> Self {
        Self {
            chars,
            current: 0,
            aborted: false,
            hidden_cursor: false,
        }
    }

    fn next(&mut self) -> char {
        let next_char = self.chars[self.current];
        self.current = (self.current + 1) % self.chars.len();

        next_char
    }

    /// Display the next loading character, overwriting the previous one.
    pub fn display(&mut self) -> Result<()> {
        if self.aborted {
            return Ok(());
        }

        let mut stdout = stdout();

        if !self.hidden_cursor {
            stdout.execute(cursor::Hide)?;
            execute!(stdout, cursor::Hide)?;
            self.hidden_cursor = true;
        }

        execute!(
            stdout,
            cursor::SavePosition,
            style::Print(self.next()),
            cursor::RestorePosition
        )
    }

    /// Stop the animation and clear the line.
    pub fn stop_and_clear(&mut self) -> Result<()> {
        self.aborted = true;

        execute!(
            stdout(),
            cursor::RestorePosition,
            terminal::Clear(terminal::ClearType::FromCursorDown),
            cursor::Show
        )
    }
}
