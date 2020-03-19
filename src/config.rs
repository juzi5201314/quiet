use config as config_rs;
use config_rs::ConfigError;

use serde::Deserialize;
use crate::CONFIG;
use std::ops::Deref;
use parking_lot::RwLock;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub web: Option<Web>,
    pub templates: Option<Templates>,
    pub database_url: String
}

impl Config {
    fn new() -> Result<Self, ConfigError> {
        let mut conf = config_rs::Config::default();
        conf.merge(config::Environment::default()).unwrap();
        conf.merge(config::File::with_name("config"));
        conf.try_into::<Config>()
    }

    pub fn load() -> Result<Self, ConfigError> {
        if let Ok(path) = dotenv::from_filename(".env") {
            println!("Load .env file on {}.", path.to_str().unwrap());
        }
        if cfg!(feature="sqlite") && std::env::var("DATABASE_URL").is_err() {
            std::env::set_var("DATABASE_URL", "quiet.db");
        }
        Config::new()
    }

    pub fn reload() -> Result<(), ConfigError> {
        Ok(*CONFIG.write() = Self::load()?)
    }
}

macro_rules! gen_config_getter {
    ($($funcname:ident, $($field:tt),+ => $({$default:expr})? $result:ty)*) => {
        $(gen_config_getter!(a: $funcname, $($field),+ => $({$default})? $result);)*
    };

    (a: $funcname:ident, $($field:tt),+ => $result:ty) => {
        #[inline(always)]
        pub fn $funcname(&self) -> Option<&$result> {
            Some(self.$($field.as_ref()?).+)
        }
    };

    (a: $funcname:ident, $($field:tt),+ => {$default:expr} $result:ty) => {
        #[inline(always)]
        pub fn $funcname(&self) -> $result {
            #[inline(always)]
            fn inner(config: &Config) -> Option<$result> {
                Some(config.$($field.as_ref()?).+).cloned()
            }
            inner(self).unwrap_or($default)
        }
    };
}

impl Config {
    gen_config_getter! {
        listen_addr, web, bind => { "localhost:7080".to_owned() } String
        workers, web, workers => { num_cpus::get() } usize
        templates_path, templates, path => { "templates".to_owned() } String
        static_resources, templates, static_resources => { "static".to_owned() } String
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct Web {
    pub bind: Option<String>,
    pub workers: Option<usize>
}

#[derive(Deserialize, Debug, Default)]
pub struct Templates {
    pub path: Option<String>,
    pub static_resources: Option<String>
}