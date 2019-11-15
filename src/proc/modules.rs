use crate::{Error, Result};
use std::str::FromStr;

/// represent an entry in /proc/modules
#[derive(Debug)]
pub struct Module {
    name: String,
    mem_size: usize,
    instance_nums: usize,
    deps: Vec<String>,
    state: State,
    offset: usize,
}

impl Module {
    getter_gen! {
        name: String,
        mem_size: usize,
        instance_nums: usize,
        deps: Vec<String>,
        state: State,
        offset: usize
    }
}

impl FromStr for Module {
    type Err = Error;

    fn from_str(line: &str) -> Result<Self> {
        let columns: Vec<&str> = line.split_ascii_whitespace().collect();
        if columns.len() != 6 {
            return Err(Error::BadFormat);
        }
        let name = columns[0].to_string();
        let mem_size = columns[1].parse::<usize>()?;
        let instance_nums = columns[2].parse::<usize>()?;
        let deps: Vec<String> = if columns[3] == "-" {
            Vec::new()
        } else {
            columns[3].split(',').map(|s| s.to_string()).collect()
        };
        let state = State::from_str(columns[4])?;
        let offset = usize::from_str_radix(columns[5].trim_start_matches("0x"), 16)?;
        Ok(Module {
            name,
            mem_size,
            instance_nums,
            deps,
            state,
            offset,
        })
    }
}


#[derive(Debug)]
pub enum State {
    Live,
    Loading,
    Unloading,
}

impl FromStr for State {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "Live" {
            Ok(State::Live)
        } else if s == "Loading" {
            Ok(State::Loading)
        } else if s == "Unloading" {
            Ok(State::Unloading)
        } else {
            Err(Error::BadFormat)
        }
    }
}

#[inline(always)]
fn to_modules(line: &str) -> Result<Module> {
    Module::from_str(line)
}

default_list! {
    modules, "/proc/modules", Module, to_modules
}