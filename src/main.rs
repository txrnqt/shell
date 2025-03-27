use std::{env, fs::{self, read}, io::{stdin, stdout, Write}, path::{Display, Path}, process::{Child, Command, Stdio}};

use walkdir::{DirEntry, WalkDir};

fn main() {
    loop {
        // > = character promt
        // flush make sure it prints before read_line

        // let mut curent_dir: = Path::display(&self);

        print!("> ");
        stdout().flush();

        let mut input  = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

        match command {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let  Err(e) = env::set_current_dir(&root) {
                    eprint!("{}", e);
                }

                previous_command = None;

            },
            "ls" => {
                    fn is_hidden(entry: &DirEntry) -> bool {
                        entry
                            .file_name()
                            .to_str()
                            .map(|s| s.starts_with('.'))
                            .unwrap_or(false)
                    }

                    for entry in WalkDir::new(".")
                        .min_depth(1)
                        .max_depth(1)
                        .into_iter()
                        .filter_entry(|e| !is_hidden(e))
                    {
                        let entry = entry.unwrap();
                        println!("{}", entry.path().display());
                    }
                    
                    previous_command = None;
            },
            "cat" => {

                let mut x = &args.peekable().peek();
                let read = fs::read(x);
                println!("{}", read);
                previous_command = None;
            },
            "exit" => return,
            command => {
                let stdin = previous_command
                    .map_or(Stdio::inherit(), |output: Child| Stdio::from(output.stdout.unwrap()));

                let stdout = if commands.peek().is_some() {
                    Stdio::piped()
                } else {
                    Stdio::inherit()
                };

                let output = Command::new(command)
                    .args(args)
                    .stdin(stdin)
                    .stdout(stdout)
                    .spawn();

                match  output {
                    Ok(output) => { previous_command = Some(output); }, 
                    Err(e) => {
                        previous_command = None;
                        eprintln!("{}", e)
                    }
                }
            }
        }
    }
    if let Some(mut final_command) = previous_command {
        final_command.wait();
    }
}
}
