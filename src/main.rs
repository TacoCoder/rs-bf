extern crate libc;

use std::io::{ Read, BufRead, Write, stdout, stdin };
use std::fs::File;
use std::env;

struct Interpreter {
    stack: Vec<i8>,
    stack_ptr: usize
}

impl Interpreter {
    fn new(stack_size: usize, stack_value: i8) -> Interpreter {
        let mut interp = Interpreter { stack: Vec::with_capacity(stack_size), stack_ptr: 0 };
        for i in 0..stack_size {
            interp.stack.push(stack_value);
        }
        interp
    }

    fn interp(&mut self, source: &str) {
        let char_vec: Vec<char> = source.chars().collect();

        let mut i = 0;
        'mainloop: while i < char_vec.len() {
            let c = char_vec[i];
            match c {
                '>' => {
                    self.stack_ptr += 1;
                },
                '<' => {
                    self.stack_ptr -= 1;
                },
                '+' => {
                    self.stack[self.stack_ptr] = self.stack[self.stack_ptr].wrapping_add(1);
                },
                '-' => {
                    self.stack[self.stack_ptr] = self.stack[self.stack_ptr].wrapping_sub(1);
                },
                '.' => {
                    print!("{}", (self.stack[self.stack_ptr] as u8) as char);
                    stdout().flush();
                },
                ',' => {
                    let mut ic = 0;
                    unsafe { ic = libc::getchar(); }
                    self.stack[self.stack_ptr] = ic as i8;
                },
                '[' => {
                    if self.stack[self.stack_ptr] == 0 {
                        let mut insides = 0;
                        loop { 
                            i+=1;
                            if char_vec[i] == ']' && insides == 0 {
                                i+=1;
                                continue 'mainloop;
                            }

                            if char_vec[i] == ']' && insides != 0 {
                                insides -= 1;
                            }

                            if char_vec[i] == '[' { insides += 1; };
                        }
                    }
                },
                ']' => {
                    if self.stack[self.stack_ptr] != 0 {
                        let mut insides = 0;
                        loop {
                            i-=1;
                            if char_vec[i] == '[' && insides == 0 {
                                i+=1;
                                continue 'mainloop;
                            }

                            if char_vec[i] == '[' && insides != 0 {
                                insides -= 1;
                            }

                            if char_vec[i] == ']' { insides += 1; }
                        }
                    }
                },
                _   => {
                
                }
            }
            i+=1;

        }
    }
}


// Run without args to use stdin
// First arg is filename to interpret
// May or may not run on windows, not sure how libc acts on there, will make it independent of libc soon
// Next goal is to add a compiler (that will compile to ASM, which can be compiled into an executable)
fn main() {
    let args: Vec<_> = env::args().collect();

    let mut interp = Interpreter::new(1000, 0);
    
    if args.len() > 1 {
        let mut f = match File::open(&args[1]) {
            Ok(ok) => ok,
            Err(_) => panic!("Unable to find file: {}", &args[1]),
        };

        let mut file_contents = String::new();
        match f.read_to_string(&mut file_contents) {
            Ok(_) => (),
            Err(_) => panic!("Found file but unable to read contents"),
        }

        interp.interp(&file_contents[..]);
    }else{
        let stdin = stdin();
        for line in stdin.lock().lines() {
            interp.interp(&line.unwrap()[..]);
        }
    }
}
