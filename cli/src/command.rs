use clap::{command, Args, Parser, Subcommand};
use std::{fs::File, io::BufReader};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    GenTrace(RunArgs),
    Prove(RunArgs),
    GenPreprocess(RunArgs),
}

#[derive(Args)]
pub struct RunArgs {
    #[arg(short, long)]
    pub trace: Option<String>,
    #[arg(short, long)]
    pub guest: Option<String>,
    #[arg(short, long)]
    pub func: Option<String>,
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
        Commands::GenPreprocess(args) => {
            gen_preprocess(args);
        }
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
    println!("gen trace")
}

fn gen_preprocess(args: &RunArgs) {
    println!("gen preprocess");

    if let Some(guest) = args.guest.clone() {
        if let Some(func) = args.func.clone() {
            entries::generate_preprocess(&guest, &func);
            return;
        }
    }
    println!("guest and func args are required");
}
