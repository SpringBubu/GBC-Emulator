mod cpu;
mod registers;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary: Vec<u8> = std::fs::read("./roms/dmg_boot.bin")?;

    for byte in &binary {
        print!("0x{byte:02X}, ");
    }

    cpu::Cpu::test(binary);

    Ok(())
}
