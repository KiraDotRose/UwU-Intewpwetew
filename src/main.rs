use std::fs;
use std::env;
use std::io::prelude::*;
use std::process;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    let mut contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    contents = contents.replace("UwU", "+"); 
    contents = contents.replace("QwQ", "-"); 
    contents = contents.replace("OwO", ">"); 
    contents = contents.replace("°w°", "<"); 
    contents = contents.replace("@w@", "."); 
    contents = contents.replace(">w<", ","); 
    contents = contents.replace("~w~", "["); 
    contents = contents.replace("¯w¯", "]"); 

    let mut brainfuck = fs::File::create("/tmp/brainfuck.bf")?;
    brainfuck.write_all(contents.as_bytes())?;

    let mut run_brainfuck = process::Command::new("brainfuck");
    run_brainfuck.arg("/tmp/brainfuck.bf").status();

    Ok(())


}

