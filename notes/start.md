* Rustlang learning note
sudo apt-get update
sudo apt install build-essential

Macro - ! after the function name (can take a variable number of arguments, functions cannot)
Cargo check
Cargo build
Cargo build --release
Option<String> -> optional string

docs.rs/std -> url for rust std library

match -> pattern matching (match Ok(()) or Err(e))
Option unwrap() -> panic if none
Option expect("message") -> panic and show "message"
Result unwrap() -> panic if err exist
Result<T,E> type is an enum, T and Error with values

use impl{} to create associative function and methods to struct
to create methods, the first parameter will be "self"

import -> use std::collection::HashMap;

if fn can fail with error, return Result<(), std::io::Error>

match and if are expressions, can be binded to variables

let content = std::fs::read_to_string("kv.db")?; -> use ? to propagate the error to the caller

use todo!() to tell compiler that the return type will be handled later

the last method to be called can take ownership of self so that when other methods are called after this method, the compiler will warn you of that

"drop trait" specifies what happens when an owned value goes out of scope
