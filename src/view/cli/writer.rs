use std::fmt::Display;
use std::io::{Stdout, Write};

use crossterm::cursor::{RestorePosition, SavePosition};
use crossterm::style::{Print, PrintStyledContent};
use crossterm::terminal::{Clear, ClearType};
use crossterm::{ExecutableCommand, QueueableCommand};
use tokio::sync::oneshot;
use tokio::time::Duration;

use crate::view::style::StyledString;

pub trait CliWriterRepository {
    fn clear_current_line(&mut self);

    fn print_lines<T: Display>(&mut self, lines: impl IntoIterator<Item = T>);
    fn print_lines_styled(&mut self, lines: impl IntoIterator<Item = Vec<StyledString>>);

    fn print_animation_single_line_styled<
        Sprites: IntoIterator<Item = Vec<StyledString>> + Sized + Clone,
    >(
        &mut self,
        sprites: Sprites,
        interval: Duration,
        finisher: oneshot::Receiver<()>,
    ) where
        <Sprites as IntoIterator>::IntoIter: Clone;

    fn print_animation_single_line<T: Display, Sprites: IntoIterator<Item = T> + Sized + Clone>(
        &mut self,
        sprites: Sprites,
        interval: Duration,
        finisher: oneshot::Receiver<()>,
    ) where
        <Sprites as IntoIterator>::IntoIter: Clone;
}

pub struct CrosstermCliWriter {
    pub stdout: Stdout,
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

    fn print_lines_styled(&mut self, lines: impl IntoIterator<Item = Vec<StyledString>>) {
        for line in lines {
            for word in line {
                self.stdout.queue(PrintStyledContent(word.into())).unwrap();
            }
            self.stdout.queue(Print("\n")).unwrap();
        }
        self.stdout.flush().unwrap();
    }

    fn print_animation_single_line_styled<
        Sprites: IntoIterator<Item = Vec<StyledString>> + Sized + Clone,
    >(
        &mut self,
        sprites: Sprites,
        interval: Duration,
        mut finisher: oneshot::Receiver<()>,
    ) where
        <Sprites as IntoIterator>::IntoIter: Clone,
    {
        self.stdout.execute(SavePosition).unwrap();

        for sprites_line in sprites.into_iter().cycle() {
            sprites_line.into_iter().for_each(|sprite| {
                self.stdout
                    .queue(PrintStyledContent(sprite.into()))
                    .unwrap();
            });

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
