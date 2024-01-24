use std::error::Error;

use crossterm::style::{Color, SetForegroundColor};

pub fn print_pretty_error<E>(error: E) -> E
where
    E: AsRef<dyn Error>,
{
    let set_color = SetForegroundColor(Color::Red);
    let reset_color = SetForegroundColor(Color::Reset);

    eprint!("{}", set_color);
    eprintln!("-------------------------------");
    eprintln!(" Error:");
    eprint!("{}", reset_color);

    // Message
    eprintln!("  {}", error.as_ref());

    if let Some(source) = error.as_ref().source() {
        eprint!("\n\n");
        eprint!("{}", set_color);
        eprintln!(" Caused by:");
        eprint!("{}", reset_color);
        eprintln!("  {}", source);
    }

    eprint!("{}", set_color);
    eprintln!("-------------------------------");
    eprint!("{}", reset_color);

    error
}
