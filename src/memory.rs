// Address Space Mappings
const ROM_BANK_00_START: u16 = 0x0000;
const ROM_BANK_00_END: u16 = 0x3FFF;
const ROM_BANK_01_START: u16 = 0x4000;
const ROM_BANK_01_END: u16 = 0x7FFF;

const VIDEO_RAM_START: u16 = 0x8000;
const VIDEO_RAM_END: u16 = 0x9FFF;

const EXTERNAL_RAM_START: u16 = 0xA000;
const EXTERNAL_RAM_END: u16 = 0xBFFF;

const WORK_RAM_START: u16 = 0xC000; // TODO: 0xD000 seemingly important for CGB mode
const WORK_RAM_END: u16 = 0xDFFF;

const ECHO_RAM_START: u16 = 0xE000;
const ECHO_RAM_END: u16 = 0xFDFF;

const OBJECT_ATTRIBUTE_MEMORY_START: u16 = 0xFE00;
const OBJECT_ATTRIBUTE_MEMORY_END: u16 = 0xFE9F;

const NOT_USABLE_START: u16 = 0xFEA0;
const NOT_USABLE_END: u16 = 0xFEFF;

const IO_REGISTERS_START: u16 = 0xFF00;
const IO_REGISTERS_END: u16 = 0xFF7F;

const HIGH_RAM_START: u16 = 0xFF80;
const HIGH_RAM_END: u16 = 0xFFFE;

const INTERRUPT_ENABLE_REGISTER: u16 = 0xFFFF;

const VRAM_SIZE: usize = (EXTERNAL_RAM_START - VIDEO_RAM_START) as usize;
const WRAM_SIZE: usize = (ECHO_RAM_START - WORK_RAM_START) as usize;
const HRAM_SIZE: usize = (INTERRUPT_ENABLE_REGISTER - HIGH_RAM_START) as usize;

pub struct Mmu {
    // ROM
    // ERAM
    vram: [u8; VRAM_SIZE],
    wram: [u8; WRAM_SIZE],
    // OAM
    hram: [u8; HRAM_SIZE],
}

impl Mmu {
    pub fn new() -> Self {
        Self {
            vram: [0; VRAM_SIZE],
            wram: [0; WRAM_SIZE],
            hram: [0; HRAM_SIZE],
        }
    }

    pub fn read(&self, address: u16) -> u8 {
        match address {
            ROM_BANK_00_START..=ROM_BANK_00_END => 0,
            ROM_BANK_01_START..=ROM_BANK_01_END => 0,
            VIDEO_RAM_START..=VIDEO_RAM_END => 0,
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => 0,
            WORK_RAM_START..=WORK_RAM_END => 0,
            ECHO_RAM_START..=ECHO_RAM_END => 0,
            OBJECT_ATTRIBUTE_MEMORY_START..=OBJECT_ATTRIBUTE_MEMORY_END => 0,
            NOT_USABLE_START..=NOT_USABLE_END => 0,
            IO_REGISTERS_START..=IO_REGISTERS_END => 0,
            HIGH_RAM_START..=HIGH_RAM_END => 0,
            INTERRUPT_ENABLE_REGISTER => 0,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        match address {
            ROM_BANK_00_START..=ROM_BANK_00_END => 0,
            ROM_BANK_01_START..=ROM_BANK_01_END => 0,
            VIDEO_RAM_START..=VIDEO_RAM_END => 0,
            EXTERNAL_RAM_START..=EXTERNAL_RAM_END => 0,
            WORK_RAM_START..=WORK_RAM_END => 0,
            ECHO_RAM_START..=ECHO_RAM_END => 0,
            OBJECT_ATTRIBUTE_MEMORY_START..=OBJECT_ATTRIBUTE_MEMORY_END => 0,
            NOT_USABLE_START..=NOT_USABLE_END => 0,
            IO_REGISTERS_START..=IO_REGISTERS_END => 0,
            HIGH_RAM_START..=HIGH_RAM_END => 0,
            INTERRUPT_ENABLE_REGISTER => 0,
        };
    }
}
