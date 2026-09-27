mod cpu;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let binary: Vec<u8> = std::fs::read("./roms/dmg_boot.bin")?;

    for byte in &binary {
        print!("0x{byte:02X}, ");
    }

    let mut cpu = cpu::Cpu::new();
    cpu.test(binary);

    Ok(())
}
