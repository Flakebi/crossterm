//! This is an `ANSI escape code` specific implementation for terminal related action.
//! This module is used for windows 10 terminals and unix terminals by default.

use super::*;

use std::io::Write;

/// This struct is an ansi escape code implementation for terminal related actions.
pub struct AnsiTerminal;
use cursor::TerminalCursor;


impl AnsiTerminal {
    pub fn new() -> AnsiTerminal {
        AnsiTerminal {}
    }
}

impl ITerminal for AnsiTerminal {
    fn clear(&self, clear_type: ClearType, screen: &mut Screen) {
        match clear_type {
            ClearType::All => {
                write!(screen, csi!("2J"));
                TerminalCursor::new(screen).goto(0,0);
            }
            ClearType::FromCursorDown => {
                write!(screen, csi!("J"));
            }
            ClearType::FromCursorUp => {
                write!(screen, csi!("1J"));
            }
            ClearType::CurrentLine => {
                write!(screen, csi!("2K"));
            }
            ClearType::UntilNewLine => {
                write!(screen, csi!("K"));
            }
        };
    }

    fn terminal_size(&self, _stdout: &Arc<TerminalOutput>) -> (u16, u16) {
        functions::get_terminal_size()
    }

    fn scroll_up(&self, count: i16, screen: &mut Screen) {
        write!(screen, csi!("{}S"), count);
    }

    fn scroll_down(&self, count: i16, screen: &mut Screen) {
        write!(screen, csi!("{}T"), count);
    }

    fn set_size(&self, width: i16, height: i16, screen: &mut Screen) {
        write!(screen, csi!("8;{};{}t"), height, width);
    }

    fn exit(&self,stdout: &Arc<TerminalOutput>) {
        // drop the screen with the current stdout. This will make sure when in raw mode this will be disabled first.
        let screen = Screen::from(stdout.clone());
        drop(screen);
        functions::exit_terminal();
    }
}
