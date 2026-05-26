use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "bastion", about = "Autonomous verifiable AI fund (rust port)")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Cmd,
}

#[derive(Subcommand)]
pub enum Cmd {
    /// Run one research -> debate -> verdict pass
    Run { #[arg(short, long, default_value = "NVDA")] ticker: String },
}
