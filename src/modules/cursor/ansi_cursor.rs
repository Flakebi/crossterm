//! This is an ANSI specific implementation for cursor related action.
//! This module is used for windows 10 terminals and unix terminals by default.
//! Note that the cursor position is 0 based. This means that we start counting at 0 when setting the cursor position ect.

use super::*;
use std::io::Write;


/// This struct is an ansi implementation for cursor related actions.
pub struct AnsiCursor;

impl AnsiCursor {
    pub fn new() -> Box<AnsiCursor> {
        Box::from(AnsiCursor {})
    }
}

impl ITerminalCursor for AnsiCursor {
    fn goto(&self, x: u16, y: u16, screen: &mut Screen) {
        write!(screen, csi!("{};{}H"), y + 1, x + 1);
    }

    fn pos(&self, stdout: &Arc<TerminalOutput>) -> (u16, u16) {
        functions::get_cursor_position(stdout)
    }

    fn move_up(&self, count: u16, screen: &mut Screen) {
        write!(screen, csi!("{}A"), count);
    }

    fn move_right(&self, count: u16, screen: &mut Screen) {
        write!(screen, csi!("{}C"), count);
    }

    fn move_down(&self, count: u16, screen: &mut Screen) {
        write!(screen, csi!("{}B"), count);
    }

    fn move_left(&self, count: u16, screen: &mut Screen) {
        write!(screen, csi!("{}D"), count);
    }

    fn save_position(&self, screen: &mut Screen) {
        write!(screen, csi!("s"));
    }

    fn reset_position(&self, screen: &mut Screen) {
        write!(screen, csi!("u"));
    }

    fn hide(&self, screen: &mut Screen) {
        write!(screen, csi!("?25l"));
    }

    fn show(&self, screen: &mut Screen) {
        write!(screen, csi!("?25h"));
    }

    fn blink(&self, blink: bool, screen: &mut Screen) {
        if blink {
            write!(screen, csi!("?12h"));
        } else {
            write!(screen, csi!("?12l"));
        }
    }
}
