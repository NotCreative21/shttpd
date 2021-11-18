use tokio::net::{TcpListener};
use std::{env, time};

mod config;

use shttpd::*;
use config::*;

/*
    Create async even loop to handle new connections
*/
#[tokio::main]
async fn main() {

    let startup = time::Instant::now();

    // TODO
    // Parse config options using config.rs
    let config:ConfigOptions = parse_config();

    // TODO
    // use env args to specify different configs or options
    let envargs:Vec<String> = env::args().collect();

    // Set defaults if no args are taken in 
    if envargs.len() > 1 {
        match envargs[1].as_str() {
            "gen_config" => {
                default_config();
                println!("Created new configuration file!");
                std::process::exit(0);
            },
            _ => println!("invalid env arg") 
        };
    }

    //println!("{:?}", config.dir.html_dir);
    // Bind the listener to the address
    let listener = TcpListener::bind(get_net(&config)).await.unwrap();

    // Load main page data into struct
    let file_data:Pages = load_cache(&config.dir.html_dir);
    println!("{} Files loaded into cache in {:#?}\nstarting webserver on {}:{}", 
        file_data.page_list.len(), 
        startup.elapsed(),
        config.ip,
        config.port);

    // Start event loop
    loop {
        // The second item contains the IP and port of the new connection.
        let (stream, _ip_port) = listener.accept()
            .await
            .unwrap();
        
        // send connection to be responded to
        process_connection(stream, 
            file_data.page_data.clone(), 
            file_data.page_list.clone(), 
            config.dir.html_dir.clone(),
            config.options.default_page.clone()).await;

    }
}
