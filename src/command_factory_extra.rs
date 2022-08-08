use crate::{completion_generator, CompletionGenerator};
use clap::CommandFactory;
use clap_complete::{generate, Shell};
use pipe_trait::Pipe;
use std::{io::Write, process::ExitCode, str};

/// Additional methods for [`CommandFactory`].
pub trait CommandFactoryExtra: CommandFactory {
    /// Get completion content.
    fn get_completion<Completion, Name>(name: Name, shell: Shell) -> Completion
    where
        Completion: Write + Default,
        Name: Into<String>,
    {
        let mut completion = Completion::default();
        let mut command = Self::command();
        generate(shell, &mut command, name, &mut completion);
        completion
    }

    /// Get completion string.
    fn get_completion_string(
        name: impl Into<String>,
        shell: Shell,
    ) -> Result<String, str::Utf8Error> {
        Self::get_completion::<Vec<u8>, _>(name, shell)
            .pipe_as_ref(str::from_utf8)
            .map(ToString::to_string)
    }

    /// Generate a shell completion file.
    fn generate_completion(
        generator: CompletionGenerator,
    ) -> Result<(), completion_generator::Error> {
        generator.run::<Self>()
    }

    /// Create and run the completion generator.
    fn run_completion_generator() -> ExitCode {
        CompletionGenerator::main::<Self>()
    }
}

impl<App: CommandFactory> CommandFactoryExtra for App {}
