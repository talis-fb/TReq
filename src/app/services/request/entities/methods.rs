use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum METHODS {
    #[default]
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    PATCH,
}

impl FromStr for METHODS {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "GET" => METHODS::GET,
            "POST" => METHODS::POST,
            "PUT" => METHODS::PUT,
            "DELETE" => METHODS::DELETE,
            "HEAD" => METHODS::HEAD,
            "PATCH" => METHODS::PATCH,
            _ => return Err(anyhow::Error::msg("No valid METHOD")),
        })
    }
}

impl ToString for METHODS {
    fn to_string(&self) -> String {
        self.as_str().into()
    }
}
impl METHODS {
    pub fn as_str(&self) -> &'static str {
        match self {
            METHODS::GET => "GET",
            METHODS::POST => "POST",
            METHODS::PUT => "PUT",
            METHODS::DELETE => "DELETE",
            METHODS::HEAD => "HEAD",
            METHODS::PATCH => "PATCH",
        }
    }
}
