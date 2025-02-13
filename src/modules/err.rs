/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the standard
/// "Result" enum.
use std::fmt::Result;

/// Importing the
/// module containing
/// Actix Web's 
/// "ResponseError".
use actix_web::error;

/// Importing the standard
/// "Display" trait.
use std::fmt::Display;

/// Importing the standard
/// "Error" trait.
use std::error::Error;

/// Importing the standard
/// "Formatter" trait.
use std::fmt::Formatter;

/// A data structure for
/// storing and handling errors.
#[derive(Clone,Eq,PartialEq, Debug)]
pub struct SentinelErr {
    pub details: String
}

/// Implements functions
/// for the "SentinelErr"
/// structure.
impl SentinelErr {

    /// Implements a function to create
    /// a new instance of this data structure.
    pub fn new(details: &str) -> SentinelErr {
        SentinelErr {
            details: details.to_owned()
        }
    }

    /// Implements a function to return
    /// a string representation of this 
    /// data structure.
    pub fn to_string(self) -> String {
        self.details.to_string()
    }
}

/// Implements the error trait.
impl Error for SentinelErr {
    fn description(&self) -> &str {
        &self.details
    }
}

/// Implements the "Display" trait
/// for the "SentinelErr" structure.
impl Display for SentinelErr {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f,"{}",self.details)
    }
}

/// "Deriving" a response error for the 
/// "SentinelErr" structure.
impl error::ResponseError for SentinelErr {}