// emulates the 8080
// written following this guide: http://www.emulator101.com/
use std::fs;
use std::i64;
use std::env;

use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Improper usage. Please pass the name of the hexdump file to emulate as an argument.");
        return;
    }

    // read the hex into a vector of strings, ignoring the line numbers
    let mut hex_strings: Vec<&str> = Vec::new();
    let reader = BufReader::new(File::open(&args[1]).expect("Cannot open file."));
    let l: Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    for i in 0..l.len() {
        let strings: Vec<&str> = l[i].split_whitespace().collect();
        for j in 0..strings.len() {
            // every 9 strings is just the line number
            if j % 9 != 0 {
                hex_strings.push(strings[j]);
            }
        }
    }

    // convert the hex strings into unsigned 16-bit integers
    let mut rom: Vec<u16> = Vec::new();
    for i in 0..hex_strings.len() {
        let int_rep = u16::from_str_radix(hex_strings[i], 16).unwrap();
        rom.push(int_rep);
    }

    emulate_all(rom);

}


fn emulate_all(hex_dump: Vec<u16>) {
    let cc = ConditionCodes {
        z: false,
        s: false,
        p: false,
        cy: false,
        ac: false,
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
    };

    // loads the rom into memory
    for i in 0..hex_dump.len() {
        // memory is in bytes, but our hexdump is in format xxxx xxxx ...
        // so we need to load in the leftmost 8 bits of each value, then the rightmost 8 bits.
        state.memory.push((hex_dump[i] >> 8) as u8);
        state.memory.push(hex_dump[i] as u8);
    }

    loop {
        emulate(state);
        println!("state is: {}", state.clone().dump_state());

        // break when the program counter reaches the end (of the memory)
        // TODO: I think check that pc < memory.len()
        //if state.pc
    }
}


// flags used for arithmetic operations
#[derive(Clone)]
struct ConditionCodes {
    z: bool, // true when result is 0
    s: bool, // true when MSB (bit 7) is 1
    p: bool, // true when result has even parity
    cy: bool, // true when instruction caused a carry out to a higher bit
    ac: bool, // TODO (not used by space invaders)
}

#[derive(Clone)]
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
}

impl State8080 {
    // for debugging. converts the fields to strings for printing.
    fn dump_state(self) -> String {
        let mut s = "a:".to_string();
        s.push_str(&self.a.to_string());
        s.push_str(" b:");
        s.push_str(&self.b.to_string());
        s.push_str(" c:");
        s.push_str(&self.c.to_string());
        s.push_str(" d:");
        s.push_str(&self.d.to_string());
        s.push_str(" e:");
        s.push_str(&self.e.to_string());
        s.push_str(" h:");
        s.push_str(&self.h.to_string());
        s.push_str(" l:");
        s.push_str(&self.l.to_string());
        s.push_str(" sp:");
        s.push_str(&self.sp.to_string());
        s.push_str(" pc:");
        s.push_str(&self.pc.to_string());
        s.push_str(" memory size:");
        s.push_str(&self.memory.len().to_string());

        return s;
    }

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
    fn get_hl(&mut self) -> u16 {
        let hl: u16 = (self.h as u16) << 8 | (self.l as u16);
        return hl;
    }

    // returns the byte at the 16-bit address passed-in
    fn get_mem(&mut self, addr: u16) -> u8 {
        return self.memory[addr as usize];
    }

    // sets the byte at the 16-bit address passed-in
    fn set_mem(&mut self, addr: u16, val: u8) {
        self.memory[addr as usize] = val;
    }
}


