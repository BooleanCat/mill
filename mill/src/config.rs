use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::path::{Path, PathBuf};
use std::{env, fs, io};

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Could not read file {}: {}", filename.display(), source))]
    ReadFile {
        filename: std::path::PathBuf,
        source: std::io::Error,
    },

    #[snafu(display("Could not parse TOML: {}", source))]
    ParseToml { source: toml::de::Error },
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(default)]
pub struct Config {
    root: PathBuf,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root: PathBuf::from("/").join("run").join("mill"),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Error> {
        match Self::home_config() {
            Err(_) => Ok(Default::default()),
            Ok(home) => match fs::read_to_string(&home).context(ReadFile { filename: home }) {
                Ok(content) => toml::from_str(&content).context(ParseToml),
                Err(Error::ReadFile {
                    filename: _,
                    source,
                }) if source.kind() == io::ErrorKind::NotFound => Ok(Default::default()),
                Err(error) => Err(error),
            },
        }
    }

    pub fn from_file<P: AsRef<Path>>(p: P) -> Result<Self, Error> {
        toml::from_str(&fs::read_to_string(p.as_ref()).context(ReadFile {
            filename: p.as_ref(),
        })?)
        .context(ParseToml {})
    }

    fn home_config() -> Result<PathBuf, env::VarError> {
        match env::var("HOME") {
            Err(error) => Err(error),
            Ok(path) => Ok(PathBuf::from(&path)
                .join(".config")
                .join("mill")
                .join("mill.toml")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;
    use serial_test::serial;
    use std::path::PathBuf;
    use std::{env, fs};

    #[test]
    fn config_from_file_reads_valus() {
        let work_dir = tempfile::tempdir().unwrap();

        let config: Config = toml::from_str("root = '/foo/bar'").unwrap();
        let config_path = PathBuf::from(work_dir.path()).join("bmillead.toml");

        fs::write(&config_path, toml::to_vec(&config).unwrap()).unwrap();

        let got = Config::from_file(&config_path).unwrap();

        assert_eq!(config, got);
    }

    #[test]
    fn config_from_file_reads_values_with_defaults() {
        let work_dir = tempfile::tempdir().unwrap();

        fs::write(work_dir.path().join("mill.toml"), "").unwrap();

        let got = Config::from_file(work_dir.path().join("mill.toml")).unwrap();
        let want: Config = toml::from_str("root = '/run/mill'").unwrap();

        assert_eq!(want, got);
    }

    #[test]
    fn config_load_loads_defaults() {
        let got = Config::load().unwrap();
        let want: Config = toml::from_str("root = '/run/mill'").unwrap();

        assert_eq!(want, got);
    }

    #[test]
    #[serial]
    fn config_load_loads_from_home() {
        // Store original value of $HOME
        let original = env::var("HOME");

        // Ensure we got home or it wasn't set
        if let Err(error) = &original {
            assert_eq!(error, &env::VarError::NotPresent);
        }

        let work_dir = tempfile::tempdir().unwrap();
        let config_dir = PathBuf::from(work_dir.path().join(".config").join("mill"));

        fs::create_dir_all(&config_dir).unwrap();

        let config: Config = toml::from_str("root = '/run/mill'").unwrap();

        fs::write(
            &config_dir.join("mill.toml"),
            toml::to_vec(&config).unwrap(),
        )
        .unwrap();

        // Update $HOME to work_dir
        env::set_var("HOME", work_dir.path());

        let got = Config::load();

        // Restore or unset $HOME
        match original {
            Ok(value) => env::set_var("HOME", value),
            Err(env::VarError::NotPresent) => env::remove_var("HOME"),
            Err(_) => unreachable!(), // Alread checked for this above
        };

        assert_eq!(got.unwrap(), config);
    }
}
