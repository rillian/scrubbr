use std::io;
use std::path::Path;
use std::ffi::OsStr;
use std::io::BufRead;

fn walk(start_path: &Path) -> io::Result<()> {
    for entry in std::fs::read_dir(start_path)? {
        let path = entry?.path();
        if let Some(name) = path.file_name().and_then(|x| x.to_str()) {
            // Skip dotfiles and directories.
            if name.starts_with(".") {
                continue;
            }
        }
        if path.is_dir() {
            println!(" dir {:?}", path);
            walk(&path)?;
        } else {
            if path.file_stem() == Some(OsStr::new("SHA256SUMS")) {
                println!("SHA2 {:?}", path);
                validate(&path)?;
            } else {
                println!("file {:?}", path);
            }
        }
    }
    Ok(())
}

fn validate(checksum_path: &Path) -> io::Result<()> {
    let file = std::fs::File::open(checksum_path)?;
    let file = std::io::BufReader::new(file);
    for line in file.lines() {
        println!("  {}", line?);
    }
    Ok(())
}

fn main() {
    println!("Hello, world! Let's see what we have...");
    let dirname = Path::new(".");
    walk(dirname).expect("Failed to walk directory");
}
