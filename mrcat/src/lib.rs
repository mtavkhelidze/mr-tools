pub mod lib {
    use std::error::Error;
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    pub type MRResult<T> = Result<T, Box<dyn Error>>;

    fn open(filename: &str) -> MRResult<Box<dyn BufRead>> {
        match filename {
            "-" => Ok(Box::new(std::io::stdin().lock())),
            _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
        }
    }

    fn print_line(number_lines: bool, line_res: (usize, Result<String, std::io::Error>)) -> () {
        match line_res.1 {
            Ok(line) => {
                if number_lines {
                    println!("{} {}", line_res.0, line);
                    ()
                } else {
                    println!("{}", line)
                }
            }
            Err(e) => {
                eprintln!("Cannot output line {}: {:?}", line_res.0, e)
            }
        };
    }

    fn print_lines(number_lines: bool) -> impl FnMut(Box<dyn BufRead>) -> () {
        return move |buf_reader| {
            buf_reader
                .lines()
                .enumerate()
                .for_each(|lineRes| print_line(number_lines, lineRes));
        };
    }
    pub fn cat(file_names: &Vec<String>, number_lines: bool) -> MRResult<()> {
        let result = file_names
            .iter()
            .flat_map(|filename| open(filename))
            .for_each(print_lines(number_lines));
        Ok(())
    }
}
