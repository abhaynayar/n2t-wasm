use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern { fn alert(s: &str); }

#[wasm_bindgen]
pub fn greet() { alert("Hello, n2t-wasm!"); }

// ------------------------------------------------------------------------

#[wasm_bindgen(raw_module="../www/index.js")]
extern "C" { fn put_xy(addr: u16, value: u16); }

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct Emu {
    // Registers:
    pc: u16, ra: u16,
    rd: u16, rm: u16,

    // Memory:
    rom: [u16; 0x8000],
    ram: [u16; 0x6000+1],

    // ALU:
    x: u16, y: u16,
    zr: bool, ng: bool,
}

#[wasm_bindgen]
impl Emu {
    pub fn new() -> Emu {
        Emu {
            // Registers:
            pc: 0, ra: 0,
            rd: 0, rm: 0,
            
            // Memory:
            rom: [0; 0x8000],
            ram: [0; 0x6000+1],

            // ALU:
            x: 0, y: 0,
            zr: false, ng: false,
        }
    }

    pub fn alu(&mut self, comp:u16) -> u16 {
        // Input:  x,y, { zx,nx,zy,ny,f,no }
        // Output: out, { zr,ng } -> wasm doesn't allow returning this

        // Extract comp bits:
        let zx = (comp>>5)&1;
        let nx = (comp>>4)&1;
        let zy = (comp>>3)&1;
        let ny = (comp>>2)&1;
        let f  = (comp>>1)&1;
        let no = (comp>>0)&1;

        // Compute output:
        if zx == 1 { self.x = 0 }
        if nx == 1 { self.x = !self.x }
        if zy == 1 { self.y = 0 }
        if ny == 1 { self.y = !self.y }
        let mut out = if f==1 {
            self.x + self.y
        } else {
            self.x & self.y
        };
        if no == 1 { out = !out }

        // Flags:
        self.zr = out==0;
        self.ng = (out>>15)&1 == 1;

        return out;
    }
    
    pub fn tick(&mut self) {

        if self.ra <= 0x6000+1 {
            self.rm = self.ram[self.ra as usize];
        }
        
        let inst = self.rom[self.pc as usize];

        //log(&format!("ra: {}, rd: {}, rm: {}, inst: {}",
        //    self.ra,
        //    self.rd,
        //    self.rm,
        //    inst,
        //    //disassemble(inst)
        //));

        // A Instructions:
        if inst >> 15 == 0 {
            self.ra = inst;
            self.pc += 1;
        }

        // C Instructions:
        // 111ACCCCCCDDDJJJ
        else {
            let a_bit = (inst>>12) & 1;
            let comp = (inst & 0x1fc0) >> 6;
            let dest = (inst & 0x0038) >> 3;
            let jump = (inst & 0x0007) >> 0;

            // ALU takes D and M/A as input.
            // First input "x": D.
            self.x = self.rd;
            
            // 12th bit is a control bit for second input "y": M/A.
            // If it is 1 then we send M to the ALU, else we send A.
            if a_bit == 1 {
                self.y = self.ram[self.ra as usize];
            } else {
                self.y = self.ra;
            };

            // Get the ALU to compute.
            let alu_res = self.alu(comp);

            // ALU output goes into registers decided by dest bits.
            if (dest>>0)&1 == 1 { self.store_ram(self.ra, alu_res); }
            if (dest>>1)&1 == 1 { self.rd = alu_res; }
            if (dest>>2)&1 == 1 { self.ra = alu_res; }

            // First, determine if we are going to jump:
            let jump_res = match jump {
                0x00 => false,
                0x01 => !(self.zr || self.ng),  // JGT
                0x02 => self.zr,                // JEQ
                0x03 => !self.ng,               // JGE
                0x04 => self.ng,                // JLT
                0x05 => !self.zr,               // JNE
                0x06 => self.ng || self.zr,     // JLE
                0x07 => true,                   // JMP
                   _ => true,
            };

            // Then, jump (or increment):
            self.pc = if jump_res {
                self.ra
            } else {
                self.pc+1
            };
        }
    }

    pub fn load_rom(&mut self, code: &str) {
        let mut line_counter = 0;
        for line in code.lines() {
            let mut opcode: u16 = 0;
            for (i,c) in line.chars().enumerate() {
                let current_bit = c as u16 - '0' as u16;
                opcode |= current_bit << (15-i);
            }
            self.rom[line_counter] = opcode as u16;
            line_counter += 1;
        }
    }
    
    pub fn store_ram(&mut self, address: u16, value: u16) {
        self.ram[address as usize] = value;
        // Only update the canvas for addresses in screen memory
        if address>=0x4000 && address<0x6000 {
            put_xy(address,value);
        }
    }

    pub fn run(&mut self) {
        //self.store_ram(0,100);
        for _i in 0..10_000 {
            self.tick();
        }
    }
}

pub fn disassemble(opcode: u16) -> String {
    let mut res = String::new();
    if ((opcode >> 15) & 1) == 1 {
        let comp = (opcode & 0x1fc0) >> 6;
        let dest = (opcode & 0x0038) >> 3;
        let jump = (opcode & 0x0003) >> 0;
        let comp_str = match comp {
            0x2a => "0",
            0x3f => "1",
            0x3a => "-1",
            0x0c => "D",
            0x30 => "A",
            0x0d => "!D",
            0x31 => "!A",
            0x0f => "-D",
            0x33 => "-A",
            0x1f => "D+1",
            0x37 => "A+1",
            0x0e => "D-1",
            0x32 => "A-1",
            0x02 => "D+A",
            0x23 => "D-A",
            0x07 => "A-D",
            0x00 => "D&A",
            0x15 => "D|A",
            0x70 => "M",
            0x71 => "!M",
            0x73 => "-M",
            0x77 => "M+1",
            0x72 => "M-1",
            0x42 => "D+M",
            0x53 => "D-M",
            0x47 => "M-D",
            0x40 => "D&M",
            0x55 => "D|M",
            _ => "?"
        };
        let dest_str = match dest {
            0x00 => "",
            0x01 => "M",
            0x02 => "D",
            0x03 => "MD",
            0x04 => "A",
            0x05 => "AM",
            0x06 => "AD",
            0x07 => "AMD",
            _ => "?"
        };
        let jump_str = match jump {
             0x00 => "",
             0x01 => "JGT",
             0x02 => "JEQ",
             0x03 => "JGE",
             0x04 => "JLT",
             0x05 => "JNE",
             0x06 => "JLE",
             0x07 => "JMP",
            _ => "?",
        };
        
        res.push_str(dest_str);
        res.push_str("=");
        res.push_str(comp_str);
        res.push_str(";");
        res.push_str(jump_str);
    } else {
        res.push_str("@");
        res.push_str(&(opcode&0x7fff).to_string());
    }
    
    res
}
