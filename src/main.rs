extern crate libc;
extern crate clap;

use std::io::{ Read, BufRead, Write, stdout, stdin };
use std::fs::File;
use std::env;

use clap::{ Arg, App };

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
    let arg_matches = App::new("rs-bf")
                              .version("0.1.0")
                              .author("Harrison Rigg <harrison.rigg@student.ntschools.net>")
                              .about("A simple brainf**k interpreter written in Rust")
                              .arg(Arg::with_name("stack-size")
                                .short("s")
                                .long("stack-size")
                                .value_name("STACK-SIZE")
                                .help("Sets the size to be used for the stack/tape")
                                .takes_value(true)
                                .required(false))
                              .arg(Arg::with_name("stack-value")
                                .short("i")
                                .long("stack-value")
                                .value_name("STACK-VALUE")
                                .help("The value that will fill the stack/tape initialy")
                                .takes_value(true)
                                .required(false))
                              .arg(Arg::with_name("FILE")
                                .help("The file to interpret")
                                .required(false)
                                .index(1))
                              .get_matches();

    let stack_size_str = arg_matches.value_of("stack-size").unwrap_or("30000");
    let stack_size = match stack_size_str.parse::<usize>() {
        Ok(ok) => ok,
        Err(_) => panic!("Please enter a valid number for stack-size")
    };

    let stack_value_str = arg_matches.value_of("stack-value").unwrap_or("0");
    let stack_value = match stack_value_str.parse::<u8>() {
        Ok(ok) => ok,
        Err(_) => panic!("Please enter a valid number for stack-value")
    };

    let mut interp = Interpreter::new(stack_size, stack_value);

    if let Some(file_name) = arg_matches.value_of("FILE") {
        let mut f = match File::open(file_name) {
            Ok(ok) => ok,
            Err(_) => panic!("Unable to open file: {}", file_name)
        };

        let mut file_contents = String::new();
        match f.read_to_string(&mut file_contents) {
            Ok(_) => (),
            Err(_) => panic!("Failed to read file contents: {}", file_name)
        };

        interp.interp(file_contents);
    } else {
        let stdin = stdin();
        for line in stdin.lock().lines() {
            interp.interp(line.unwrap());
        }
    }
}
