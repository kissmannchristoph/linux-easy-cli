use std::fs;
use std::str;
use serde::{Deserialize, Serialize};
use serde_yaml::{self};

pub fn read_file_to_string() -> Result<String, std::io::Error> {
    return fs::read_to_string("data/commands.yaml");
}

pub fn load() -> Result<Vec<Command>, CommandLoadError> {
    let mut rs = vec![];
    let cfg_file = read_file_to_string();

    if cfg_file.is_err() {
        return Err(CommandLoadError { r#type: "file not found".to_string() });
    }

    let tmp_obj = serde_yaml::from_str(cfg_file.unwrap().as_str());

    if tmp_obj.is_err() {
        println!("{:?}", tmp_obj.as_ref().unwrap_err());
        return Err(CommandLoadError { r#type: tmp_obj.unwrap_err().to_string() });
    }

    let obj_vec: CfgCommands = tmp_obj.unwrap();

    for o in obj_vec {
        let t = String::from(o.r#type);
        rs.extend([Command {
            r#type: t.clone(), echo: o.echo, alias: o.alias, command: o.command, require_value: o.require_value, parsed_type: Some(parse_command_type(&t.to_string()).unwrap())
        }]);
    }

    return Ok(rs);
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CommandType {
    CommandTypeAlias,
    CommandTypeEcho
}

pub fn parse_command_type(t: &String) -> Option<CommandType> {
    if t.to_owned() == "alias".to_string() {
        return Some(CommandType::CommandTypeAlias);
    } else if t.to_owned() == "echo".to_string() {
        return Some(CommandType::CommandTypeEcho);
    }
   return None; 
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommandLoadError {
    r#type: String
}

//#[derive(Debug, Serialize, Deserialize)]
pub type CfgCommands = Vec<Command>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub require_value: bool,
    pub command: String,
    pub r#type: String,
    pub parsed_type: Option<CommandType>,
    pub alias: Option<String>,
    pub echo: Option<String>
}
