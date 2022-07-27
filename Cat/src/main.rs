
use std::env;
use std::process::exit;
use std::fs;

const HELP_MSG: &str = "
Usage: cat <File_name>";

fn main() {


    // Prints each argument on a separate line
    let args: Vec<String> = env::args().collect();

    if &args.len() < &2 {
        println!("{}", HELP_MSG);
        exit(1);
    }else {}

    let contents = match fs::read_to_string(&args[1]) {
        Ok(file) => file,
        Err(_) => panic!("Could not read file!"),
    };

    println!("File:{} \n-------------------", &args[1]);
    println!("{}", contents);

    exit(0);
}
