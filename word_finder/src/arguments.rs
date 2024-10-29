pub mod help;
pub mod search;
pub mod file_reader;

use crate::errors::*;

pub struct ArgsStruct {
    pub file_path: String,
    pub word: String,
}

pub fn parse_args(args: Vec<String>) -> Result<ArgsStruct, ArgsError> {
    if args.len() < 3 {
        help::print_help();
        return Err(ArgsError::InvalidArguments);
    }
    let args_struct = ArgsStruct {
        file_path: args[1].clone(),
        word: args[2].clone(),
    };

    return Ok(args_struct);
}
