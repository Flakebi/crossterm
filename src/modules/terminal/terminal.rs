//! With this module you can perform actions that are terminal related.
//! Like clearing and scrolling in the terminal or getting the size of the terminal.

use super::*;

use std::fmt;

/// Struct that stores an specific platform implementation for terminal related actions.
///
/// Check `/examples/terminal` in the library for more specific examples.
///
/// ```rust
/// use crossterm::terminal;
///
/// let screen = Screen::default();
/// let term = terminal(&screen);
///
/// term.scroll_down(5);
/// term.scroll_up(4);
/// let (with, height) = term.terminal_size();
///
/// ```
pub struct Terminal<'stdout> {
    terminal: Box<ITerminal + Sync + Send>,
    screen: &'stdout Arc<TerminalOutput>,
}

impl<'stdout> Terminal<'stdout> {
    /// Create new terminal instance whereon terminal related actions can be performed.
    pub fn new(screen: &'stdout Arc<TerminalOutput>) -> Terminal<'stdout> {
        #[cfg(target_os = "windows")]
        let terminal = functions::get_module::<Box<ITerminal + Sync + Send>>(
            Box::new(WinApiTerminal::new()),
            Box::new(AnsiTerminal::new()),
        ).unwrap();

        #[cfg(not(target_os = "windows"))]
        let terminal = Box::from(AnsiTerminal::new()) as Box<ITerminal + Sync + Send>;

        Terminal {
            terminal,
            screen: screen,
        }
    }

    /// Get the terminal size (x,y).
    ///
    /// ```rust
    /// let screen = Screen::default();
    /// let mut term = terminal(&screen);
    ///
    /// let size = term.terminal_size();
    /// println!("{:?}", size);
    ///
    /// ```
    pub fn terminal_size(&self) -> (u16, u16) {
        return self.terminal.terminal_size(&self.screen);
    }

    pub fn clear(&self, clear_type: ClearType, screen: &mut Screen) {
        self.terminal.clear(clear_type, screen)
    }

    /// Exit the current process.
    ///
    /// ```rust
    /// let screen = Screen::default();
    /// let mut term = terminal(&screen);
    ///
    /// let size = term.exit();
    ///
    /// ```
    pub fn exit(&self) {
        self.terminal.exit(&self.screen);
    }
}

/// Get an terminal implementation whereon terminal related actions could performed.
/// Pass the reference to any screen you want this type to perform actions on.
pub fn terminal<'stdout>(screen: &'stdout Screen) -> Terminal<'stdout> {
    Terminal::new(&screen.stdout)
}
