use std::{
    io::{self, BufRead}, 
    net::{IpAddr, Ipv4Addr, AddrParseError}, 
    fmt::format, 
    error::Error,
    num::ParseIntError,
    path::PathBuf
};

use commands::{UserLogin, Command, InitUserCmd, InitSshCmdData};

use crate::{commands::{Port, SshKeyCmd, UserCommands}, server_info::ServerInfo};

mod commands;
mod server_info;


type InputResult<T> = Result<T, InputParseError>;


 fn main() -> InputResult<()> {
     println!("\n=========\n\n Let's init server! \n===================\n\n");
     
    let stdin = io::stdin();
    let server_addr = askAddress(&stdin)?;

    println!("What password does root have?");

    let mut buff = String::new();
    stdin.read_line(&mut buff);
    let root_passw = buff.trim().to_owned();


    let commands = inputToCmds()?;


    let result = ServerInfo{
        address: server_addr,
        root_pass: root_passw,
        commands
    };

    println!("Summary: {}", result);

    Ok(())
 }


fn inputToCmds() -> InputResult<Vec<UserCommands>>{
    let stdin = io::stdin();

    let root_login = UserLogin("root".into());
    let root_init = ask_user_inits(&root_login, &stdin)?;

    let result = vec![
        UserCommands{
            commands: vec![Command::InitUser(root_init)],
            user: root_login,
        }
    ];

    Ok(result)    
}


fn askAddress(stdin: &io::Stdin) -> InputResult<IpAddr>{
    println!("What is the server's ip? (v4)");
    
    let mut server_address = String::new();
    stdin.read_line(&mut server_address);

    let server_address = server_address.trim();

    return Ok(
        server_address.parse::<IpAddr>()?
    );
}

fn ask_user_inits(user: &UserLogin, stdin: &io::Stdin) -> InputResult<Vec<InitUserCmd>> {
    println!("Init user '{}'", &user.0);
    
    let ssh_init = askSsh(stdin)?;
    
    println!("
        Now about UFW. 
        What ports should allow incoming traffic (BEYOND that ssh port {} you specified)?
        Write comma-separated numbers
    ", ssh_init.ssh_port);

    let mut buff = String::new();
    stdin.read_line(&mut buff);

    let mut ports: Vec<Port> = vec![];
    
    for str_num in buff.trim().split(',') {
        let parsed = str_num.trim().parse::<Port>()?;
        ports.push(parsed);
    };

    let result = vec![
        InitUserCmd::InitSsh(ssh_init),
        InitUserCmd::InitUfw { ports_allowed: ports }
    ];

    Ok(result)
}

fn askSsh(stdin: &io::Stdin) -> InputResult<InitSshCmdData> {
    println!("Now tell about ssh");
    println!("What port will ssh use?");

    let mut buffer = String::new();

    stdin.read_line(&mut buffer);
    let ssh_port : Port = buffer.trim().parse::<Port>()?;

    println!("Should i generate new ssh key? (y for 'yes', n for 'i wish to use existing key')");
    buffer.clear();

    stdin.read_line(&mut buffer);
    let must_generate_key = match buffer.trim() {
        "y" => true,
        "n" => false,
        _ => panic!("User is an idiot with dyslexia"),
    };
    

    buffer.clear();
    let ssh_key = match must_generate_key {
        true => {
            let path: PathBuf;
            let name: String;

            println!("Where should be this key generated? Write a directory name inside ~/.ssh/  (will be created)");
            stdin.read_line(&mut buffer);

            // todo: fix relative path
            path = PathBuf::from(&buffer.trim());

            buffer.clear();
            println!("Name of key file please");
            stdin.read_line(&mut buffer);
            
            name = buffer.trim().to_owned();

            SshKeyCmd::CreateNew { path, name }
        },

        false => {
            let path: PathBuf;

            println!("Where it is? path relative to ~/.ssh/");
            stdin.read_line(&mut buffer);

            //todo: the same
            path = PathBuf::from(&buffer.trim());

            SshKeyCmd::UseExisting { path }
        }
    };


    let result = InitSshCmdData{
        ssh_port,
        ssh_key,
    };
    
    Ok(result)
}



// ============  Errors

#[derive(Debug)]
enum InputParseError{
    Address(String),
    Other(String),
}

impl From<AddrParseError> for InputParseError{
    fn from(value: AddrParseError) -> Self {
        Self::Address(format!("Couldn't parse ip address. {}", value))
    }
}

impl From<ParseIntError> for InputParseError {
    fn from(value: ParseIntError) -> Self {
        Self::Other("Couldn't parse number".into())
    }
}