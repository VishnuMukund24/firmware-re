use std::fs;
use std::io;
use std::path::Path;

/// Load a binary file into a Vec<u8>
pub fn load_firmware<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    let data = fs::read(&path)?;
    let size = data.len();
    let ext = path.as_ref()
                  .extension()
                  .and_then(|e| e.to_str())
                  .unwrap_or("unknown");

    println!("Loaded firmware:");
    println!("- Path: {:?}", path.as_ref());
    println!("- Extension: .{}", ext);
    println!("- Size: {} bytes", size);
    println!("- First 8 bytes: {:?}", &data[..8.min(size)]);

    Ok(data)
}