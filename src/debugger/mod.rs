mod command;

use n64::*;

use self::command::*;

use std::io::*;

pub struct Debugger {
    n64: N64
}

impl Debugger {
    pub fn new(n64: N64) -> Debugger {
        Debugger {
            n64: n64
        }
    }

    pub fn run(&mut self) {
        loop {
            print!("r64> ");
            stdout().flush().unwrap();

            let command = read_stdin().parse();
            match command {
                Ok(Command::Step) => self.step(),
                Ok(Command::Exit) => break,
                Err(_) => println!("Invalid input")
            }
        }
    }

    pub fn step(&mut self) {
        println!("{:018X}", self.n64.cpu().reg_pc());

        /*match instr.opcode() {
            Special => print!("Special: {:?}", instr.special_op()),
            RegImm => print!("RegImm: {:?}", instr.reg_imm_op()),
            _ => print!("{:?}", instr)
        }
        match delay_slot {
            DelaySlot::Yes => println!(" (DELAY)"),
            _ => println!("")
        };*/

        self.n64.step();
    }
}

fn read_stdin() -> String {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    input.trim().into()
}
