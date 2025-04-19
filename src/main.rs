mod loader;

fn main() {
    let path = "firmware.bin"; // Change this to any binary file you have
    match loader::load_firmware(path) {
        Ok(_) => println!("Firmware loaded successfully!"),
        Err(e) => eprintln!("Error loading firmware: {}", e),
    }
}
