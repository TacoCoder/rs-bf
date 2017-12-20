## rs-bf
[![Build Status](https://travis-ci.org/TacoCoder/rs-bf.svg?branch=master)](https://travis-ci.org/TacoCoder/rs-bf)

A simple Brainf**k interpreter written in pure Rust.
Wrote this as a learning project, not meant to be practical, but if you have any suggestions let me know!

Example Usage: `rs-bf some_file.bf --stack-size 30000 --stack-value 0`   
Leaving `[FILE]` blank uses stdin instead.

Using without driver:
```rust
  extern crate rs_bf;
  use rs_bf::interpreter::Interpreter;

  // Takes stack size and initial value
  let mut interp = Interpreter::new(30000, 0);

  // Prints "Hello World" to the screen
  interp.interp("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.");
```

##### Coming Soon
* ~~More/better command line arguments~~
* ~~Cleaner code with comments~~
* ~~A library along with a better driver~~
* A compiler
