mod barplot;
mod describe;
mod scatter;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rasuka")]
#[command(
    about = "RASUKA [Rust Kaiseki]",
    long_about = "Welcome to RASUKA. 
        \nThis is an app to help you analyzing your data through command line
        \nPlease Choose your desired visulization
        \nThe output will be saved in the current project folder where this app is being run.
        \nCreated by: sagarabilly",
    version = "1.0"
)]
struct Cli {
    /// input your path <String>
    #[arg(short = 'p', long)]
    path: String,

    /// input your param x-axis column <String>
    #[arg(short = 'x', long)]
    paramx: String,

    /// input your param y-axis column <String>
    #[arg(short = 'y', long)]
    paramy: String,

    /// Choose Visualization [Look at Commands to see available visualization]
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Scatter,
    Barplot,
    Describe,
}

fn main() {
    let cli = Cli::parse();
    println!("-------------------------------------------------------------");
    println!("Detected file, {}", cli.path);
    println!("Inputted parameter, {} & {}", cli.paramx, cli.paramy);
    println!("-------------------------------------------------------------");

    match cli.command {
        Some(Commands::Scatter) => {
            println!("Showing Scatter Visualization:");
            match scatter::generate_scatter(&cli.path, &cli.paramx, &cli.paramy) {
                Ok(_) => println!("Scatter generated successfully."),
                Err(e) => eprintln!("Error generating scatter: {}", e),
            }
        }
        Some(Commands::Barplot) => {
            println!("Showing the Barplot Visualization");
            match barplot::generate_hb(&cli.path, &cli.paramx, &cli.paramy) {
                Ok(_) => println!("Barplot generated succesfully."),
                Err(e) => println!("Error generating barplot: {}", e),
            }
        }
        Some(Commands::Describe) => {
            println!("Describing Data");
            match describe::describe(&cli.path, &cli.paramx) {
                Ok(_) => println!("Data described successfully"),
                Err(e) => println!("Error in calculation! Please debug"),
            }
        }
        None => {
            println!("No command provided. Please specify the command!");
        }
    }
}
