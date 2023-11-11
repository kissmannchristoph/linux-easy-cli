use std::env::args;
use core::option::Option;

fn get_args_pattern() -> &'static str {
    return "=";
}

#[derive(Debug)]
pub struct KeyValueArgument {
    key: String,
    value: Option<String>,
}

impl KeyValueArgument {
    pub fn get_key(&self) -> String {
        return String::from(&self.key.to_owned());
    }
    pub fn get_value(&self) -> Result<String, ()> {
        if *&self.value.is_none() {
            return Err(());
        } else {
           return Ok(String::from(&self.value.to_owned().unwrap()));
        }
    }
}

#[derive(Debug)]
pub struct KeyValueArgumentParse {
    data: Option<KeyValueArgument>,
}

impl KeyValueArgumentParse {
    fn is_error(&self) -> bool {
        let is = &self.data.is_none().to_owned();
        return *is;
    }
    pub fn get(&self) -> Result<&KeyValueArgument, ()> {
        if *&self.is_error() {
            return Err(());
        } else {
           return Ok(&self.data.as_ref().unwrap());
        }
    }
}

pub struct KeyValueArgumentList {
    list: Vec<KeyValueArgumentParse>
}

pub enum HasKeysOption {
    HasKeysOptionSameKeys = 0,
    HasKeysOptionSameLen = 1
}

mod keys {
    pub struct HasKeysResult {
        pub state: bool,
        pub req_keys: Vec<String>,
        pub current_keys: Vec<String>,
    }

    impl HasKeysResult {
        pub fn is_error(&self) -> bool { 
            let b: bool = *&self.state.to_owned();
            return !b;
        }
    }
}

impl KeyValueArgumentList {
    pub fn get_errors(&self) -> Option<Vec<&KeyValueArgumentParse>> {
        let mut arr: Vec<&KeyValueArgumentParse> = vec![];
        for item in &self.list {
            let i = item.to_owned();
            if i.get().is_err() {
                arr.extend([i]);
            }
        }

        if arr.len() == 0 {
            return None;
        }

        return Some(arr);
    }
    pub fn get_list(&self) -> &Vec<KeyValueArgumentParse> {
        return &self.list;
    }
    pub fn has_keys(&self, keys: Vec<String>, options: Option<Vec<HasKeysOption>>) -> keys::HasKeysResult {
        let mut state = true;
        let mut current_keys = vec![];

        for parse in &self.list {
            current_keys.extend([parse.to_owned().get().unwrap().get_key()])
        }

        if options.is_some() {
        for option in options.unwrap() {
            match option {
                HasKeysOption::HasKeysOptionSameKeys => {
                    for key in keys.clone() {
                        if !current_keys.contains(&key) {
                            state = false;
                            break;
                        }
                    }
                },
                HasKeysOption::HasKeysOptionSameLen => {
                    if current_keys.len() != keys.len().to_owned() {
                        state = false;
                        break;
                    }
                }
            }
        }
    }

        return keys::HasKeysResult {
            state, req_keys: keys, current_keys
        };
    }   
}

fn count_char_occurrences(input_str: &str, target_char: char) -> usize {
    input_str.chars().filter(|&c| c == target_char).count()
}

fn parse_argument(argument: &str) -> KeyValueArgumentParse {
    let mut data = KeyValueArgumentParse { data: None };

    if count_char_occurrences(argument, '=') > 1 {
        return data;
    }

    let mut key_value_argument = KeyValueArgument {
        key: "".to_string(),
        value: None,
    };

    if argument.contains(get_args_pattern()) {
        let split = argument.split(get_args_pattern());
        let collection: Vec<&str> = split.collect();

        key_value_argument.key = collection[0].to_string();
        key_value_argument.value.get_or_insert(collection[1].to_string()); //= String::from("sad");
    } else {
        key_value_argument.key = argument.to_owned();
    }

    data.data.get_or_insert(key_value_argument);

    return data;
}

pub fn get_argument_list() -> KeyValueArgumentList {
    let mut list = KeyValueArgumentList {
        list: vec![]
    };

    let args_vec: &Vec<String> = &args().collect();
    let args_list: Vec<String> = args_vec.clone();
    let min: &String = &args_list.into_iter().max().unwrap();

    for arg in args_vec.to_owned() {
        if arg == min.to_owned() {
            println!("[Argument Loader] skip first argument {:?}", arg);
            continue;
        }

        list.list.extend([parse_argument(&arg)])
    }
    
    return list;
}

 

