use std::fs;
use serde_derive::Deserialize;


// TODO - pretty much no values are used atm
/*
    Declare main config options with many optional
    Defaults will be defined later on
*/
#[derive(Deserialize)]
pub struct ConfigOptions {
    ip: String,
    port: String,
    /*
    html_dir: Option<String>,
    log_dir: Option<String>,
    page404: Option<bool>,
    live_config: Option<bool>,
    live_pages: Option<bool>,
    assume_html: Option<bool>,
    logging: Option<bool>,
    */
}

// TODO - Logging options
/*
#[derive(Deserialize)]
pub struct LogOptions {
    response_time: Option<bool>,
    incoming_ip: Option<bool>,
    req_size: Option<bool>,
    req_count: Option<bool>,
    page_visits: Option<bool>
}*/


// TODO - file options
/*pub struct files*/


// TODO - parse env arg options
/*
pub fn get_config() -> ConfigOptions {

}*/


/*
    Return the network settings from the main struct, used 
    at start up to start the webserver
*/
pub fn get_net(config:ConfigOptions) -> String {
    format!("{}:{}", config.ip, config.port)
}


// TODO - use different config file if env specifies
/*
    Load config options from the config file using serde crate for TOML
*/
pub fn parse_config() -> ConfigOptions {
    let config:ConfigOptions = toml::from_str(&fs::read_to_string("./shttpd.conf")
        .expect("failed to access config"))
        .expect("config error, please check your config"); 
    println!("Starting webserver on {}:{}", config.ip, config.port);
    config
}
