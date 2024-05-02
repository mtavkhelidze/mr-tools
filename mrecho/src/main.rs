use clap::{ArgAction, Parser};

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Input text
    #[arg(value_name = "text", required = true)]
    strings: Vec<String>,

    /// Do not output final newline
    #[arg(short, action(ArgAction::SetTrue))]
    no_newline: Option<bool>,
}

fn main() {
    let cli = Cli::parse();
    let nl: &str = if cli.no_newline.unwrap_or(false) {
        ""
    } else {
        "\n"
    };
    let out = cli.strings.join(" ");
    print!("{}{}", out, nl);
}
