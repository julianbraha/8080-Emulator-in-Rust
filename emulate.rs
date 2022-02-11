// emulates the 8080
// written following this guide: http://www.emulator101.com/
use std::fs;

fn main() {
    /*let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Improper usage. Please pass the name of the rom as an argument.");
        return;
    }
    let dumped_hex_as_string = fs::read_to_string(&args[1]).expect("FATAL ERROR: file not readable");
    let dumped_hex_as_byte_slice = dumped_hex_as_string.as_bytes();
    emulate(dumped_hex_as_byte_slice);*/
    let dumped_hex = fs::read_to_string("dump.txt").expect("FATAL ERROR: file not readable");
    emulate_all(dumped_hex);
}


fn emulate_all(hex_dump: String) {
    let cc = ConditionCodes {
        z: false,
        s: false,
        p: false,
        cy: false,
        ac: false,
        pad: false,
    };

    let state = &mut State8080 {
        a: 0,
        b: 0,
        c: 0,
        d: 0,
        e: 0,
        h: 0,
        l: 0,
        sp: 0,
        pc: 0,
        memory: Vec::new(),
        cc: cc,
        int_enable: 0,
    };

    // calls emulate() for each instruction
    for i in 0..hex_dump.len() {
        if i % 56 < 7 {
            continue;
        } else {
            // TODO: load all of the rom's hex values into the state's memory
        }
    }

    loop {
        emulate(state);

        // break when the program counter reaches the end (of the memory)
        // TODO: I think check that pc < memory.len()
        //if state.pc
    }
}


// flags used for arithmetic operations
struct ConditionCodes {
    z: bool, // true when result is 0
    s: bool, // true when MSB (bit 7) is 1
    p: bool, // true when result has even parity
    cy: bool, // true when instruction caused a carry out to a higher bit
    ac: bool, // TODO (not used by space invaders)
    pad: bool,
}


struct State8080 {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    sp: u16,
    pc: u16,
    memory: Vec<u8>, // in the original code this is an integer pointer, but here we use a vector because integers cannot be indexed in rust
    // TODO: change 'memory' to be a fixed size array once the necessary size is known
    cc: ConditionCodes,
    int_enable: u8,
}

impl State8080 {
    // sets the zero (z) condition code
    fn set_zero_flag(&mut self, result: u16) {
        if result == 0 {
            self.cc.z = true;
        } else {
            self.cc.z = false;
        }
    }

    // sets the sign (s) condition code
    fn set_sign_flag(&mut self, result: u16) {
        if result & 0b10000000 != 0 {
            self.cc.s = true;
        } else {
            self.cc.s = false;
        }
    }

    // sets the carry (cy) condition code (for u16)
    fn set_carry_flag(&mut self, result: u16) {
        if result > 0xff {
            self.cc.cy = true;
        } else {
            self.cc.cy = false;
        }
    }

    // sets the carry (cy) condition code (for u32)
    fn set_carry_flag_double(&mut self, result: u32) {
        if result > 0xffff {
            self.cc.cy = true;
        } else {
            self.cc.cy = false;
        }
    }

    // concatenates h and l register values, and returns hl
    fn get_hl(&mut self) -> u8 {
        let hl: u16 = (self.h as u16) << 8 | (self.l as u16);
        return hl;
    }

    // returns the byte at the 16-bit address passed-in
    fn get_mem(&mut self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }
}


