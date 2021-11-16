use tokio::net::{TcpListener};
use std::env;

mod config;

use shttpd::*;
use config::*;

/*
    Create async even loop to handle new connections
*/
#[tokio::main]
async fn main() {

    // TODO
    // Parse config options using config.rs
    let config:ConfigOptions = parse_config();

    // TODO
    // use env args to specify different configs or options
    let envargs:Vec<String> = env::args().collect();

    // TODO
    // Set defaults if no args are taken in
    /*
    if envargs.len() > 1 {

    }*/

    // Bind the listener to the address
    let listener = TcpListener::bind(get_net(config)).await.unwrap();

    // Load main page data into struct
    let file_data:Pages = load_cache();
    println!("{} Files loaded into cache", file_data.page_list.len());

    // Start event loop
    loop {
        // The second item contains the IP and port of the new connection.
        let (stream, _ip_port) = listener.accept()
            .await
            .unwrap();
        
        // send connection to be responded to
        process_connection(stream, file_data.page_data.clone(), file_data.page_list.clone()).await;
    }
}
