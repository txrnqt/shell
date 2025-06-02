use anyhow::anyhow;

#[derive(Clone, Debug)]
pub enum Command {
    Exit,
    Echo(String),
    Ls,
    Pwd,
    Cd(String),
    Touch(String),
    Rm(String),
    Cat(String),
} 

impl TryFrom<&str> for Command {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split_value: Vec<&str> = value.split_whitespace().collect();
        match split_value[0] {
            "exit" => Ok(Command::Exit),
            "ls" => Ok(Command::Ls),
            "echo" => {
                if split_value.len() < 2 {
                    Err(anyhow!("echo requires an argument"))
                } else {
                    Ok(Command::Echo(split_value[1..].join(" ")))
                }
            }
            "pwd" => Ok(Command::Pwd),
            "cd" => {
                if split_value.len() < 2 {
                    Err(anyhow!("cd requires an argument"))
                } else {
                    Ok(Command::Cd(split_value[1..].join(" ")))
                }
            }
            "touch" => {
                if split_value.len() < 2 {
                    Err(anyhow!("touch requires an argument"))
                } else {
                    Ok(Command::Touch(split_value[1..].join(" ")))
                }
            }
            "rm" => {
                if split_value.len() < 2 {
                    Err(anyhow!("rm requires an argument"))
                } else {
                    Ok(Command::Rm(split_value[1..].join(" ")))
                }
            }
            "cat" => {
                if split_value.len() < 2 {
                    Err(anyhow!("cat requires an argument"))
                } else {
                    Ok(Command::Cat(split_value[1..].join(" ")))
                }
            }
            _ => Err(anyhow!("Unknown command!!!")),
        }
    }
}