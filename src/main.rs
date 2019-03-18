use std::io;

fn walk(start_path: &str) -> io::Result<()> {
    for entry in std::fs::read_dir(start_path)? {
        let path = entry?.path();
        if path.is_dir() {
            println!("directory {:?}", path);
        } else {
            println!("file {:?}", path);
        }
    }
    Ok(())
}

fn main() {
    println!("Hello, world! Let's see what we have...");
    walk(".").expect("Failed to walk directory");
}
