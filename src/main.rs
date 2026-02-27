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
    
    fn execute(&mut self,opcode:u16){
        match opcode & 0x
    }
    
   
    
}

fn main(){
    let chip8 = Chip8::new();
}

