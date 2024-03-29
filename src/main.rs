use chat::{Chat, Connection};
use std::io;
use std::thread;
use std::time::Duration;

mod chat;
mod term;

use crossterm::{
    cursor::MoveTo,
    event, execute,
    style::{Print, PrintStyledContent, Stylize},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use term::Terminal;

fn main_() -> io::Result<()> {
    let mut chat = Chat::new(([0, 0, 0, 0], 8080));
    let mut term = Terminal::new()?;

    term.update_size()?;
    term.stdout.execute(Clear(ClearType::All))?;

    loop {
        let status_len;
        let status = if let Some(ref conn) = chat.conn {
            match conn {
                Connection::Client(stream) => if let Ok(peer) = stream.peer_addr() {
                    let s = format!(" ✔ connected: {peer} ");
                    status_len = s.len();
                    s
                } else {
                    let s = " ✔ connected ";
                    status_len = s.len();
                    s.to_string()
                }
                .white()
                .on_green()
                .bold(),
                Connection::Listener(listener) => {
                    let s = match listener.local_addr() {
                        Ok(addr) => {
                            let s = format!(" ◎ listening: {addr} ");
                            status_len = s.len();
                            s.on_yellow().white()
                        },
                        Err(e) => {
                            let s = format!(" ◎ listening: {e} ");
                            status_len = s.len();
                            s.on_yellow().red()
                        }
                    };
                    s.bold()
                }
            }
        } else {
            let s = " ✕ not connected ";
            status_len = s.len();
            s.to_string().white().on_dark_red()
        };

        let help = if chat.conn.is_none() {
            "[C: Connect | L: listen]"
        } else {
            "[Shift-Q: Disconnect]"
        };

        execute! {
            term.stdout,
            MoveTo(0, term.size.1),
            PrintStyledContent(" ".repeat(term.size.0 as usize - 1).on_white()),
            MoveTo(0, term.size.1),
            PrintStyledContent(status),
        }?;

        if term.entering_addr {
            execute! {
                term.stdout,
                MoveTo(status_len as u16, term.size.1),
                PrintStyledContent("Address: ".on_white().black().bold()),
                PrintStyledContent(term.input_buf.as_str().on_white().black())
            }?;
        } else {
            execute! {
                term.stdout,
                MoveTo(term.size.0.checked_sub(help.len() as u16).unwrap_or(0), term.size.1),
                PrintStyledContent(help.on_white().black())
            }?;
        }

        if event::poll(term.poll_interval)? {
            term.handle_event(event::read()?, &mut chat)?;
        }

        thread::sleep(Duration::from_millis(50));
    }
}

fn main() {
    if let Err(e) = main_() {
        eprintln!("ERROR: {e}");
    }
}
