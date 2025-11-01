use clap::{CommandFactory, Parser};
use clap_complete::{generate, Shell};
use derive_more::{Display, Error};
use std::{
    fs::File,
    io,
    path::{Path, PathBuf},
    process::ExitCode,
};

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

/// Error caused by filesystem operation.
#[derive(Debug, Display, Error)]
#[display("{}: {error}", path.to_string_lossy())]
pub struct FileSystemError {
    /// Path in question.
    #[error(not(source))]
    path: PathBuf,
    /// Emitted error.
    #[error(source)]
    error: io::Error,
}

impl FileSystemError {
    /// Create the error.
    fn new(path: PathBuf, error: io::Error) -> Self {
        Self { path, error }
    }

    /// Get the path that caused the error.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Get the source of the error.
    pub fn source(&self) -> &io::Error {
        &self.error
    }
}

/// Error of the generator.
#[derive(Debug, Display, Error)]
#[non_exhaustive]
pub enum Error {
    /// Error caused by filesystem operation.
    FileSystem(FileSystemError),
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
        let mut output_file = match File::create(&output) {
            Ok(output_file) => output_file,
            Err(error) => return Err(Error::FileSystem(FileSystemError::new(output, error))),
        };
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
