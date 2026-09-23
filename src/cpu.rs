enum Register8BitID {
    A,
    B,
    C,
    D,
    E,
    F,
    L,
}

enum Register16BitID {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

enum Flags {
    Zero,
    Subtraction,
    HalfCarry,
    Carry,
}

struct Register {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
}


