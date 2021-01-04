use wasm_bindgen::prelude::*;

// ----------------------------- Emulator ------------------------------- //

#[wasm_bindgen]
pub struct Emu {
    // Registers
    pc: u16, ra: u16,
    rd: u16, rm: u16,

    // Memory
    rom: [u16; 0x8000],
    ram: [u16; 0x8000],

    // Debug
    pause: bool,
}

#[wasm_bindgen]
impl Emu {
    pub fn new() -> Emu {
        Emu {
            // Registers
            pc: 0, ra: 0,
            rd: 0, rm: 0,

            // Memory
            rom: [0; 0x8000],
            ram: [0; 0x8000],

            // Debug
            pause: false,
        }
    }

    pub fn pause(&mut self) {
        self.pause = true;
    }

    pub fn continue_execution(&mut self) {
        for _i in 0..1000 {
            if self.pause == true { return; }
            self.tick();
        }
    }

    pub fn reset(&mut self) {
        for i in 0..0x8000 {
            self.store_ram(i,0);
            self.rom[0] = 0;
        }

        self.pc = 0;
        self.ra = 0;
        self.rd = 0;
        self.rm = 0;
    }

    pub fn load_rom(&mut self, code: &str) {
       let mut line_counter = 0;
        for line in code.lines() {
            let mut opcode: u16 = 0;
            for (i,c) in line.chars().enumerate() {
                let current_bit = c as u16 - '0' as u16;
                opcode |= current_bit << (15-i);
            }
            self.rom[line_counter] = opcode;
            line_counter += 1;
        }
    }

    pub fn load_ram(&mut self, addr: u16) -> u16 {
        self.ram[addr as usize]
    }

    pub fn store_ram(&mut self, addr: u16, val: u16) {
        if addr < 0x8000 {
            self.ram[addr as usize] = val;
            // TODO(abhay): Send words instead of pixels.
            /*
            if addr>=0x4000 {
                let row = (addr-0x4000)/32;
                let col = (addr-0x4000)%32;
                for i in 0..16 {
                    let set = (val>>(15-i)) & 1;
                    put_xy(col+i, row, set);
                }
            }
            */
        }
    }
    
