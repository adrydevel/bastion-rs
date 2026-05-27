use clap::Parser;
mod config; mod risk; mod council; mod chain; mod cli;
use cli::{Cli, Cmd};

fn main() {
    let args = Cli::parse();
    match args.command {
        Cmd::Run { ticker } => {
            let votes = vec![council::Side::Long, council::Side::Long, council::Side::Flat];
            let side = council::decide(&votes, config::Config::default().quorum);
            println!("bastion: {ticker} -> {:?}", side);
        }
    }
}
