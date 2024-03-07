use crossterm::{
    cursor::{self, SetCursorStyle},
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{self, enable_raw_mode, Clear, ClearType},
    ExecutableCommand,
};
use std::{
    io::{self, Stdout},
    time::Duration,
};

use crate::chat::Chat;

pub struct Terminal {
    pub size: (u16, u16),
    pub poll_interval: Duration,
    pub stdout: Stdout,
    pub entering_addr: bool,
    pub input_buf: String
}

impl Terminal {
    pub fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        stdout.execute(cursor::Hide)?;
        Ok(Self {
            size: (1, 1),
            poll_interval: Duration::from_millis(100),
            stdout,
            entering_addr: false,
            input_buf: String::with_capacity(100),
        })
    }
    pub fn update_size(&mut self) -> io::Result<()> {
        self.size = terminal::size()?;
        Ok(())
    }
    /// Handle events such as keystrokes, mouse clicks, etc.
    #[inline]
    pub fn handle_event(&mut self, event: Event, chat: &mut Chat) -> io::Result<()> {
        match event {
            Event::Resize(nw, nh) => {
                self.size = (nw, nh);
                self.stdout.execute(Clear(terminal::ClearType::All))?;
            }
            Event::Key(key) => match key.code {
                KeyCode::Char('c') => {
                    // Control-C (SIGINT) hit
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        let _ = self.stdout.execute(Clear(ClearType::All));
                        return Err(io::Error::new(
                            io::ErrorKind::Interrupted,
                            "signal SIGINT (interrupt) hit (caused by CTRL-C)",
                        ));
                    }
                    if self.entering_addr {
                        self.input_buf.push('c');
                    } else {
                        self.entering_addr = true;
                        self.stdout.execute(cursor::Show)?;
                    }
                }
                KeyCode::Char('l') if !self.entering_addr => {
                    chat.listen()?;
                }
                KeyCode::Backspace if self.entering_addr => {
                    let _ = self.input_buf.pop();
                }
                KeyCode::Char(c) if self.entering_addr => {
                    self.input_buf.push(c);
                }
                KeyCode::Enter | KeyCode::Esc if self.entering_addr => {
                    self.entering_addr = false;
                    self.stdout.execute(cursor::Hide)?;
                    self.input_buf.clear();
                }
                _ => {}
            },
            _ => {}
        }
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = self.stdout.execute(cursor::Show);
    }
}
