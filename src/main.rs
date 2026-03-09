use std::fs::File;
use std::io::Read;
use std::path::Display;
use rand::Rng;

pub struct Chip8{
    memory:[u8;4096], //4KB memory
    registers:[u8;16], //V0 - VF registers
    i:u16,
    pc:u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer:u8,
    sound_timer: u8,    
    display: [[u8;64]; 32],
    keypad: [bool;16],
}

impl Chip8{
    pub fn new() -> Self{
        Self{
            memory:[0;4096],
            registers:[0;16],
            i:0,
            pc:0x200,
            stack:[0;16],
            sp:0,
            delay_timer:0,
            sound_timer:0,
            display:[[0;64]; 32],
            keypad:[false;16],
        }
    }
    
    pub fn cycle(&mut self){
        let opcode = self.fetch();
        self.execute(opcode);
    }
    
    fn fetch(&mut self) ->u16{
        let high_byte = self.memory[self.pc as usize]as u16;
        let low_byte = self.memory[(self.pc + 1) as usize] as u16;
        
        let opcode = (high_byte << 8) | low_byte;
        self.pc +=2;
        
        opcode
    }
    
    fn execute(&mut self,opcode: u16){
        let x = ((opcode & 0x0F00) >> 8)as usize;
        let y = ((opcode & 0x00F0) >> 4)as usize;
        let nnn = opcode & 0x0FFF;
        let nn = (opcode & 0x00FF)as u8;
        let n = (opcode & 0x000F)as u8 ;
        
        match opcode & 0xF000 {
            0x0000 => match opcode {
                0x00E0 => self.display = [[0;64];32],
                0x00EE => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => (),
            },
            0x1000 => self.pc = nnn,
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }
            0x3000 => {
                if self.registers[x] == nn{
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.registers[x] != nn {
                    self.pc += 2;
                }                
            }
            0x6000 => self.registers[x] = nn,
            0x7000 => {
                self.registers[x] = self.registers[x].wrapping_add(nn);
            }
            
            //register math operation
            0x8000 => match opcode & 0x000F
            {
                //0 - 4 add operations
                0x0000 => self.registers[x] = self.registers[y],
                0x0001 => self.registers[x] |=  self.registers[y],
                0x0002 => self.registers[x] &= self.registers[y],
                0x0003 => self.registers[x] ^= self.registers[y],
                0x0004 => {             
                    //add with carry
                    let (val, overflow) = self.registers[x].overflowing_add(self.registers[y]);
                    self.registers[x] = val;                    
                    self.registers[0xF] = if overflow {1} else {0};
                }
                //5 & 7 sub operations
                0x0005 => {
                    let(val,borrow) = self.registers[x].overflowing_sub(self.registers[y]);
                    self.registers[x] = val;
                    self.registers[0xF] = if !borrow {1} else{0};
                }
                0x0007 => {
                    let (val,borrow) = self.registers[y].overflowing_sub(self.registers[x]);
                    self.registers[x] = val;
                    self.registers[0xF] = if !borrow {1} else {0};
                }
                0x0006 => {
                    //8XY6 -Right shift by 1 bit
                    self.registers[0xF] = self.registers[x] & 0x01;
                    self.registers[x] >>= 1;
                }
                0x000E => {
                    self.registers[0xF] =( self.registers[x] & 0x80) >> 7;
                    self.registers[x] <<= 1;
                }
                
                _ => println!("Sub-opcode not implemented"),
            }
            0x9000 => {
                if self.registers[x] != self.registers[y] 
                {
                    self.pc += 2;
                }
            }
            0xA000 => {
                self.i = nnn;
            }
            0xB000 => {
                self.pc = nnn + self.registers[0] as u16;
            }
            0xC000 => {
                let random_byte : u8 = rand::thread_rng().gen();
                self.registers[x] = random_byte & nn;
            }
            0xD000 => {
                let x = self.registers[x] as usize % 64;
                let y = self.registers[y] as usize % 32;
                let height = n as usize;
                
                self.registers[0x0F] = 0;
                
                for row in 0..height {
                    let sprite_byte = self.memory[(self.i + row as u16)as usize];
                    for col in 0..8 {
                        let sprite_pixel = (sprite_byte >> (7-col )) & 0x01;
                        let screen_x = (x + col) % 64;
                        let screen_y = (y + row) % 32;
                        
                        if sprite_pixel == 1{
                            if self.display[screen_y][screen_x] == 1{
                                self.registers[0x0F] = 1;
                            }
                            self.display[screen_y][screen_x] ^= 1;
                        }
                    }
                }
            }
            
            _ => println!("Opcode {:04X} not implemented yet",opcode),
            
            
        }
        
        
    }
    
   
    
}

fn main(){
    let chip8 = Chip8::new();
}

