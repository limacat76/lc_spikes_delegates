use std;

pub trait Environment {
    fn get_name(&self) -> std::string::String;
}