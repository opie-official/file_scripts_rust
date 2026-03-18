use std::env;
use std::fs;
use std::io::Read;

const SEPARATOR: &str = "\n________________________________________________________\n";
const SEPARATOR2: &str = "\n!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!\n";

fn print_help() {
    println!("USAGE OF ROUT:\nrout <*files>");
}

fn parse_args(_args: &Vec<String>) {
    for el in &_args[1..] {
        let file = fs::File::open(&el);
        if file.is_err() {
            println!("{}{}{} Cannot open this file", SEPARATOR, el, SEPARATOR2,);
        }
        let mut text = String::new();
        let _res = file
            .unwrap()
            .read_to_string(&mut text)
            .map_err(|e| format!("Error while reading file {}: {}", el, e));
        if _res.is_err(){
            println!("{0}{1}{2} Error while reading {1}: {3}", SEPARATOR, el, SEPARATOR2, _res.err().unwrap());
            
        }

        println!("{0}{1}{0}{2}", SEPARATOR, el, &text);
    }
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    match _args.len() {
        1 => print_help(),
        _ => parse_args(&_args),
    }
}
