use std::path::PathBuf;

pub type Port = u16;

#[derive(Debug)]
pub struct UserLogin(pub String);


#[derive(Debug)]
pub struct UserCommands{
    pub user: UserLogin,
    pub commands: Vec<Command>
}

/// A command that will be executed (for a specific user.
///  Note that these commands (in general) don't contain user identity, 
///  so mapping "user -> command" must be handled by client code
/// )
#[derive(Debug)]
pub enum Command{
    CreateUser(CreateUserCmdData),    
    InitUser(Vec<InitUserCmd>),
}

#[derive(Debug)]
pub struct CreateUserCmdData{
    login: UserLogin,
    pass: String,
}

#[derive(Debug)]
pub enum InitUserCmd{
    InitSsh(InitSshCmdData),
    InitUfw{ports_allowed: Vec<Port>}
}

#[derive(Debug)]
pub struct InitSshCmdData{
    pub ssh_port: Port,
    pub ssh_key: SshKeyCmd,
}

#[derive(Debug)]
pub enum SshKeyCmd{
    CreateNew{path: PathBuf, name: String},
    UseExisting{path: PathBuf}
}





// ==============|  Commands implementation  |===============

