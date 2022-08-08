use crate::{completion_generator, CompletionGenerator};
use clap::CommandFactory;
use std::process::ExitCode;

/// Additional methods for [`CommandFactory`].
pub trait CommandFactoryExtra: CommandFactory {
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
