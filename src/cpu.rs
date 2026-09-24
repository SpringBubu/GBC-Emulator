enum RegisterName {
    // 8-bit registers
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,

    // 16-bit registers
    AF,
    BC,
    DE,
    HL,

    SP,
    PC,
}

enum FlagName {
    Z,
    N,
    H,
    C,
}

struct Flags {
    zero: bool,
    subtraction: bool,
    half_carry: bool,
    carry: bool,
}

struct Register {
    a: u8,
    f: Flags,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}

impl Register {
    pub fn new(flag_state: bool) -> Self {
        Self {
            a: 0x01,
            f: Flags {
                zero: true,
                subtraction: false,
                half_carry: flag_state,
                carry: flag_state,
            },
            b: 0x00,
            c: 0x12,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,
            sp: 0xFFEE,
            pc: 0x0100,
        }
    }
}
