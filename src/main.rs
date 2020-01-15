use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Handle ctrlc
    ctrlc::set_handler(|| {}).expect("Failed to set ctrlc handler");
    // Get prompt
    let PS1 = match env::var("PS1") {
        Ok(p) => p,
        Err(_) => "% ".to_string(),
    };

    loop {
        print!("{}", PS1);
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("Failed to read line...");
        let cmd: Vec<&str> = s.trim().split(" ").collect();

        // Execute the command
        match cmd[0] {
            "exit" => return,
            "cd" => {
                if cmd.len() > 1 {
                    let res = env::set_current_dir(cmd[1]);
                } else {
                    let res = env::set_current_dir(match env::var("HOME") {
                        Ok(h) => h,
                        Err(_) => String::from("/"),
                    });
                }

                match res = {
                    Ok(_) => {},
                    Err(e) => println!("{}", res);
                };
            },

            // Variables
            "val" => {
                if cmd.len() < 4 {
                    println!("Expected 3 arguments");
                } else if cmd[2] != "=" {
                    println!("Expected =");
                } else {
                    env::set_var(cmd[1], cmd[3]);
                }
            },
            "dbg" => {
                println!("{}", match env::var(cmd[1]) {
                    Ok(v) => v,
                    Err(e) => format!("{}", e),
                });
            },
            _ => {
                let mut child = match Command::new(cmd[0]).args(&cmd[1..]).spawn() {
                    Ok(c) => c,
                    Err(e) => {
                        println!("rush: error: {}", e);
                        continue;
                    },
                };

                // Don't let other processess spawn
                child.wait();
            },
        };
    }
}
