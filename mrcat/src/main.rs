use mrcat;
use mrcat::{cat, get_args};

fn main() {
    let config = get_args().unwrap();
    cat(&config).unwrap();
}
