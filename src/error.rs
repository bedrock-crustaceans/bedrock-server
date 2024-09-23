use std::error::Error;

pub enum StartError {
    
}

#[derive(Debug, Clone)]
pub enum LoginError {
    Other(dyn Error)
}
