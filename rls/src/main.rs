use std::env;
use std::fs;
use std::path::Path;
fn print_help() {
    println!("USAGE OF RLS:\n rls")
}

#[derive(Copy, Clone)]
enum Size {
    Infinite,
    Depth(i32),
}

fn read_current() {
    let current = fs::read_dir(".").unwrap();
    for dir in current {
        if dir.is_err() {
            continue;
        }
        let entry = dir.unwrap();
        let typ = entry.file_type().unwrap();
        if typ.is_file() {
            println!("<F> {}", entry.file_name().to_string_lossy());
        } else {
            println!("<D> {}", entry.file_name().to_string_lossy());
        }
    }
}

fn read_recursive(size: Size, delimiter: u32, path: &Path) {
    if let Size::Depth(0)=size{
        return;
    }
    let del = if delimiter == 0 {""} else {&*" ".repeat(delimiter as usize)};
    for el in path.read_dir().unwrap(){
        let el = el.unwrap();
        if el.file_type().unwrap().is_file(){
            println!("{}{}  <F> {}", del, if delimiter==0 {""} else {"|"}, el.file_name().to_string_lossy());
        } else {
            println!("{}{}  <D> {}", del, if delimiter==0 {""} else {"|"}, el.file_name().to_string_lossy());
            match size{
                Size::Infinite=>
                    read_recursive(Size::Infinite, delimiter+4, &*el.path()),
                Size::Depth(num)=>
                    read_recursive(Size::Depth(num-1), delimiter+4, &*el.path())
            }
        }
    }
}

fn parse_args(_args: &Vec<String>) {
    for el in &_args[1..] {
        if el == "-c" {
            read_current();
        }
        if el.starts_with("-r") {
            let i = el.find("=");
            if i.is_some() {
                let value = &el[i.unwrap()+1..];
                let number = value.parse::<i32>();
                if number.is_err() {
                    println!("Invalid number: {}", value);
                    continue;
                }
                read_recursive(Size::Depth(number.unwrap()), 0, Path::new("."));
            } else {
                read_recursive(Size::Infinite,0, Path::new("."));
            }
        }
    }
}

fn main() {
    let _args: Vec<String> = env::args().collect();
    match _args.len() {
        1 => print_help(),
        _ => parse_args(&_args),
    }
}
