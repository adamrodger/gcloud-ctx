use assert_cmd::Command;
use assert_fs::{prelude::*, TempDir};

const CLOUDSDK_CONFIG: &'static str = "CLOUDSDK_CONFIG";

pub struct TempConfigurationStore {
    active: Option<String>,
    configs: Vec<String>,
}

impl TempConfigurationStore {
    pub fn new() -> Result<Self, anyhow::Error> {
        Ok(Self {
            active: None,
            configs: Vec::new(),
        })
    }

    pub fn build(self) -> Result<(Command, TempDir), anyhow::Error> {
        let dir = TempDir::new()?;

        std::fs::create_dir(dir.path().join("configurations"))?;

        let mut command = Command::cargo_bin("gctx")?;
        command.env(CLOUDSDK_CONFIG, dir.path());

        if let Some(active) = &self.active {
            dir.child("active_config").write_str(active)?;
        }

        self.configs
            .iter()
            .map(|name| format!("configurations/config_{}", name))
            .map(|config| dir.child(config).touch())
            .collect::<Result<(), _>>()?;

        Ok((command, dir))
    }

    pub fn with_config_activated(mut self, name: &str) -> Self {
        self.active = Some(name.to_owned());
        self.configs.push(name.to_owned());
        self
    }

    pub fn with_config(mut self, name: &str) -> Self {
        self.configs.push(name.to_owned());
        self
    }
}
