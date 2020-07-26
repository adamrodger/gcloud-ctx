use crate::Error;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_ini::{Serializer, Writer};
use std::{str::FromStr, io::{Read, Write}};
use regex::Regex;

/// Builder module
pub mod builder;
pub use builder::PropertiesBuilder;

// Region regex main string - matches strings such as europe-west1 and us-east2
const REGION_REGEX_STRING: &str = "[a-z]+-[a-z]+[0-9]";

lazy_static! {
    static ref REGION_REGEX: Regex = Regex::new(&format!("^{}$", REGION_REGEX_STRING)).unwrap();
    static ref ZONE_REGEX: Regex = Regex::new(&format!("^{}-[a-z]$", REGION_REGEX_STRING)).unwrap();
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Configuration properties
pub struct Properties {
    /// Core properties
    #[serde(skip_serializing_if = "Option::is_none")]
    core: Option<CoreProperties>,

    /// Compute properties
    #[serde(skip_serializing_if = "Option::is_none")]
    compute: Option<ComputeProperties>,
}

impl Properties {
    /// Deserialise properties from the given reader
    pub fn from_reader<R: Read>(reader: R) -> Result<Self, Error> {
        let properties = serde_ini::de::from_read(reader)?;
        Ok(properties)
    }

    /// Serialise the properties to the given writer
    pub fn to_writer<W: Write>(&self, writer: W) -> Result<(), Error> {
        let mut ser = Serializer::new(Writer::new(writer, serde_ini::LineEnding::Linefeed));
        self.serialize(&mut ser)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Supported properties in the core section
struct CoreProperties {
    /// `core/project` setting
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<String>,

    /// `core/account` setting
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<String>,
}

impl Default for CoreProperties {
    fn default() -> Self {
        Self {
            account: None,
            project: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Supported properties in the compute section
struct ComputeProperties {
    /// `compute/zone` setting - default compute zone
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<Zone>,

    /// `compute/region` setting - default compute region
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<Region>,
}

impl Default for ComputeProperties {
    fn default() -> Self {
        Self {
            zone: None,
            region: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
/// Google Cloud Platform region
pub struct Region(String);

impl FromStr for Region {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if REGION_REGEX.is_match(s) {
            Ok(Region(s.to_owned()))
        } else {
            Err(Error::InvalidRegion(s.to_owned()))
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
/// Google Cloud Platform zone
pub struct Zone(String);

impl FromStr for Zone {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if ZONE_REGEX.is_match(s) {
            Ok(Zone(s.to_owned()))
        } else {
            Err(Error::InvalidZone(s.to_owned()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn region_from_string_valid() {
        assert!("australia-southeast1".parse::<Region>().is_ok());
        assert!("europe-west1".parse::<Region>().is_ok());
        assert!("northamerica-northeast1".parse::<Region>().is_ok());
        assert!("us-central1".parse::<Region>().is_ok());
        assert!("us-east4".parse::<Region>().is_ok());
    }

    #[test]
    fn region_from_string_invalid() {
        assert!("europe-west1-d".parse::<Region>().is_err(), "zone");

        assert!("  europe-west1".parse::<Region>().is_err(), "leading whitespace");
        assert!("europe-west1  ".parse::<Region>().is_err(), "trailing whitespace");

        assert!("europe_west1".parse::<Region>().is_err(), "invalid separator");

        assert!("-west1".parse::<Region>().is_err(), "missing continent");
        assert!("europe-1".parse::<Region>().is_err(), "missing geography - name");
        assert!("europe-west".parse::<Region>().is_err(), "missing geography - id");
        assert!("europe-".parse::<Region>().is_err(), "missing geography - missing");

        assert!("EUROPE-west1".parse::<Region>().is_err(), "invalid continent id - uppercase");
        assert!("42-west1".parse::<Region>().is_err(), "invalid continent id - numeric");
        assert!("europe-WEST1".parse::<Region>().is_err(), "invalid geography - uppercase");
        assert!("europe-421".parse::<Region>().is_err(), "invalid geography - numeric");
    }

    #[test]
    fn zone_from_string_valid() {
        assert!("australia-southeast1-c".parse::<Zone>().is_ok());
        assert!("europe-west1-d".parse::<Zone>().is_ok());
        assert!("northamerica-northeast1-b".parse::<Zone>().is_ok());
        assert!("us-central1-f".parse::<Zone>().is_ok());
        assert!("us-east4-a".parse::<Zone>().is_ok());
    }

    #[test]
    fn zone_from_string_invalid() {
        assert!("europe-west1".parse::<Zone>().is_err(), "region");

        assert!("  europe-west1-d".parse::<Zone>().is_err(), "leading whitespace");
        assert!("europe-west1-d  ".parse::<Zone>().is_err(), "trailing whitespace");

        assert!("europe_west1_d".parse::<Zone>().is_err(), "invalid separator");

        assert!("-west1-d".parse::<Zone>().is_err(), "missing continent");
        assert!("europe-1-d".parse::<Zone>().is_err(), "missing geography - name");
        assert!("europe-west-d".parse::<Zone>().is_err(), "missing geography - id");
        assert!("europe--d".parse::<Zone>().is_err(), "missing geography - missing");
        assert!("europe-west1-".parse::<Zone>().is_err(), "missing zone id");

        assert!("EU-west1-d".parse::<Zone>().is_err(), "invalid continent id - uppercase");
        assert!("42-west1-d".parse::<Zone>().is_err(), "invalid continent id - numeric");
        assert!("europe-WEST1-d".parse::<Zone>().is_err(), "invalid geography - uppercase");
        assert!("europe-421-d".parse::<Zone>().is_err(), "invalid geography - numeric");
        assert!("europe-west1-D".parse::<Zone>().is_err(), "invalid zone id - uppercase");
        assert!("europe-west1-1".parse::<Zone>().is_err(), "invalid zone id - numeric");
        assert!("europe-west1-abc".parse::<Zone>().is_err(), "invalid zone id - too long");
    }
}
