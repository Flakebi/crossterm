use modules::terminal::ansi_terminal::AnsiTerminal;

use modules::terminal::ITerminal;

use Screen;

/* ======================== WinApi =========================== */
#[cfg(windows)]
mod winapi_tests {
    use modules::terminal::winapi_terminal::WinApiTerminal;
    use super::*;

    #[test]
    fn resize_winapi()
    {
        let screen = Screen::default();
        let terminal = WinApiTerminal::new();

        terminal.set_size(10, 10, &screen.stdout);

        let (x, y) = terminal.terminal_size(&screen.stdout);

        assert_eq!(x, 10);
        assert_eq!(y, 10);
    }
}

/* ======================== ANSI =========================== */
#[test]
fn resize_ansi()
{
    use std::{thread, time};
    if try_enable_ansi() {
        let screen = Screen::default();
        let terminal = AnsiTerminal::new();

        terminal.set_size(50,50, &screen.stdout);

        // see issue: https://github.com/eminence/terminal-size/issues/11
        thread::sleep(time::Duration::from_millis(30));

        let (x, y) = terminal.terminal_size(&screen.stdout);

        assert_eq!(x, 50);
        assert_eq!(y, 50);
    }
}

fn try_enable_ansi() -> bool
{
    #[cfg(windows)]
        {
            if cfg!(target_os = "windows") {
                use kernel::windows_kernel::ansi_support::try_enable_ansi_support;

                if !try_enable_ansi_support()
                    { return false; }
            }
        }

    return true;
}
