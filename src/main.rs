use anyhow::Result;

use council::{cli::{Cli, Commands}, run::execute_run, verify::execute_verify};

fn main() -> Result<()> {
    let cli = <Cli as clap::Parser>::parse();

    match cli.command {
        Commands::Run { task, policy, out } => execute_run(&task, &policy, &out),
        Commands::Verify { run } => execute_verify(&run).map(|_| ()),
    }
}
