pub mod lib {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub type MRResult<T> = Result<T, Box<dyn std::error::Error>>;

    fn open(filename: &str) -> MRResult<Box<dyn std::io::BufRead>> {
        match filename {
            "-" => Ok(Box::new(std::io::stdin().lock())),
            _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
    }

    pub fn cat(file_names: &Vec<String>) -> MRResult<()> {
        let result = file_names.iter().map(|filename| {
            open(filename).map(|buf_reader| {
                buf_reader
                    .lines()
                    .for_each(|line| println!("{}", line.unwrap()));
            })
        });
        result.collect()
    }
}