// emulates one 8080 instruction
fn emulate(state: &mut State8080) {
    let opcode: u8 = state.get_mem(state.pc); // only needs 4 bytes, but rust doesn't have that...
    let byte_2: u8 = state.get_mem(state.pc + 1);
    let byte_3: u8 = state.get_mem(state.pc + 2);
    println!("opcode: {}", opcode);
    println!("byte 2 is: {}", byte_2);
    println!("byte 3 is: {}", byte_3);
    match opcode {
        0x00 => {
            // NOP
            // (do nothing)
            state.pc += 1;
        },
        0x01 => {
            // LXI B,word
            state.c = byte_2;
            state.b = byte_3;
            state.pc += 2;
        },
        0x02 => {
            // STAX B
            let bc: u16 = ((state.b as u16) << 8) | (state.c as u16);
            state.set_mem(bc, state.a);
            state.pc += 1;
        },
        0x03 => {
            // INX B
            let bc: u16 = ((state.b as u16) << 8) | (state.c as u16);
            let bc_inc: u16 = bc + 1;

            // shift out the rightmost 8 bits, and truncate to use only the remaining 8 bits.
            state.b = (bc_inc >> 8) as u8;

            // truncate the leftmost 8 bits
            state.c = bc_inc as u8;
            state.pc += 1;
        },
        0x04 => {
            // INR B
            let sum: u16 = (state.b as u16) + 1;
            state.b = sum as u8;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
            state.pc += 1;
        },
        0x05 => {
            // DCR B
            let diff: u16 = (state.b as u16) - 1;
            state.b = diff as u8;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
            state.pc += 1;
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
            println!("unimplemented instruction: {}", opcode);
            return;
        },
        0x09 => {
            // DAD B
            let hl: u16 = ((state.h as u16) << 8) | (state.l as u16);
            let bc: u16 = ((state.b as u16) << 8) | (state.c as u16);

            let sum: u32 = (hl as u32) + (bc as u32);

            // h stores the leftmost 8 bits. l stores the rightmost 8 bits.
            // if we cast sum from u16 to u8, then the leftmost 8 bits are dropped.
            state.l = sum as u8;
            state.h = (sum >> 8) as u8;

            state.set_carry_flag_double(sum);
            state.pc += 1;
        },
        0x0a => {
            // LDAX B
            let bc = (state.b as u16) << 8 | state.c as u16;
            let bc_mem = state.get_mem(bc);
            state.a = bc_mem;
            state.pc += 1;
        },
        0x0b => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x0c => {
            // INR C
            let sum: u16 = (state.c as u16) + 1;
            state.c = sum as u8;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
            state.pc += 1;
        },
        0x0d => {
            // DCR C
            let diff: u16 = (state.c as u16) - 1;
            state.c = diff as u8;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
            state.pc += 1;
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
            state.pc += 1;
        },
        0x10 => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x11 => {
            // LXI D,D16
            state.d = byte_3;
            state.e = byte_2;
            state.pc += 2;
        },
        0x12 => {
            // STAX D
            let de: u16 = ((state.d as u16) << 8) | (state.e as u16);
            state.set_mem(de, state.a);
            state.pc += 1;
        },
        0x13 => {
            // INX B
            let de: u16 = ((state.d as u16) << 8) | (state.e as u16);
            let de_inc: u16 = de + 1;

            // shift out the rightmost 8 bits, and truncate to use only the remaining 8 bits.
            state.d = (de_inc >> 8) as u8;

            // truncate the leftmost 8 bits
            state.e = de_inc as u8;
            state.pc += 1;
        },
        0x14 => {
            // INR D
            let sum: u16 = (state.d as u16) + 1;
            state.d = sum as u8;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
            state.pc += 1;
        },
        0x15 => {
            // DCR D
            let diff: u16 = (state.d as u16) - 1;
            state.d = diff as u8;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
            state.pc += 1;
        },
        0x16 => {
            // MVI D,D8
            state.d = byte_2;
            state.pc += 1;
        },
        0x17 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x18 => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x19 => {
            // DAD D
            let hl: u16 = ((state.h as u16) << 8) | (state.l as u16);
            let de: u16 = ((state.d as u16) << 8) | (state.e as u16);

            let sum: u32 = (hl as u32) + (de as u32);

            // h stores the leftmost 8 bits. l stores the rightmost 8 bits.
            // if we cast sum from u16 to u8, then the leftmost 8 bits are dropped.
            state.l = sum as u8;
            state.h = (sum >> 8) as u8;

            state.set_carry_flag_double(sum);

            state.pc += 1;
        },
        0x1a => {
            // LDAX D
            let de = (state.d as u16) << 8 | state.e as u16;
            let de_mem = state.get_mem(de);
            state.a = de_mem;
            state.pc += 1;
        },
        0x1b => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x1c => {
            // INR E
            let sum: u16 = (state.e as u16) + 1;
            state.e = sum as u8;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
            state.pc += 1;
        },
        0x1d => {
            // DCR E
            let diff: u16 = (state.e as u16) - 1;
            state.e = diff as u8;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
            state.pc += 1;
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
            state.pc += 1;
        },
        0x20 => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
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
            state.pc += 1;
        },
        0x23 => {
            // INX H
            let hl: u16 = state.get_hl();
            let hl_inc: u16 = hl + 1;

            // shift out the rightmost 8 bits, and truncate to use only the remaining 8 bits.
            state.h = (hl_inc >> 8) as u8;

            // truncate the leftmost 8 bits
            state.l = hl_inc as u8;
            state.pc += 1;
        },
        0x24 => {
            // INR H
            let sum: u16 = (state.h as u16) + 1;
            state.h = sum as u8;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
            state.pc += 1;
        },
        0x25 => {
            // DCR H
            let diff: u16 = (state.h as u16) - 1;
            state.h = diff as u8;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
            state.pc += 1;
        },
        0x26 => {
            // MVI H,D8
            state.h = byte_2;
            state.pc += 1;
        },
        0x27 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x28 => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x29 => {
            // DAD H
            let mut hl = state.get_hl() as u32;
            hl <<= 1;
            state.set_carry_flag_double(hl);
            state.h = (hl >> 8) as u8;
            state.l = hl as u8;
            state.pc += 1;
        },
        0x2a => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x2b => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x2c => {
            // INR L
            let sum: u16 = (state.l as u16) + 1;
            state.l = sum as u8;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
            state.pc += 1;
        },
        0x2d => {
            // DCR L
            let diff: u16 = (state.l as u16) - 1;
            state.l = diff as u8;
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            // TODO: handle AC cc
            state.cc.p = parity(diff & 0xff);
            state.pc += 1;
        },
        0x2e => {
            // MVI L,D8
            state.l = byte_2;
            state.pc += 1;
        },
        0x2f => {
            // CMA (not)
            state.a = !state.a;
            state.pc += 1;
        },
        0x30 => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x31 => {
            // LXI SP,D16
            state.sp = (byte_3 as u16) | (byte_2 as u16);
            state.pc += 1;
        },
        0x32 => {
            // STA adr
            // required for space invaders
            println!("unimplemented instruction: {}", opcode);
            return;
            //state.set_mem((byte_2 as u16) << 8 | (byte_3 as u16), state.a);
        },
        0x33 => {
            // INX SP
            state.sp += 1;
            state.pc += 1;
        },
        0x34 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x35 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x36 => {
            // MVI M,D8
            let hl = state.get_hl();
            state.set_mem(hl, byte_2);
            state.pc += 1;
        },
        0x37 => {
            // STC
            state.cc.cy = true;
            state.pc += 1;
        },
        0x38 => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x39 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x3a => {
            // LDA adr
            let addr = (byte_2 as u16) << 8 | (byte_3 as u16);
            let mem = state.get_mem(addr);
            state.a = mem;
            state.pc += 1;
        },
        0x3b => {
            // DCX SP
            state.sp -= 1;
            state.pc += 1;
        },
        0x3c => {
            // INR A
            let sum: u16 = (state.a as u16) + 1;
            state.a = sum as u8;
            state.set_zero_flag(sum);
            state.set_sign_flag(sum);
            // TODO: handle AC cc
            state.cc.p = parity(sum & 0xff);
            state.pc += 1;

        },
        0x3d => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x3e => {
            // MVI A,D8
            state.a = byte_2;
            state.pc += 1;
        },
        0x3f => {
            // CMC
            state.cc.cy = !state.cc.cy;
            state.pc += 1;
        },
        0x40 => {
            // MOV B,B
            // TODO: does this actually do anything?
            state.b = state.b;
            state.pc += 1;
        },
        0x41 => {
            // MOV B,C
            state.b = state.c;
            state.pc += 1;
        },
        0x42 => {
            // MOV B,D
            state.b = state.d;
            state.pc += 1;
        },
        0x43 => {
            // MOV B,E
            state.b = state.e;
            state.pc += 1;
        },
        0x44 => {
            // MOV B,B
            state.b = state.h;
            state.pc += 1;
        },
        0x45 => {
            // MOV B,B
            state.b = state.l;
            state.pc += 1;
        },
        0x46 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x47 => {
            // MOV B,A
            state.b = state.a;
            state.pc += 1;
        },
        0x48 => {
            // MOV C,B
            state.c = state.b;
            state.pc += 1;
        },
        0x49 => {
            // MOV C,C
            state.c = state.c;
            state.pc += 1;
        },
        0x4a => {
            // MOV C,D
            state.c = state.d;
            state.pc += 1;
        },
        0x4b => {
            // MOV C,E
            state.c = state.e;
            state.pc += 1;
        },
        0x4c => {
            // MOV C,H
            state.c = state.h;
            state.pc += 1;
        },
        0x4d => {
            // MOV C,L
            state.c = state.l;
            state.pc += 1;
        },
        0x4e => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x4f => {
            // MOV C,A
            state.c = state.a;
            state.pc += 1;
        },
        0x50 => {
            // MOV D,B
            state.d = state.b;
            state.pc += 1;
        },
        0x51 => {
            // MOV D,C
            state.d = state.c;
            state.pc += 1;
        },
        0x52 => {
            // MOV D,D
            state.d = state.d;
            state.pc += 1;
        },
        0x53 => {
            // MOV D,E
            state.d = state.e;
            state.pc += 1;
        },
        0x54 => {
            // MOV D,H
            state.d = state.h;
            state.pc += 1;
        },
        0x55 => {
            // MOV D,L
            state.d = state.l;
            state.pc += 1;
        },
        0x56 => {
            // MOV D,M
            let hl = state.get_hl();
            let m = state.get_mem(hl);
            state.d = m;
            state.pc += 1;
        },
        0x57 => {
            // MOV D,A
            state.d = state.a;
            state.pc += 1;
        },
        0x58 => {
            // MOV E,B
            state.e = state.b;
            state.pc += 1;
        },
        0x59 => {
            // MOV E,C
            state.e = state.c;
            state.pc += 1;
        },
        0x5a => {
            // MOV E,D
            state.e = state.d;
            state.pc += 1;
        },
        0x5b => {
            // MOV E,E
            state.e = state.e;
            state.pc += 1;
        },
        0x5c => {
            // MOV E,H
            state.e = state.h;
            state.pc += 1;
        },
        0x5d => {
            // MOV E,L
            state.e = state.l;
            state.pc += 1;
        },
        0x5e => {
            // MOV E,M
            let hl = state.get_hl();
            let m = state.get_mem(hl);
            state.e = m;
            state.pc += 1;
        },
        0x5f => {
            // MOV E,A
            state.e = state.a;
            state.pc += 1;
        },
        0x60 => {
            // MOV H,B
            state.h = state.b;
            state.pc += 1;
        },
        0x61 => {
            // MOV H,C
            state.h = state.c;
            state.pc += 1;
        },
        0x62 => {
            // MOV H,D
            state.h = state.d;
            state.pc += 1;
        },
        0x63 => {
            // MOV H,E
            state.h = state.e;
            state.pc += 1;
        },
        0x64 => {
            // MOV H,H
            state.h = state.h;
            state.pc += 1;
        },
        0x65 => {
            // MOV H,L
            state.h = state.l;
            state.pc += 1;
        },
        0x66 => {
            // MOV H,M
            let hl = state.get_hl();
            let m = state.get_mem(hl);
            state.h = m;
            state.pc += 1;
        },
        0x67 => {
            // MOV H,A
            state.h = state.a;
            state.pc += 1;
        },
        0x68 => {
            // MOV L,B
            state.l = state.b;
            state.pc += 1;
        },
        0x69 => {
            // MOV L,C
            state.l = state.c;
            state.pc += 1;
        },
        0x6a => {
            // MOV L,D
            state.l = state.d;
            state.pc += 1;
        },
        0x6b => {
            // MOV L,E
            state.l = state.e;
            state.pc += 1;
        },
        0x6c => {
            // MOV L,H
            state.l = state.h;
            state.pc += 1;
        },
        0x6d => {
            // MOV L,L
            state.l = state.l;
            state.pc += 1;
        },
        0x6e => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x6f => {
            // MOV L,A
            state.l = state.a;
            state.pc += 1;
        },
        0x70 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x71 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x72 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x73 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x74 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x75 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x76 => {
            // HLT
            return;
        },
        0x77 => {
            // MOV M,A
            let hl = state.get_hl();
            state.set_mem(hl, state.a);
            state.pc += 1;
        },
        0x78 => {
            // MOV A,B
            state.a = state.b;
            state.pc += 1;
        },
        0x79 => {
            // MOV A,C
            state.a = state.c;
            state.pc += 1;
        },
        0x7a => {
            // MOV A,D
            state.a = state.d;
            state.pc += 1;
        },
        0x7b => {
            // MOV A,E
            state.a = state.e;
            state.pc += 1;
        },
        0x7c => {
            // MOV A,H
            state.a = state.h;
            state.pc += 1;
        },
        0x7d => {
            // MOV A,L
            state.a = state.l;
            state.pc += 1;
        },
        0x7e => {
            // MOV A,M
            let hl = state.get_hl();
            state.set_mem(hl, state.a);
            state.pc += 1;
        },
        0x7f => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
        },
        0x8e => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
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
            state.pc += 1;
        },
        0x92 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
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
            state.pc += 1;
        },
        0x94 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
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
            state.pc += 1;
        },
        0x96 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x97 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x98 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x99 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x9a => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x9b => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x9c => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x9d => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x9e => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0x9f => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xa0 => {
            // ANA B
            let and = (state.a as u16) & (state.b as u16);
            state.set_zero_flag(and);
            state.set_sign_flag(and);
            state.set_carry_flag(and);

            // parity flag
            state.cc.p = parity(and & 0xff);

            // TODO: handle AC cc

            state.a = and as u8;
            state.pc += 1;
        },
        0xa1 => {
            // ANA C
            let and = (state.a as u16) & (state.c as u16);
            state.set_zero_flag(and);
            state.set_sign_flag(and);
            state.set_carry_flag(and);

            // parity flag
            state.cc.p = parity(and & 0xff);

            // TODO: handle AC cc

            state.a = and as u8;
            state.pc += 1;
        },
        0xa2 => {
            // ANA D
            let and = (state.a as u16) & (state.d as u16);
            state.set_zero_flag(and);
            state.set_sign_flag(and);
            state.set_carry_flag(and);

            // parity flag
            state.cc.p = parity(and & 0xff);

            // TODO: handle AC cc

            state.a = and as u8;
            state.pc += 1;
        },
        0xa3 => {
            // ANA E
            let and = (state.a as u16) & (state.e as u16);
            state.set_zero_flag(and);
            state.set_sign_flag(and);
            state.set_carry_flag(and);

            // parity flag
            state.cc.p = parity(and & 0xff);

            // TODO: handle AC cc

            state.a = and as u8;
            state.pc += 1;
        },
        0xa4 => {
            // ANA H
            let and = (state.a as u16) & (state.h as u16);
            state.set_zero_flag(and);
            state.set_sign_flag(and);
            state.set_carry_flag(and);

            // parity flag
            state.cc.p = parity(and & 0xff);

            // TODO: handle AC cc

            state.a = and as u8;
            state.pc += 1;
        },
        0xa5 => {
            // ANA L
            let and = (state.a as u16) & (state.l as u16);
            state.set_zero_flag(and);
            state.set_sign_flag(and);
            state.set_carry_flag(and);

            // parity flag
            state.cc.p = parity(and & 0xff);

            // TODO: handle AC cc

            state.a = and as u8;
            state.pc += 1;
        },
        0xa6 => {
            // ANA M
            let hl = state.get_hl();
            let m = state.get_mem(hl);
            let and = (state.a as u16) & (m as u16);
            state.set_zero_flag(and);
            state.set_sign_flag(and);
            state.set_carry_flag(and);

            // parity flag
            state.cc.p = parity(and & 0xff);

            // TODO: handle AC cc

            state.a = and as u8;
            state.pc += 1;
        },
        0xa7 => {
            // ANA A
            // TODO: optimize this?
            let and = (state.a as u16) & (state.a as u16);
            state.set_zero_flag(and);
            state.set_sign_flag(and);
            state.set_carry_flag(and);

            // parity flag
            state.cc.p = parity(and & 0xff);

            // TODO: handle AC cc

            state.a = and as u8;
            state.pc += 1;
        },
        0xa8 => {
            // XRA B
            // a and b are u8, but we need to capture the carry-out, so we use u16
            let xor: u16 = (state.a as u16) ^ (state.b as u16);

            state.set_zero_flag(xor);
            state.set_sign_flag(xor);
            state.set_carry_flag(xor);

            // parity flag
            state.cc.p = parity(xor & 0xff);

            // TODO: handle AC cc

            state.a = xor as u8;
            state.pc += 1;
        },
        0xa9 => {
            // XRA C
            // a and c are u8, but we need to capture the carry-out, so we use u16
            let xor: u16 = (state.a as u16) ^ (state.c as u16);

            state.set_zero_flag(xor);
            state.set_sign_flag(xor);
            state.set_carry_flag(xor);

            // parity flag
            state.cc.p = parity(xor & 0xff);

            // TODO: handle AC cc

            state.a = xor as u8;
            state.pc += 1;
        },
        0xaa => {
            // XRA D

            let xor: u16 = (state.a as u16) ^ (state.d as u16);

            state.set_zero_flag(xor);
            state.set_sign_flag(xor);
            state.set_carry_flag(xor);

            // parity flag
            state.cc.p = parity(xor & 0xff);

            // TODO: handle AC cc

            state.a = xor as u8;
            state.pc += 1;
        },
        0xab => {
            // XRA E

            let xor: u16 = (state.a as u16) ^ (state.e as u16);

            state.set_zero_flag(xor);
            state.set_sign_flag(xor);
            state.set_carry_flag(xor);

            // parity flag
            state.cc.p = parity(xor & 0xff);

            // TODO: handle AC cc

            state.a = xor as u8;
            state.pc += 1;
        },
        0xac => {
            // XRA H

            let xor: u16 = (state.a as u16) ^ (state.h as u16);

            state.set_zero_flag(xor);
            state.set_sign_flag(xor);
            state.set_carry_flag(xor);

            // parity flag
            state.cc.p = parity(xor & 0xff);

            // TODO: handle AC cc

            state.a = xor as u8;
            state.pc += 1;
        },
        0xad => {
            // XRA L

            let xor: u16 = (state.a as u16) ^ (state.l as u16);

            state.set_zero_flag(xor);
            state.set_sign_flag(xor);
            state.set_carry_flag(xor);

            // parity flag
            state.cc.p = parity(xor & 0xff);

            // TODO: handle AC cc

            state.a = xor as u8;
            state.pc += 1;
        },
        0xae => {
            // XRA M
            let hl: u16 = state.get_hl();
            let m: u8 = state.get_mem(hl);

            let xor: u16 = (state.a as u16) ^ (m as u16);

            state.set_zero_flag(xor);
            state.set_sign_flag(xor);
            state.set_carry_flag(xor);

            // parity flag
            state.cc.p = parity(xor & 0xff);

            // TODO: handle AC cc

            state.a = xor as u8;
            state.pc += 1;
        },
        0xaf => {
            // XRA A
            // TODO: can we just optimize this to set a to 0, and set flags?
            let xor: u16 = (state.a as u16) ^ (state.a as u16);

            state.set_zero_flag(xor);
            state.set_sign_flag(xor);
            state.set_carry_flag(xor);

            // parity flag
            state.cc.p = parity(xor & 0xff);

            // TODO: handle AC cc

            state.a = xor as u8;
            state.pc += 1;
        },
        0xb0 => {
            // ORA B
            let or: u16 = (state.a as u16) | (state.b as u16);
            state.set_zero_flag(or);
            state.set_sign_flag(or);
            state.set_carry_flag(or);

            // parity flag
            state.cc.p = parity(or & 0xff);

            // TODO: handle AC cc

            state.a = or as u8;
            state.pc += 1;
        },
        0xb1 => {
            // ORA C
            let or: u16 = (state.a as u16) | (state.c as u16);
            state.set_zero_flag(or);
            state.set_sign_flag(or);
            state.set_carry_flag(or);

            // parity flag
            state.cc.p = parity(or & 0xff);

            // TODO: handle AC cc

            state.a = or as u8;
            state.pc += 1;
        },
        0xb2 => {
            // ORA D
            let or: u16 = (state.a as u16) | (state.d as u16);
            state.set_zero_flag(or);
            state.set_sign_flag(or);
            state.set_carry_flag(or);

            // parity flag
            state.cc.p = parity(or & 0xff);

            // TODO: handle AC cc

            state.a = or as u8;
            state.pc += 1;
        },
        0xb3 => {
            // ORA E
            let or: u16 = (state.a as u16) | (state.e as u16);
            state.set_zero_flag(or);
            state.set_sign_flag(or);
            state.set_carry_flag(or);

            // parity flag
            state.cc.p = parity(or & 0xff);

            // TODO: handle AC cc

            state.a = or as u8;
            state.pc += 1;
        },
        0xb4 => {
            // ORA H
            let or: u16 = (state.a as u16) | (state.h as u16);
            state.set_zero_flag(or);
            state.set_sign_flag(or);
            state.set_carry_flag(or);

            // parity flag
            state.cc.p = parity(or & 0xff);

            // TODO: handle AC cc

            state.a = or as u8;
            state.pc += 1;
        },
        0xb5 => {
            // ORA L
            let or: u16 = (state.a as u16) | (state.l as u16);
            state.set_zero_flag(or);
            state.set_sign_flag(or);
            state.set_carry_flag(or);

            // parity flag
            state.cc.p = parity(or & 0xff);

            // TODO: handle AC cc

            state.a = or as u8;
            state.pc += 1;
        },
        0xb6 => {
            // ORA M
            let hl = state.get_hl();
            let m = state.get_mem(hl);

            let or: u16 = (state.a as u16) | (m as u16);
            state.set_zero_flag(or);
            state.set_sign_flag(or);
            state.set_carry_flag(or);

            // parity flag
            state.cc.p = parity(or & 0xff);

            // TODO: handle AC cc

            state.a = or as u8;
            state.pc += 1;
        },
        0xb7 => {
            // ORA A
            // TODO: can this be optimized to just not modify a and set the flags?
            let or: u16 = (state.a as u16) | (state.a as u16);
            state.set_zero_flag(or);
            state.set_sign_flag(or);
            state.set_carry_flag(or);

            // parity flag
            state.cc.p = parity(or & 0xff);

            // TODO: handle AC cc

            state.a = or as u8;
            state.pc += 1;
        },
        0xb8 => {
            // CMP B
            let diff: u16 = (state.a as u16) - (state.b as u16);
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            state.set_carry_flag(diff);

            // parity flag
            state.cc.p = parity(diff & 0xff);

            // TODO: handle AC cc
            state.pc += 1;
        },
        0xb9 => {
            // CMP C
            let diff: u16 = (state.a as u16) - (state.c as u16);
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            state.set_carry_flag(diff);

            // parity flag
            state.cc.p = parity(diff & 0xff);

            // TODO: handle AC cc
            state.pc += 1;
        },
        0xba => {
            // CMP D
            let diff: u16 = (state.a as u16) - (state.d as u16);
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            state.set_carry_flag(diff);

            // parity flag
            state.cc.p = parity(diff & 0xff);

            // TODO: handle AC cc
            state.pc += 1;
        },
        0xbb => {
            // CMP E
            let diff: u16 = (state.a as u16) - (state.e as u16);
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            state.set_carry_flag(diff);

            // parity flag
            state.cc.p = parity(diff & 0xff);

            // TODO: handle AC cc
            state.pc += 1;
        },
        0xbc => {
            // CMP H
            let diff: u16 = (state.a as u16) - (state.h as u16);
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            state.set_carry_flag(diff);

            // parity flag
            state.cc.p = parity(diff & 0xff);

            // TODO: handle AC cc
            state.pc += 1;
        },
        0xbd => {
            // CMP L
            let diff: u16 = (state.a as u16) - (state.l as u16);
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            state.set_carry_flag(diff);

            // parity flag
            state.cc.p = parity(diff & 0xff);

            // TODO: handle AC cc
            state.pc += 1;
        },
        0xbe => {
            // CMP M
            let hl = state.get_hl();
            let m = state.get_mem(hl);

            let diff: u16 = (state.a as u16) - (m as u16);
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            state.set_carry_flag(diff);

            // parity flag
            state.cc.p = parity(diff & 0xff);

            // TODO: handle AC cc
            state.pc += 1;
        },
        0xbf => {
            // CMP A
            // TODO: optimize this?
            let diff: u16 = (state.a as u16) - (state.a as u16);
            state.set_zero_flag(diff);
            state.set_sign_flag(diff);
            state.set_carry_flag(diff);

            // parity flag
            state.cc.p = parity(diff & 0xff);

            // TODO: handle AC cc
            state.pc += 1;
        },
        0xc0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xc1 => {
            // POP B
            state.c = state.get_mem(state.sp);
            state.b = state.get_mem(state.sp + 1);
            state.sp += 2;
            state.pc += 1;
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
            state.pc += 1;
        },
        0xc5 => {
            // PUSH B
            state.set_mem(state.sp - 1, state.b);
            state.set_mem(state.sp - 2, state.c);
            state.sp -= 2;
            state.pc += 1;
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
            state.pc += 1;
        },
        0xc7 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xc8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xc9 => {
            // RET
            state.pc = state.get_mem(state.pc) as u16 | (state.get_mem(state.sp + 1) as u16) << 8;
            state.sp += 2;
        },
        0xca => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xcb => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xcc => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xcd => {
            // CALL address

            let ret: u16 = state.pc + 2;

            state.set_mem(state.sp - 1, (ret >> 8) as u8 & 0xff);
            state.set_mem(state.sp - 2, ret as u8 & 0xff);
            state.sp = state.sp - 2;
            state.pc = (byte_3 as u16) << 8 | byte_2 as u16;
        },
        0xce => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xcf => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xd0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xd1 => {
            // POP D
            state.e = state.get_mem(state.sp);
            state.d = state.get_mem(state.sp + 1);
            state.sp += 2;
            state.pc += 1;
        },
        0xd2 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xd3 => {
            // TODO: necessary for space invaders
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xd4 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xd5 => {
            // PUSH D
            state.set_mem(state.sp - 1, state.d);
            state.set_mem(state.sp - 2, state.e);
            state.sp -= 2;
            state.pc += 1;
        },
        0xd6 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xd7 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xd8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xd9 => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xda => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xdb => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xdc => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xdd => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xde => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xdf => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xe0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xe1 => {
            // POP H
            state.l = state.get_mem(state.sp);
            state.h = state.get_mem(state.sp + 1);
            state.sp += 2;
            state.pc += 1;
        },
        0xe2 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xe3 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xe4 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xe5 => {
            // PUSH H
            state.set_mem(state.sp - 2, state.l);
            state.set_mem(state.sp - 1, state.h);
            state.sp -= 2;
            state.pc += 1;
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
            state.pc += 1;
        },
        0xe8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xe9 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xea => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xeb => {
            // XCHG
            let temp = state.h;
            state.h = state.d;
            state.d = temp;

            let temp = state.l;
            state.l = state.e;
            state.e = temp;
            state.pc += 1;
        },
        0xec => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xed => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xee => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xef => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xf0 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xf1 => {
            // POP PSW
            state.a = state.get_mem(state.sp + 1);
            let psw: u8 = state.get_mem(state.sp);
            state.cc.z = 0x01 == psw & 0x01;
            state.cc.s = 0x02 == psw & 0x02;
            state.cc.p = 0x04 == psw & 0x04;
            state.cc.cy = 0x05 == psw & 0x08;
            state.cc.ac = 0x10 == psw & 0x10;
            state.sp += 2;
            state.pc += 1;
        },
        0xf2 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xf3 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xf4 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xf5 => {
            // PUSH PSW
            state.set_mem(state.sp - 1, state.a);
            let psw: u8 = state.cc.z as u8 |
                          (state.cc.s as u8) << 1 |
                          (state.cc.p as u8) << 2 |
                          (state.cc.cy as u8) << 3 |
                          (state.cc.ac as u8) << 4;
            state.set_mem(state.sp - 2, psw);
            state.sp -= 2;
            state.pc += 1;
        },
        0xf6 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xf7 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xf8 => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xf9 => {
            // SPHL
            state.sp = state.get_hl();
            state.pc += 1;
        },
        0xfa => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xfb => {
            // TODO: necessary for space invaders
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xfc => {
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
        },
        0xfd => {
            // -
            println!("unimplemented instruction: {}", opcode);
            return;
            state.pc += 1;
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
            state.pc += 1;
        },
    }
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
