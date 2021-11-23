use serde_derive::Deserialize;
use std::fs;
use std::path::PathBuf;

// TODO - pretty much no values are used atm
/*
    Declare main config options with many optional
    Defaults will be defined later on
*/
#[derive(Deserialize)]
pub struct ConfigOptions {
    pub ip: String,
    pub port: String,
    pub dir: Dir,
    pub options: Options,
    /*
    pub log_dir:Option<String>,
    page404:Option<bool>,
    live_config:Option<bool>,
    live_pages:Option<bool>,
    assume_html:Option<bool>,
    logging:Option<bool>,
    */
}

#[derive(Deserialize)]
pub struct Dir {
    pub html_dir: Option<String>,
    pub log_file: Option<String>,
    pub ignored_extensions: Option<Vec<String>>,
    pub ignored_files: Option<Vec<String>>,
    pub max_cache_size: Option<u64>,
}

#[derive(Deserialize)]
pub struct Options {
    pub default_page: Option<String>,
    pub page404: Option<bool>,
    pub live_config: Option<bool>,
    pub live_pages: Option<bool>,
    pub assume_html: Option<bool>,
}
// TODO - Logging options
/*
#[derive(Deserialize)]
pub struct LogOptions {
    response_time:Option<bool>,
    incoming_ip:Option<bool>,
    req_size:Option<bool>,
    req_count:Option<bool>,
    page_visits:Option<bool>
}*/

// TODO - parse env arg options
/*
pub fn get_config() -> ConfigOptions {

}*/

/*
    Return the network settings from the main struct, used
    at start up to start the webserver
*/
pub fn get_net(config: &ConfigOptions) -> String {
    format!("{}:{}", config.ip, config.port)
}

fn append_dir(html_dir: String, dir: String) -> String {
    format!("{}/{}", html_dir, dir)
}

// TODO - use different config file if env specifies
/*
    Load config options from the config file using serde crate for TOML
*/
pub fn parse_config(conf: String) -> ConfigOptions {
    let mut config: ConfigOptions =
        toml::from_str(&fs::read_to_string(conf).expect("failed to access config"))
            .expect("config error, please check your config");

    //assert_eq!(config.dir.html_dir, None);

    if config.dir.html_dir.as_ref().unwrap().is_empty() {
        config.dir.html_dir = Some("./web".to_string());
    }

    if config
        .options
        .default_page
        .as_ref()
        .expect("failed to read default page")
        .is_empty()
        && !check_dir(append_dir(
            config
                .dir
                .html_dir
                .as_ref()
                .expect("invalid html directory")
                .to_string(),
            config
                .options
                .default_page
                .as_ref()
                .expect("invalid default page")
                .to_string(),
        ))
    {
        eprintln!("default page does not exist: check default_page in config");
        std::process::exit(1);
    }

    if config.options.page404.expect("failed to read config")
        && !check_dir(append_dir(
            config
                .dir
                .html_dir
                .as_ref()
                .expect("invalid html directory")
                .to_string(),
            "./404.html".to_string(),
        ))
    {
        eprintln!(
            "404 page file does not exist: check page404 in config or create one in the html_dir"
        );
        std::process::exit(1);
    }
    config
}

fn check_dir(dir: String) -> bool {
    if PathBuf::from(dir).exists() {
        return true;
    }
    false
}

/*
    If the server is run with the gen_config option, this will copy the default config file
    into somewhere the program can read it from
*/
pub fn default_config() {
    fs::copy("./shttpd.conf", "shttpd.conf.old").expect("failed to backup old config");

    fs::remove_file("./shttpd.conf").expect("failed to delete current config file");

    fs::copy("./template/conf_template.conf", "new.conf")
        .expect("failed to copy default config, please ensure there is one downloaded");
}
