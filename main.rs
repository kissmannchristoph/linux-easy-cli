use lib::key_value_argument_parser::HasKeysOption::{*};
use lib::{key_value_argument_parser, commands_loader};

mod lib {
    pub mod key_value_argument_parser;
    pub mod commands_loader;
}

fn main() {
    let commands = commands_loader::load();
    
    if commands.is_err() {
        println!("commands has errors");
        return;
    }

    let unw_command = commands.unwrap();

    println!("--commands loaded--");
    for command in &unw_command {
        println!("Command: {:?}", &command.clone().command);
        println!("Type: {:?}", &command.clone().parsed_type);
    }
    println!("--end--");
    let mut input_string = String::new();

    loop {
        input_string = "".to_string();
        std::io::stdin().read_line(&mut input_string).unwrap();
        println!("asdsad {:?}", input_string.to_owned().replace("\n", "") );
        if input_string.replace("\n", "") == "exit" {
            break;
        }

        let cleared = input_string.replace("\n", "");

        let mut found: Option<&commands_loader::Command> = None;
        
        for command in &unw_command {
            if command.command == cleared {
                found = Some(command);
                break;
            }
        }

        if found.is_none() {
            println!("Command not found");
            return;
        }

        let parsed_command: &commands_loader::Command = found.unwrap();
    
        match parsed_command.parsed_type.as_ref().unwrap() {
            commands_loader::CommandType::CommandTypeAlias => {
                let alias = &parsed_command.to_owned().alias;
                
                if alias.is_none() {
                    println!("if you set alias as type, you need to set the alias value");
                    return;
                }

                println!("found alias: {:?}", alias.to_owned().unwrap())
            },
            commands_loader::CommandType::CommandTypeEcho => {
                let echo = &parsed_command.to_owned().echo;
                
                if echo.is_none() {
                    println!("if you set echo as type, you need to set the echo value");
                    return;
                }

                println!("found echo: {:?}", echo.to_owned().unwrap())
            }
        }
    }




   
    /*let argument_list = key_value_argument_parser::get_argument_list();
    
    if argument_list.get_errors().is_some() {
        println!("you have errors");
        return;
    }

    let has_keys_result = argument_list.has_keys(vec!["bla".to_string(), "ic".to_string()], Some(vec![HasKeysOptionSameKeys, HasKeysOptionSameLen]));

    if has_keys_result.is_error() {
        println!("--has key error--");
        println!("Required Keys: {:?}", has_keys_result.req_keys);
        println!("Current Keys: {:?}", has_keys_result.current_keys);
        println!("--end--");
        return;
    }

    for argument in argument_list.get_list() {
        let parsed = argument.get();
        if parsed.is_err() {
            println!("ERORR: only able to a single =");
            continue;
        }

        let arg = parsed.unwrap();

        let key = arg.get_key();

        if arg.get_value().is_ok() {
            let value = String::from(arg.get_value().unwrap());
            println!("argument key={key} value={value}");
        } else {
            println!("argument key={key}");
        }


    }*/
}
 