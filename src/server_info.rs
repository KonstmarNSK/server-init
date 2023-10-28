use std::{net::IpAddr, fmt::{Display, format}};

use crate::commands::UserCommands;

pub struct ServerInfo{
    pub root_pass: String,
    pub address: IpAddr,

    pub commands: Vec<UserCommands>
}


impl Display for ServerInfo{

    // I don't really care about performance here. It's just an interactive cli tool
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { 
        let commands : Vec<String> = self.commands.iter()
            .map( |cmd| format!("{:?}", cmd))
            .collect();

        let result = format!(
            "
                Address is: {:?}
                Root pass is: {} (Let everyone know it! We don't have secrets from each other, right?)

                For users:
                {:?}
            ",

            self.address,
            self.root_pass,

            commands
        );  

        f.write_str(&result.as_str());
        

        Ok(())
    }
}