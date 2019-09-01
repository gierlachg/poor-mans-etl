use core::fmt;
use std::error::Error;

use crate::record::Record;

pub trait Extractor<R>: Iterator<Item=R> where R: Record {}

#[derive(Debug)]
pub struct ExtractorError {
    message: String
}

impl ExtractorError {
    pub fn new(message: &str) -> ExtractorError {
        ExtractorError { message: message.to_string() }
    }
}

impl fmt::Display for ExtractorError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{}", self.message)
    }
}

impl Error for ExtractorError {
    fn description(&self) -> &str {
        &self.message
    }
}