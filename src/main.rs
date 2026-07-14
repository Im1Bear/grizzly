mod cleaners;
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Eat,
    Clean,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Eat | Commands::Clean => {
            let (c1, s1) = cleaners::temp::clean_temp();
            let (c2, s2) = cleaners::browser::clean_browsers();
            let (c3, s3) = cleaners::prefetch::clean_prefetch();
            cleaners::eventlog::clean_eventlog();
            cleaners::recycle::clean_recycle();

            let total_count = c1 + c2 + c3;
            let total_mb = (s1 + s2 + s3) as f64 / 1_048_576.0;

            println!("\n🐻 Grizzly hat gefressen:");
            println!("├── Elemente: {}", total_count);
            println!("└── Größe:    {:.2} MB", total_mb);
        }
    }
}
