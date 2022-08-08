use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use std::{fs::File, io, path::PathBuf, process::ExitCode};
use thiserror::Error;

/// Generate completions.
#[derive(Debug, Parser)]
pub struct CompletionGenerator {
    /// Bin name.
    #[clap(long, short)]
    name: String,
    /// Target shell.
    #[clap(long, short, value_enum)]
    shell: Shell,
    /// Output file.
    #[clap(long, short)]
    output: PathBuf,
}

/// Error of the generator.
#[derive(Debug, Error)]
pub enum Error {
    /// Error caused by filesystem operation.
    #[error("{}: {error}", path.to_string_lossy())]
    FileSystem {
        /// Path in question.
        path: PathBuf,
        /// Emitted error.
        #[source]
        error: io::Error,
    },
}

impl CompletionGenerator {
    /// Run the generator.
    pub fn run<App: CommandFactory>(self) -> Result<(), Error> {
        let CompletionGenerator {
            name,
            shell,
            output,
        } = self;
        let mut cmd = App::command();
        let mut output_file = File::create(&output).map_err(|error| Error::FileSystem {
            path: output.clone(),
            error,
        })?;
        generate(shell, &mut cmd, name, &mut output_file);
        Ok(())
    }

    /// The program that generates shell completions.
    pub fn main<App: CommandFactory>() -> ExitCode {
        match CompletionGenerator::parse().run::<App>() {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("{error}");
                ExitCode::FAILURE
            }
        }
    }
}
