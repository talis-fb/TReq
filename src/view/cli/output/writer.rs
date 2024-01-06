use std::fmt::Display;
use std::io::Write;

use crossterm::cursor::{RestorePosition, SavePosition};
use crossterm::style::{Print, PrintStyledContent};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use tokio::sync::oneshot;
use tokio::time::Duration;

use crate::view::style::StyledStr;

pub trait CliWriterRepository {
    fn clear_current_line(&mut self);

    fn print_lines<T: Display>(&mut self, lines: impl IntoIterator<Item = T>);

    fn print_animation_single_line<T: Display, Sprites: IntoIterator<Item = T> + Sized + Clone>(
        &mut self,
        sprites: Sprites,
        interval: Duration,
        finisher: oneshot::Receiver<()>,
    ) where
        <Sprites as IntoIterator>::IntoIter: Clone;

    fn print_centered_text_with_border(&mut self, text: &str, border_char: char);

    fn print_lines_styled<'a, StyledValues>(
        &mut self,
        lines: impl IntoIterator<Item = StyledValues>,
    ) where
        StyledValues: IntoIterator<Item = StyledStr<'a>>;
}

pub struct CrosstermCliWriter {
    pub stdout: Box<dyn Write + Send>,
}

impl CliWriterRepository for CrosstermCliWriter {
    fn clear_current_line(&mut self) {
        self.stdout.queue(Clear(ClearType::CurrentLine)).unwrap();
        self.stdout.flush().unwrap();
    }

    fn print_lines<T: Display>(&mut self, lines: impl IntoIterator<Item = T>) {
        for line in lines {
            self.stdout
                .queue(Print(line))
                .unwrap()
                .queue(Print("\n"))
                .unwrap();
        }
        self.stdout.flush().unwrap();
    }

    fn print_lines_styled<'a, StyledValues>(
        &mut self,
        lines: impl IntoIterator<Item = StyledValues>,
    ) where
        StyledValues: IntoIterator<Item = StyledStr<'a>>,
    {
        for line in lines {
            for word in line {
                self.stdout.queue(PrintStyledContent(word.into())).unwrap();
            }
            self.stdout.queue(Print("\n")).unwrap();
        }
        self.stdout.flush().unwrap();
    }

    fn print_centered_text_with_border(&mut self, text: &str, border_char: char) {
        let (col, rows) = crossterm::terminal::size().unwrap();
        let w = crossterm::terminal::window_size().unwrap();
        let w_col = w.columns;
        let w_r = w.rows;

        println!("o coisa {col} {rows}");
        println!("o coisa {w_col} {w_r}");

        let das = crossterm::terminal::window_size().unwrap();
        let rows = das.rows;

        let size_border = ((rows / 2) - ((text.len() as u16) / 2))
            .checked_sub(1)
            .unwrap();

        for _ in 0..size_border {
            self.stdout.execute(Print(border_char)).unwrap();
        }

        self.stdout.execute(Print(text)).unwrap();

        for _ in 0..size_border {
            self.stdout.execute(Print(border_char)).unwrap();
        }

        self.stdout.flush().unwrap();
    }

    fn print_animation_single_line<T: Display, Sprites: IntoIterator<Item = T> + Sized + Clone>(
        &mut self,
        sprites: Sprites,
        interval: Duration,
        mut finisher: oneshot::Receiver<()>,
    ) where
        <Sprites as IntoIterator>::IntoIter: Clone,
    {
        self.stdout.execute(SavePosition).unwrap();

        for sprite in sprites.into_iter().cycle() {
            self.stdout.queue(Print(sprite)).unwrap();
            self.stdout.flush().unwrap();

            std::thread::sleep(interval);

            self.stdout
                .queue(RestorePosition)
                .unwrap()
                .queue(Clear(ClearType::CurrentLine))
                .unwrap();

            self.stdout.flush().unwrap();

            if finisher.try_recv().is_ok() {
                break;
            }
        }
    }
}
