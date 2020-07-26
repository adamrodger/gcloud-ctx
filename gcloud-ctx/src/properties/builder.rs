use super::*;

#[derive(Debug)]
/// Properties builder
pub struct PropertiesBuilder {
    /// core/project setting
    project: Option<String>,

    /// core/account setting
    account: Option<String>,

    /// compute/zone setting
    zone: Option<Zone>,

    /// compute/region setting
    region: Option<Region>,
}

impl Default for PropertiesBuilder {
    fn default() -> Self {
        Self {
            project: None,
            account: None,
            zone: None,
            region: None,
        }
    }
}

impl PropertiesBuilder {
    /// Build the properties
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
    pub fn zone(&mut self, zone: Zone) -> &mut Self {
        self.zone = Some(zone);
        self
    }

    /// Set the region property
    pub fn region(&mut self, region: Region) -> &mut Self {
        self.region = Some(region);
        self
    }
}
