use std::{env, fs, path::Path};
use unicode_normalization::UnicodeNormalization;

fn main() -> Result<(), String> {
    let args = env::args();
    let args = args.skip(1);
    if args.len() < 1 {
        return Err("Expected at least one argument".into());
    }
    for arg in args.skip(1) {
        let path = Path::new(&arg);
        let half_width = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(_) => return Err(format!("Failed to read from {}", path.display())),
        };

        let full_width = half_width.chars().nfkc().stream_safe().collect::<String>();
        if fs::write(path, full_width).is_err() {
            return Err(format!("Failed to write to {}", path.display()));
        }
    }
    Ok(())
}
