extern crate libc;

use std::io::{stdout, Read, Write};

// Represents a Interpreter, which holds a the tape and the tape ptr
pub struct Interpreter {
    stack: Vec<u8>,
    stack_ptr: usize
}

impl Interpreter {

    // Creates a new Interpreter with the specified stack size and initial stack value
    pub fn new(stack_size: usize, stack_value: u8) -> Interpreter {
        Interpreter { stack: vec![stack_value; stack_size], stack_ptr: 0 }
    }

    // Interperets the the string passed in
    pub fn interp(&mut self, source: String) {

        // Get a vector of bytes to read through
        let mut char_vec: Vec<u8> = Vec::new();
        (source.as_ref() as &[u8]).read_to_end(&mut char_vec);

        // Loop through all the bytes
        let mut i = 0;
        'mainloop: while i < char_vec.len() {
            let c = char_vec[i];
            match c {
                // > -- Stack ptr forwards
                b'>' => {
                    self.stack_ptr = self.stack_ptr.wrapping_add(1);
                },
                // < -- Stack ptr backwards
                b'<' => {
                    self.stack_ptr = self.stack_ptr.wrapping_sub(1);
                },
                // + -- Add to current stack value
                b'+' => {
                    self.stack[self.stack_ptr] = self.stack[self.stack_ptr].wrapping_add(1);
                },
                // - -- Take from current stack value
                b'-' => {
                    self.stack[self.stack_ptr] = self.stack[self.stack_ptr].wrapping_sub(1);
                },
                // . -- Prints the current stack value to STDOUT
                b'.' => {
                    print!("{}", self.stack[self.stack_ptr] as char);
                    stdout().flush();
                },
                // , -- Places a char from STDIN into the current stack value
                b',' => {
                    let mut ic = 0;
                    unsafe { ic = libc::getchar(); }
                    self.stack[self.stack_ptr] = ic as u8;
                },
                // [ -- If current stack value is 0, jump over to the next ']', else continue
                // execution
                b'[' => {
                    if self.stack[self.stack_ptr] == 0 {
                        // We need to check for nested loops, nested increments everytime we find a
                        // '[' and decrements when we find a ']', if nested is zero and were on a
                        // ']', we end the jump ahead
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
                // ] -- Jumps back to the last '[' if the current stack value is 0, else, continues
                // execution
                b']' => {
                    if self.stack[self.stack_ptr] != 0 {
                        // Same mechanics here as '[', just reversed
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
                // Anything else, leave it be. Brainf**k ignores all other characters :) :) :)
                _   => {
                
                }
            }
            // Jump ahead through the source
            i+=1;

        }
    }
}


