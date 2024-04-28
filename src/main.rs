use clap::{Parser, ArgAction};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input text
    #[arg(value_name = "INPUT")]
    input: String,

    /// Omit newlines
    #[arg(short, action(ArgAction::SetTrue))]
    no_newlines: Option<bool>,

}

fn main() {
    let cli = Cli::parse();
}
