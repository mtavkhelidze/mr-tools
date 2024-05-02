use clap::{ArgAction, Parser};

use mrcat::lib::{cat, MRResult};

fn main() {
    let config = get_args().unwrap();
    cat(&config.file_names).unwrap();
}

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Arguments {
    /// Input files
    #[arg(value_name = "file(s)", required = true)]
    file_names: Vec<String>,

    /// Output line numbers
    #[arg(short, action(ArgAction::SetTrue))]
    number_lines: Option<bool>,

    /// Number non-blank lines
    #[arg(short = 'b', action(ArgAction::SetTrue))]
    number_non_blanks: Option<bool>,
}

fn get_args() -> MRResult<Arguments> {
    let config = Arguments::parse();
    Ok(config)
}
