use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    println!("[INFO] Current directory: [{}]", env::current_dir().unwrap().display());
    println!("[OK] Searching for file [{}]", &args[1]);

    let input = match fs::read_to_string(&args[1]) {
        Ok(s) => {
            println!("[OK] File found: [{}]", &args[1]);
            s
        },
        Err(_) => {
            println!("[ERR] File not found [{}]", &args[1]);
            return;
        }
    };
    
    let mut floor = 0;
    let mut basement_pos = 0;
    let mut basement_pos_found = false;

    for (i, &c) in input.as_bytes().iter().enumerate() {
        if c == b'(' {
            floor += 1;
        } else if c == b')' {
            floor -= 1;
        }
        if !basement_pos_found && floor == -1 { 
            basement_pos = i+1;
            basement_pos_found = true;
        }
    }

    println!("[Done] Santa should go to floor: [{floor}]");
    println!("[Done] The position of the character that causes Santa to go to the basemenet is at: [{basement_pos}]");
}
