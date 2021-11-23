use std::{env, time};
use tokio::net::TcpListener;

mod config;

use config::*;
use shttpd::*;

/*
    Create async even loop to handle new connections
*/
#[tokio::main]
async fn main() {
    let startup = time::Instant::now();

    // use env args to specify different configs or options
    let envargs: Vec<String> = env::args().collect();

    let mut config: ConfigOptions = parse_config("./shttpd.conf".to_string());

    // Set defaults if no args are taken in
    if envargs.len() > 1 {
        match envargs[1].as_str() {
            "gen_config" => {
                default_config();
                println!("Created new configuration file!");
                std::process::exit(0);
            }
            "config" => {
                config = parse_config(envargs[2].to_string());
            }
            _ => println!("invalid env arg"),
        };
    }

    // Bind the listener to the address
    let listener = TcpListener::bind(get_net(&config)).await.unwrap();

    // Load main page data into struct
    let file_data: Pages = load_cache(
        &config.dir.html_dir,
        &config.dir.ignored_extensions,
        &config.dir.ignored_files,
        config.dir.max_cache_size,
    );
    println!(
        "{} Files loaded into cache in {:#?}\nstarting webserver on {}:{}",
        file_data.page_list.len(),
        startup.elapsed(),
        config.ip,
        config.port
    );

    // Start event loop
    loop {
        // The second item contains the IP and port of the new connection.
        let (stream, _ip_port) = listener.accept().await.unwrap();

        // send connection to be responded to
        process_connection(
            stream,
            file_data.page_data.clone(),
            file_data.page_list.clone(),
            file_data.full_list.clone(),
            config.dir.html_dir.clone(),
            config.options.default_page.clone(),
        )
        .await;
    }
}
