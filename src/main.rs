use clap::{Parser, ArgAction};

#[derive(Debug)]
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input text
    #[arg(value_name = "text", required = true)]
    strings: Vec<String>,

    /// Do not print newline
    #[arg(short, action(ArgAction::SetTrue))]
    no_newlines: Option<bool>,

}

fn main() {
    let cli = Cli::parse();
    let nl = if cli.no_newlines.unwrap_or(false) { "" } else { "\n" };
    print!("{}{}", cli.strings.join(" "), nl);
}
