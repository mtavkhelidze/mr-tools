use clap::{ArgAction, Parser};

type MRResult<T> = Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Arguments {
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

pub fn get_args() -> MRResult<Arguments> {
    let config = Arguments::parse();
    Ok(config)
}

pub fn open(filename: &str) -> MRResult<Box<dyn std::io::BufRead>> {
    match filename {
        "-" => Ok(Box::new(std::io::stdin().lock())),
        _ => Ok(Box::new(std::io::BufReader::new(std::fs::File::open(
            filename,
        )?))),
    }
}

pub fn cat(config: &Arguments) -> MRResult<()> {
    for filename in &config.file_names {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(_) => println!("Opened {}", filename),
        }
    }
    Ok(())
}