    pub fn draw(&mut self, frame: &mut [u8]) {
        
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let is_pixel_set = 
                (self.load_ram(0x4000 + (i/16) as u16) >> (i%16)) & 1;
            
            let rgba = if is_pixel_set != 0 {
                [0x00, 0x00, 0x00, 0xff]
            } else {
                [0xff, 0xff, 0xff, 0xff]
            };
    
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn tick(&mut self) {
        self.rm = self.ram[self.ra as usize];
        let inst = self.rom[self.pc as usize];
        if inst >> 15 == 1 {

            // C Instructions (dest=comp;jump)
            let comp = (inst & 0x1fc0) >> 6;
            let dest = (inst & 0x0038) >> 3;
            let jump = (inst & 0x0007) >> 0;
            let comp_res: u16 = match comp {

                /*  0  */ 0x2a => 0,
                /*  1  */ 0x3f => 1,
                /* -1  */ 0x3a => 0xffff, //-(1 as i16) as u16,
                /*  D  */ 0x0c => self.rd,
                /*  A  */ 0x30 => self.ra,
                /* !D  */ 0x0d => !self.rd,
                /* !A  */ 0x31 => !self.ra,
                /* -D  */ 0x0f => -(self.rd as i16) as u16,
                /*  A  */ 0x33 => self.ra,
                /* D+1 */ 0x1f => (self.rd as i16).wrapping_add(1) as u16,
                /* A+1 */ 0x37 => (self.ra as i16).wrapping_add(1) as u16,
                /* D-1 */ 0x0e => (self.rd as i16).wrapping_sub(1) as u16,
                /* A-1 */ 0x32 => (self.ra as i16).wrapping_sub(1) as u16,
                /* D+A */ 0x02 => ((self.rd as i16).wrapping_add(self.ra as i16)) as u16,
                /* D-A */ 0x23 => ((self.rd as i16).wrapping_sub(self.ra as i16)) as u16,
                /* A-D */ 0x07 => ((self.ra as i16).wrapping_sub(self.rd as i16)) as u16,
                /* D&A */ 0x00 => self.rd & self.ra,
                /* D|A */ 0x15 => self.rd | self.ra,
                /*  M  */ 0x70 => self.rm,
                /* !M  */ 0x71 => !self.rm,
                /* -M  */ 0x73 => -(self.rm as i16) as u16,
                /* M+1 */ 0x77 => (self.rm as i16).wrapping_add(1) as u16,
                /* M-1 */ 0x72 => (self.rm as i16).wrapping_sub(1) as u16,
                /* D+M */ 0x42 => ((self.rd as i16).wrapping_add(self.rm as i16)) as u16,
                /* D-M */ 0x53 => ((self.rd as i16).wrapping_sub(self.rm as i16)) as u16,
                /* M-D */ 0x47 => ((self.rm as i16).wrapping_sub(self.rd as i16)) as u16,
                /* D&M */ 0x40 => self.rd & self.rm,
                /* D|M */ 0x55 => self.rd | self.rm,
                _ => {1337}
            };

            // NOTE:
            // The order of statements below matter. DON'T change them.
            // For example: In AM=M+1, if you do A=M+1 before M=M+1,
            // M=M+1 will use M updated by A=M+1, because M depends on A.
            
            match dest {
                /*     */ 0x00 => {},
                /* A   */ 0x01 => {
                /*     */     self.store_ram(self.ra, comp_res);
                /*     */ },
                /* D   */ 0x02 => {
                /*     */     self.rd = comp_res;
                /*     */ },
                /* MD  */ 0x03 => {
                /*     */     self.store_ram(self.ra, comp_res);
                /*     */     self.rd = comp_res;
                /*     */ },
                /* A   */ 0x04 => {
                /*     */     self.ra = comp_res;
                /*     */ },
                /* AM  */ 0x05 => {
                /*     */     self.store_ram(self.ra, comp_res);
                /*     */     self.ra = comp_res;
                /*     */ },
                /* AD  */ 0x06 => {
                /*     */     self.ra = comp_res;
                /*     */     self.rd = comp_res;
                /*     */ },
                /* AMD */ 0x07 => {
                /*     */     self.store_ram(self.ra, comp_res);
                /*     */     self.ra = comp_res;
                /*     */     self.rd = comp_res;
                /*     */ },
                /*     */ _ => {}
            };

            let jump_res = match jump {
                /* INC */ 0x00 => false, // pc += 1
                /* JGT */ 0x01 => (comp_res as i16) > 0,
                /* JEQ */ 0x02 => (comp_res as i16) == 0,
                /* JGE */ 0x03 => (comp_res as i16) >= 0,
                /* JLT */ 0x04 => (comp_res as i16) < 0,
                /* JNE */ 0x05 => (comp_res as i16) != 0,
                /* JLE */ 0x06 => (comp_res as i16) <= 0,
                /* JMP */ 0x07 => true, // Unconditional
                _ => {false}
            };

            if jump_res == true {
                self.pc = self.ra-1;
            }
        } else {
            // A Instructions
            self.ra = inst & 0x7fff;
        }

        self.pc += 1;
        //println!("Inst: {:#06X}; PC: {:#06X}; A: {:#06X}; D: {:#06X}", self.rom[(self.pc-1) as usize], self.pc, self.ra, self.rd);

// ---------------------- WASM Debug output ----------------------------- //
        
        /*
        put_op(&format!("{}: {}", self.pc,
                disassemble(self.rom[self.pc as usize])));
        
        put_regs(&format!("PC: {} <br> A: {} <br> M: {} <br> D: {}",
                self.pc, self.ra, self.rm, self.rd));
        
        log(&format!("Inst: {:#06X}; PC: {:#06X}; A: {:#06X}; D: {:#06X}",
            self.rom[(self.pc-1) as usize], self.pc, self.ra, self.rd));
        */

    }

// ------------------ Trying to fix render loop ------------------------- //
    
    /*
    // TODO(abhay): How to get this to run self.tick()?
    pub fn zelda_run(&self) -> Result<(), JsValue> {
        let f = Rc::new(RefCell::new(None));
        let g = f.clone();
        let mut i = 0;

        *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
            if i > 300 {
                body().set_text_content(Some("All done!"));
                let _ = f.borrow_mut().take();
                return;
            }
            
            i += 1;
            let text = format!("requestAnimationFrame called {} times.", i);
            body().set_text_content(Some(&text));
            request_animation_frame(f.borrow().as_ref().unwrap());

        }) as Box<dyn FnMut()>));
        request_animation_frame(g.borrow().as_ref().unwrap());
        Ok(())
    }
    */
}

/*

use std::rc::Rc;
use std::cell::RefCell;
use wasm_bindgen::JsCast;

// ----------------------------- WebSys --------------------------------- //

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn body() -> web_sys::HtmlElement {
    document().body().expect("document should have a body")
}

// ---------------------------- Externs --------------------------------- //

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(raw_module="../www/index.js")]
extern "C" {
    fn put_xy(x: u16, y: u16, set: u16);
    fn put_op(x: &str);
    fn put_regs(x: &str);
}

// ---------------------------- Disassembler ---------------------------- //

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
            0x33 => "A",
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
    }

    else {
        res.push_str("@");
        res.push_str(&(opcode&0x7fff).to_string());
    }
    res
}

*/
