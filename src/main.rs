extern crate libc;
extern crate clap;

use std::io::{ Read, BufRead, Write, stdout, stdin };
use std::fs::File;

use clap::{ Arg, App };

mod interpreter;
use interpreter::Interpreter;

fn main() {
    // Parse command line args
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
                                .help("The file to interpret. If left blank stdin will be used instead.")
                                .required(false)
                                .index(1))
                              .get_matches();

    // Set the stack size
    let stack_size_str = arg_matches.value_of("stack-size").unwrap_or("30000");
    let stack_size = match stack_size_str.parse::<usize>() {
        Ok(ok) => ok,
        Err(_) => panic!("Please enter a valid number for stack-size")
    };

    // Set the initial stack value
    let stack_value_str = arg_matches.value_of("stack-value").unwrap_or("0");
    let stack_value = match stack_value_str.parse::<u8>() {
        Ok(ok) => ok,
        Err(_) => panic!("Please enter a valid number for stack-value")
    };

    // Create the interpreter with the default args/command line args
    let mut interp = Interpreter::new(stack_size, stack_value);

    // If a filename was specifed, interpret it, and check that it exists first
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
        // Else if no filename was specified, read from STDIN
        let stdin = stdin();
        for line in stdin.lock().lines() {
            interp.interp(line.unwrap());
        }
    }
}
