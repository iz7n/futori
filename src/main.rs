use std::{env, fs, io::Error, path::Path};
use unicode_normalization::UnicodeNormalization;

fn main() -> Result<(), Error> {
    let args = env::args();
    for arg in args.skip(1) {
        let path = Path::new(&arg);
        let half_width = fs::read_to_string(path).unwrap();

        let full_width = half_width.chars().nfkc().stream_safe().collect::<String>();
        fs::write(path, full_width)?;
    }
    Ok(())
}