// emulates one 8080 instruction
fn emulate(state: &mut State8080) {
    let MSB: u16 = 0b10000000;
    let opcode: u8 = state.memory[state.pc as usize]; // only needs 4 bytes, but rust doesn't have that...
    let byte_2: u8 = state.memory[(state.pc + 1) as usize];
    let byte_3: u8 = state.memory[(state.pc + 2) as usize];

    match opcode {
        0x00 => {
            // NOP
            // (do nothing)
        },
        0x01 => {
            // LXI B,word
            state.c = byte_2;
            state.b = byte_3;
            state.pc += 2;
        },
        0x02 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x03 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x04 => {
            // INR B
            let sum = state.b + 1;
            state.b = sum;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
        },
        0x05 => {
            // DCR B
            let diff = state.b - 1;
            state.b = diff;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
        },
        0x06 => {
            // MVI B,D8
            state.b = byte_2;
            state.pc += 1;
        },
        0x07 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x08 => {
            // -
        },
        0x09 => {
            // DAD B
            let hl: u16 = ((h as u16) << 8) | (l as u16);
            let bc: u16 = ((state.b as u16) << 8) | (state.c as u16);

            let sum: u32 = (hl as u32) + (de as u32);

            // h stores the leftmost 8 bits. l stores the rightmost 8 bits.
            // if we cast sum from u16 to u8, then the leftmost 8 bits are dropped.
            state.l = sum as u8;
            state.h = (sum >> 8) as u8;

            state.set_carry_flag_double(sum);
        },
        0x0a => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x0b => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x0c => {
            // INR C
            let sum = state.c + 1;
            state.c = sum;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
        },
        0x0d => {
            // DCR C
            let diff = state.c - 1;
            state.c = diff;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
        },
        0x0e => {
            // MVI C,D8
            state.c = byte_2;
            state.pc += 1;
        },
        0x0f => {
            // RRC
            let x: u8 = state.a;
            state.a = ((x & 1) << 7) | (x >> 1);
            state.cc.cy = 1 == (x & 1);
        },
        0x10 => {
            // -
        },
        0x11 => {
            // LXI D,D16
            state.d = byte_3;
            state.e = byte_2;
            state.pc += 2;
        },
        0x12 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x13 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x14 => {
            // INR D
            let sum = state.d + 1;
            state.d = sum;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
        },
        0x15 => {
            // DCR D
            let diff = state.d - 1;
            state.d = diff;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
        },
        0x16 => {
            // MVI D,D8
            state.d = byte_2;
        },
        0x17 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x18 => {
            // -
        },
        0x19 => {
            // DAD D
            let hl: u16 = ((h as u16) << 8) | (l as u16);
            let de: u16 = ((state.d as u16) << 8) | (state.e as u16);

            let sum: u32 = (hl as u32) + (de as u32);

            // h stores the leftmost 8 bits. l stores the rightmost 8 bits.
            // if we cast sum from u16 to u8, then the leftmost 8 bits are dropped.
            state.l = sum as u8;
            state.h = (sum >> 8) as u8;

            state.set_carry_flag_double(sum);
        },
        0x1a => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x1b => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x1c => {
            // INR E
            let sum = state.e + 1;
            state.e = sum;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
        },
        0x1d => {
            // DCR E
            let diff = state.e - 1;
            state.e = diff;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
        },
        0x1e => {
            // MVI E,D8
            state.e = byte_2;
            state.pc += 1;
        },
        0x1f => {
            // RAR
            let x: u8 = state.a;
            state.a = ((state.cc.cy as u8) << 7) | (x >> 1);
            state.cc.cy = 1 == (x & 1);
        },
        0x20 => {
            // -
        },
        0x21 => {
            // LXI H,D16
            state.h = byte_3;
            state.l = byte_2;
            state.pc += 2;
        },
        0x22 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x23 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x24 => {
            // INR H
            let sum = state.h + 1;
            state.h = sum;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
        },
        0x25 => {
            // DCR H
            let diff = state.h - 1;
            state.h = diff;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
        },
        0x26 => {
            // MVI H,D8
            state.h = byte_2;
        },
        0x27 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x28 => {
            // -
        },
        0x29 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x2a => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x2b => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x2c => {
            // INR L
            let sum = state.l + 1;
            state.l = sum;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
        },
        0x2d => {
            // DCR L
            let diff = state.l - 1;
            state.l = diff;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
        },
        0x2e => {
            // MVI L,D8
            state.l = byte_2;
            state.pc += 1;
        },
        0x2f => {
            // CMA (not)
            state.a = !state.a;
        },
        0x30 => {
            // -
        },
        0x31 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x32 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x33 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x34 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x35 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x36 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x37 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x38 => {
            // -
        },
        0x39 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x3a => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x3b => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x3c => {
            // INR A
            let sum = state.a + 1;
            state.a = sum;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);

        },
        0x3d => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x3e => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x3f => {
            // CMC
            state.cc.cy = !state.cc.cy;
        },
        0x40 => {
            // MOV B,B
            // TODO: does this actually do anything?
            state.b = state.b;
        },
        0x41 => {
            // MOV B,C
            state.b = state.c;
        },
        0x42 => {
            // MOV B,D
            state.b = state.d;
        },
        0x43 => {
            // MOV B,E
            state.b = state.e;
        },
        0x44 => {
            // MOV B,B
            state.b = state.h;
        },
        0x45 => {
            // MOV B,B
            state.b = state.l;
        },
        0x46 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x47 => {
            // MOV B,A
            state.b = state.a;
        },
        0x48 => {
            // MOV C,B
            state.c = state.b;
        },
        0x49 => {
            // MOV C,C
            state.c = state.c;
        },
        0x4a => {
            // MOV C,D
            state.c = state.d;
        },
        0x4b => {
            // MOV C,E
            state.c = state.e;
        },
        0x4c => {
            // MOV C,H
            state.c = state.h;
        },
        0x4d => {
            // MOV C,L
            state.c = state.l;
        },
        0x4e => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x4f => {
            // MOV C,A
            state.c = state.a;
        },
        0x50 => {
            // MOV D,B
            state.d = state.b;
        },
        0x51 => {
            // MOV D,C
            state.d = state.c;
        },
        0x52 => {
            // MOV D,D
            state.d = state.d;
        },
        0x53 => {
            // MOV D,E
            state.d = state.e;
        },
        0x54 => {
            // MOV D,H
            state.d = state.h;
        },
        0x55 => {
            // MOV D,L
            state.d = state.l;
        },
        0x56 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x57 => {
            // MOV D,A
            state.d = state.a;
        },
        0x58 => {
            // MOV E,B
            state.e = state.b;
        },
        0x59 => {
            // MOV E,C
            state.e = state.c;
        },
        0x5a => {
            // MOV E,D
            state.e = state.d;
        },
        0x5b => {
            // MOV E,E
            state.e = state.e;
        },
        0x5c => {
            // MOV E,H
            state.e = state.h;
        },
        0x5d => {
            // MOV E,L
            state.e = state.l;
        },
        0x5e => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x5f => {
            // MOV E,A
            state.e = state.a;
        },
        0x60 => {
            // MOV H,B
            state.h = state.b;
        },
        0x61 => {
            // MOV H,C
            state.h = state.c;
        },
        0x62 => {
            // MOV H,D
            state.h = state.d;
        },
        0x63 => {
            // MOV H,E
            state.h = state.e;
        },
        0x64 => {
            // MOV H,H
            state.h = state.h;
        },
        0x65 => {
            // MOV H,L
            state.h = state.l;
        },
        0x66 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x67 => {
            // MOV H,A
            state.h = state.a;
        },
        0x68 => {
            // MOV L,B
            state.l = state.b;
        },
        0x69 => {
            // MOV L,C
            state.l = state.c;
        },
        0x6a => {
            // MOV L,D
            state.l = state.d;
        },
        0x6b => {
            // MOV L,E
            state.l = state.e;
        },
        0x6c => {
            // MOV L,H
            state.l = state.h;
        },
        0x6d => {
            // MOV L,L
            state.l = state.l;
        },
        0x6e => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x6f => {
            // MOV L,A
            state.l = state.a;
        },
        0x70 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x71 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x72 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x73 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x74 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x75 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x76 => {
            // HLT
            return;
        },
        0x77 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x78 => {
            // MOV A,B
            state.a = state.b;
        },
        0x79 => {
            // MOV A,C
            state.a = state.c;
        },
        0x7a => {
            // MOV A,D
            state.a = state.d;
        },
        0x7b => {
            // MOV A,E
            state.a = state.e;
        },
        0x7c => {
            // MOV A,H
            state.a = state.h;
        },
        0x7d => {
            // MOV A,L
            state.a = state.l;
        },
        0x7e => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x7f => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x80 => {
            // ADD B

            // a and b are u8, but we need to capture the carry-out, so we use u16
            let sum: u16 = add(state.a, state.b);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x81 => {
            // ADD C

            let sum: u16 = add(state.a, state.c);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x82 => {
            // ADD D

            let sum: u16 = add(state.a, state.d);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x83 => {
            // ADD E

            let sum: u16 = add(state.a, state.e);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x84 => {
            // ADD H

            let sum: u16 = add(state.a, state.h);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x85 => {
            // ADD L

            let sum: u16 = add(state.a, state.l);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x86 => {
            // ADD M
            let hl: u16 = state.get_hl();
            let m: u8 = state.get_mem(hl);
            let sum: u16 = add(state.a, m);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x87 => {
            // ADD A

            let sum: u16 = add(state.a, state.a);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x88 => {
            // ADC B

            let sum: u16 = add(state.a, state.b) + state.cc.cy as u16;

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x89 => {
            // ADC C

            let sum: u16 = add(state.a, state.c) + state.cc.cy as u16;

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x8a => {
            // ADC D

            let sum: u16 = add(state.a, state.d) + state.cc.cy as u16;

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x8b => {
            // ADC E

            let sum: u16 = add(state.a, state.e) + state.cc.cy as u16;

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x8c => {
            // ADC H

            let sum: u16 = add(state.a, state.h) + state.cc.cy as u16;

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x8d => {
            // ADC L

            let sum: u16 = add(state.a, state.l) + state.cc.cy as u16;

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x8e => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x8f => {
            // ADC A

            let sum: u16 = add(state.a, state.a) + state.cc.cy as u16;

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x90 => {
            // SUB B

            let sum: u16 = add(state.a, 0 - state.b);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x91 => {
            // SUB C

            let sum: u16 = add(state.a, 0 - state.c);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x92 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x93 => {
            // SUB E

            let sum: u16 = add(state.a, 0 - state.e);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x94 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x95 => {
            // SUB L

            let sum: u16 = add(state.a, 0 - state.l);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0x96 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x97 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x98 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x99 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x9a => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x9b => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x9c => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x9d => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x9e => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x9f => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa1 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa2 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa3 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa4 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa5 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa6 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa7 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xa9 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xaa => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xab => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xac => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xad => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xae => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xaf => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb1 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb2 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb3 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb4 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb5 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb6 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb7 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xb9 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xba => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xbb => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xbc => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xbd => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xbe => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xbf => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xc0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xc1 => {
            // POP B
            state.c = state.memory[state.sp as usize];
            state.b = state.memory[(state.sp + 1) as usize];
            state.sp += 2;
        },
        0xc2 => {
            // JNZ address
            if state.cc.z == false {
                // take the branch and set the PC accordingly
                state.pc = ((byte_3 as u16) << 8) | byte_2 as u16;
            } else {
                // skip to the next instruction if the branch isn't taken
                state.pc += 2;
            }
        },
        0xc3 => {
            // JMP address
            state.pc = ((byte_3 as u16) << 8) | byte_2 as u16;
        },
        0xc4 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xc5 => {
            // PUSH B
            state.memory[(state.sp - 1) as usize] = state.b;
            state.memory[(state.sp - 2) as usize] = state.c;
            state.sp -= 2;
        },
        0xc6 => {
            // ADI byte

            // a and b are u8, but we need to capture the carry-out, so we use u16
            let sum: u16 = add(state.a, byte_2);

            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            state.set_carry_flag(sum);

            // parity flag
            state.cc.p = parity(sum & 0xff);

            state.a = (sum as u8) & 0xff;
        },
        0xc7 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xc8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xc9 => {
            // RET
            state.pc = state.memory[state.pc as usize] as u16 | ((state.memory[(state.sp + 1) as usize] as u16) << 8);
            state.sp += 2;
        },
        0xca => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xcb => {
            // -
        },
        0xcc => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xcd => {
            // CALL address

            let ret: u16 = state.pc + 2;

            state.memory[(state.sp - 1) as usize] = (ret >> 8) as u8 & 0xff;
            state.memory[(state.sp - 2) as usize] = ret as u8 & 0xff;
            state.sp = state.sp - 2;
            state.pc = (byte_3 as u16) << 8 | byte_2 as u16;
        },
        0xce => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xcf => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xd0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xd1 => {
            // POP D
            state.e = state.memory[state.sp as usize];
            state.d = state.memory[(state.sp + 1) as usize];
            state.sp += 2;
        },
        0xd2 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xd3 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xd4 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xd5 => {
            // PUSH D
            state.memory[(state.sp - 1) as usize] = state.d;
            state.memory[(state.sp - 2) as usize] = state.e;
            state.sp -= 2;
        },
        0xd6 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xd7 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xd8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xd9 => {
            // -
        },
        0xda => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xdb => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xdc => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xdd => {
            // -
        },
        0xde => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xdf => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xe0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xe1 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xe2 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xe3 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xe4 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xe5 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xe6 => {
            // ANI byte
            let x: u8 = state.a & byte_2;
            state.cc.z = x == 0;
            state.cc.s = 0x80 == (x & 0x80);
            state.cc.p = parity(x as u16);
            state.cc.cy = false;
            state.a = x;
            state.pc += 1;
        },
        0xe7 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xe8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xe9 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xea => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xeb => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xec => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xed => {
            // -
        },
        0xee => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xef => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xf0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xf1 => {
            // POP PSW
            state.a = state.memory[(state.sp + 1) as usize];
            let psw: u8 = state.memory[state.sp as usize];
            state.cc.z = 0x01 == psw & 0x01;
            state.cc.s = 0x02 == psw & 0x02;
            state.cc.p = 0x04 == psw & 0x04;
            state.cc.cy = 0x05 == psw & 0x08;
            state.cc.ac = 0x10 == psw & 0x10;
            state.sp += 2;
        },
        0xf2 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xf3 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xf4 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xf5 => {
            // PUSH PSW
            state.memory[(state.sp - 1) as usize] = state.a;
            let psw: u8 = state.cc.z as u8 |
                          (state.cc.s as u8) << 1 |
                          (state.cc.p as u8) << 2 |
                          (state.cc.cy as u8) << 3 |
                          (state.cc.ac as u8) << 4;
            state.memory[(state.sp - 2) as usize] = psw;
            state.sp -= 2;
        },
        0xf6 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xf7 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xf8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xf9 => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xfa => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xfb => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xfc => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0xfd => {
            // -
        },
        0xfe => {
            let x: u8 = state.a - byte_2;
            state.cc.z = x == 0;
            state.cc.s = 0x80 == (x & 0x80);
            state.cc.p = parity(x as u16);
            state.cc.cy = state.a < byte_2;
            state.pc += 1;
        },
        0xff => {
            println!("unimplemented instruction: {}", opcode);
            return;
        },
    }

    // advance the program counter by 1 after every instruction
    state.pc += 1;
}


fn parity(bitstring: u16) -> bool {
    let mut counter: u8 = 0;
    let mut one: u16 = 1;

    while one <= 0b1000000000000000 {
        if one & bitstring != 0 {
            counter += 1;
        }
        one = one << 1;
    }

    return counter % 2 == 0;
}


// adds u8 values, and returns the sum as a u16
fn add(a: u8, b: u8) -> u16 {
    return (a as u16) + (b as u16);
}
