use crate::Error;
use serde::{Deserialize, Serialize};
use serde_ini::{Serializer, Writer};
use std::io::{Read, Write};

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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Supported properties in the core section
struct CoreProperties {
    /// `core/project` setting
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<String>,

    /// `core/account` setting
    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Supported properties in the compute section
struct ComputeProperties {
    /// `compute/zone` setting - default compute zone
    #[serde(skip_serializing_if = "Option::is_none")]
    zone: Option<String>,

    /// `compute/region` setting - default compute region
    #[serde(skip_serializing_if = "Option::is_none")]
    region: Option<String>,
}

#[derive(Debug, Default)]
/// Properties builder
pub struct PropertiesBuilder {
    /// core/project setting
    project: Option<String>,

    /// core/account setting
    account: Option<String>,

    /// compute/zone setting
    zone: Option<String>,

    /// compute/region setting
    region: Option<String>,
}

impl PropertiesBuilder {
    /// Build the properties
    #[must_use]
    pub fn build(&self) -> Properties {
        let core = if self.project.is_some() || self.account.is_some() {
            Some(CoreProperties {
                project: self.project.clone(),
                account: self.account.clone(),
            })
        } else {
            None
        };

        let compute = if self.zone.is_some() || self.region.is_some() {
            Some(ComputeProperties {
                zone: self.zone.clone(),
                region: self.region.clone(),
            })
        } else {
            None
        };

        Properties { core, compute }
    }

    /// Set the project property
    pub fn project(&mut self, project: &str) -> &mut Self {
        self.project = Some(project.to_owned());
        self
    }

    /// Set the account property
    pub fn account(&mut self, account: &str) -> &mut Self {
        self.account = Some(account.to_owned());
        self
    }

    /// Set the zone property
    pub fn zone(&mut self, zone: &str) -> &mut Self {
        self.zone = Some(zone.to_owned());
        self
    }

    /// Set the region property
    pub fn region(&mut self, region: &str) -> &mut Self {
        self.region = Some(region.to_owned());
        self
    }
}
