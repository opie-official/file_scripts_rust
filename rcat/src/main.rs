use std::env;
use std::fs;
use std::fs::File;
use std::path::Path;

fn make_file(filename: &str) -> Result<(), String> {
    File::create(filename).map_err(|e| format!("Error while creating file {}: {}", filename, e))?;
    Ok(())
}

fn make_dir(dirname: &str) -> Result<(), String> {
    if dirname.contains("\\") || dirname.contains("/") {
        return Err("Error while creating directory: cannot create path as direactory".to_string());
    }
    let dir = dirname.replace(":", "");
    fs::create_dir(&dir).map_err(|e| format!("Error while creating directory {} : {}", &dir, e))?;

    Ok(())
}


fn make_path(path: &str) -> Result<(), String> {
    let path = path.strip_prefix("::").unwrap_or(path);

    let is_file = path.contains('!');
    let path = path.replace('!', "");

    let p = Path::new(&path);

    if is_file {
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Error creating dir {:?}: {}", parent, e))?;
        }
        fs::File::create(p)
            .map_err(|e| format!("Error creating file {:?}: {}", p, e))?;
    } else {
        fs::create_dir_all(p)
            .map_err(|e| format!("Error creating dir {:?}: {}", p, e))?;
    }

    Ok(())
}
fn print_help() {
    println!("USAGE OF RCAT:\n rcat <*filename> <*:dirname> <*::path/to/!file>");
}

fn parse_args(_args: &Vec<String>) {
    for el in &_args[1..] {
        if el.starts_with(":") && !el.starts_with("::") {
            // this is directory
            let res = make_dir(el.as_str());
            if res.is_ok() {
                println!("* <D> directory '{}' was created", el);
            } else {
                println!("! <D> {}", res.err().unwrap());
            }
        } else if el.starts_with("::") {
            // this is path
            let res = make_path(el.as_str());
            if res.is_ok() {
                println!("* <P> path '{}' was created", el);
            } else {
                println!("! <P> {}", res.err().unwrap());
            }
        } else {
            // this is file
            let res = make_file(el.as_str());
            if res.is_ok() {
                println!("* <F> file '{}' was created", el);
            } else {
                println!("! <F> {}", res.err().unwrap());
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
