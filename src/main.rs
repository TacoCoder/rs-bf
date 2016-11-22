extern crate libc;

use std::io::{ Read, BufRead, Write, stdout, stdin };
use std::fs::File;
use std::env;

struct Interpreter {
    stack: Vec<u8>,
    stack_ptr: usize
}

impl Interpreter {
    fn new(stack_size: usize, stack_value: u8) -> Interpreter {
        Interpreter { stack: vec![stack_value; stack_size], stack_ptr: 0 }
    }

    fn interp(&mut self, source: String) {
        let mut char_vec: Vec<u8> = Vec::new();
        (source.as_ref() as &[u8]).read_to_end(&mut char_vec);

        let mut i = 0;
        'mainloop: while i < char_vec.len() {
            let c = char_vec[i];
            match c {
                b'>' => {
                    self.stack_ptr = self.stack_ptr.wrapping_add(1);
                },
                b'<' => {
                    self.stack_ptr = self.stack_ptr.wrapping_sub(1);
                },
                b'+' => {
                    self.stack[self.stack_ptr] = self.stack[self.stack_ptr].wrapping_add(1);
                },
                b'-' => {
                    self.stack[self.stack_ptr] = self.stack[self.stack_ptr].wrapping_sub(1);
                },
                b'.' => {
                    print!("{}", self.stack[self.stack_ptr] as char);
                    stdout().flush();
                },
                b',' => {
                    let mut ic = 0;
                    unsafe { ic = libc::getchar(); }
                    self.stack[self.stack_ptr] = ic as u8;
                },
                b'[' => {
                    if self.stack[self.stack_ptr] == 0 {
                        let mut nested = 0;
                        loop { 
                            i+=1;
                            if char_vec[i] == b']' && nested == 0 {
                                i+=1;
                                continue 'mainloop;
                            }

                            if char_vec[i] == b']' && nested != 0 {
                                nested -= 1;
                            }

                            if char_vec[i] == b'[' {
                                nested += 1; 
                            };
                        }
                    }
                },
                b']' => {
                    if self.stack[self.stack_ptr] != 0 {
                        let mut nested = 0;
                        loop {
                            i-=1;
                            if char_vec[i] == b'[' && nested == 0 {
                                i+=1;
                                continue 'mainloop;
                            }

                            if char_vec[i] == b'[' && nested != 0 {
                                nested -= 1;
                            }

                            if char_vec[i] == b']' { 
                                nested += 1;
                            }
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

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut interp = Interpreter::new(30000, 0);
    
    if args.len() > 1 {
        let mut f = match File::open(&args[1]) {
            Ok(ok) => ok,
            Err(_) => panic!("Unable to open file: {}", &args[1]),
        };

        let mut file_contents = String::new();
        match f.read_to_string(&mut file_contents) {
            Ok(_) => (),
            Err(_) => panic!("Unable to read from file: {}", &args[1]),
        }

        interp.interp(file_contents);
    }else{
        let stdin = stdin();
        for line in stdin.lock().lines() {
            interp.interp(line.unwrap());
        }
    }
}
