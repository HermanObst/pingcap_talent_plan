use std::path::PathBuf;
use clap::{Parser, Subcommand};

///Does awesome things
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Args {
    /// Optional name to operate on
    name: Option<String>,

   /// Sets a custom config file
   #[arg(short, long, value_name = "FILE")]
   config: Option<PathBuf>,

   /// Sets the input file to use
   #[arg(short, long)]
   input: String,

   /// Sets the level of verbosity
   #[arg(short, long, action = clap::ArgAction::Count)] 
   verbose: u8,

   #[command(subcommand)]
   command: Option<Commands>,
}

#[derive(Debug)]
#[derive(Subcommand)]
enum Commands {
    // do testing things
    Test {
        // List test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {
    let args = Args::parse();
    println!("Hello {:?}!", args)
}   