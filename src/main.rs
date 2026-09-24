mod cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary: Vec<u8> = std::fs::read("./roms/dmg_boot.bin")?;

    for byte in &binary {
        print!("{byte:02X} ");
    }

    Ok(())
}
