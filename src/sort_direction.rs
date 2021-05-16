use std::{str::FromStr, string::ParseError};


#[derive(Debug, Clone)]
pub enum SortDirection {
    Asc,
    Desc
}


impl FromStr for &SortDirection {
    type Err = ParseError;

    fn from_str(direction: &str) -> Result<Self, Self::Err> {
        match direction {
            "asc" => Ok(&SortDirection::Asc),
            "desc" => Ok(&SortDirection::Desc),
            _ => {
                log::warn!("Unable to parse sort direction \"{}.\" Falling back to \"asc.\"", direction);
                Ok(&SortDirection::Asc)
            },
        }
    }
}
