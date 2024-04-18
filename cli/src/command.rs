use std::{fs::File, io::BufReader};
use clap::{command, Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    GenTrace(RunArgs),
    Prove(RunArgs),
}

#[derive(Args)]
pub struct RunArgs {
    #[arg(short, long)]
    pub trace: Option<String>,
    // #[arg(short, long)]
    // pub bytecode: Option<String>,
    // #[arg(short, long)]
    // pub hardcode: Option<String>,
    // #[arg(short, long)]
    // pub file: Option<String>,
    // // #[arg(short, long)]
    // // pub dry_run: bool,
}


pub fn match_operation(cli: &Cli) {
    match &cli.command {
        Commands::GenTrace(_args) => {
            gen_trace();
        }
        Commands::Prove(args) => {
            println!("create proof");

            // prove(steps, &mut rng);
        }
    }
}

fn gen_trace() {
    println!(
        "gen trace"
    )
}