use std::fs::{self};
use tokio::net::{TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use glob::glob;


/*  Main struct to store page data, the page data is stored as a vector of bytes inside
    another vector, the page list is simply a vector of strings to make checking the 
    request easier
*/
pub struct Pages {
    pub page_data: Vec<Vec<u8>>,
    pub page_list: Vec<String>,
}


/*
    Parse the incoming TCP buffer and return the page or file it is requesting as a string
    This is done so that request that are asking for an invalid file are ignored and a 404
    page can be returned properly
*/
pub fn parse_buffer(buf:[u8; 1024], dir:&Option<String>) -> String {

    // create vec of characters to store a copy of the buffer
    let mut reqchar:Vec<char> = Vec::new(); 
    
    // iterate through the buffer and add it to the vector 
    for i in buf { 
        let newchar:char = i as char;
        
        if i > 0 {
            reqchar.push(newchar);
        }

    }

    let buffer:String = reqchar
        .iter()
        .collect::<String>();

    // find where the GET request ask for a page 
    if let Some(target) = buffer.find(" HTTP/1.1") { 

        // create a new vector to store the actual request
        let mut pagereq_char:Vec<char> = Vec::new();
        
        // iterate through the buffer and push the characters within the request
        for i in 5..target { 
            pagereq_char.push(reqchar[i]);
        }
        
        let response:String = pagereq_char
            .iter()
            .collect::<String>()
            .trim()
            .to_string();
       
        return response;
    }; 

    // TODO - check config to see if a different file or no file should be returned
    // if no page is met, return the 404 page
    format!(
        "{}/{}", 
        dir
            .as_ref()
            .unwrap()
            .to_string(), 
        "404.html"
            .to_string())

}


/*
    Main function to take in the TCP connection and process it then respond. This takes in
    the list of file data and file name, then responds correctly. It uses matching indexes
    to choose the corresponding file and file index.
*/
pub async fn process_connection(mut stream:TcpStream, file_data:Vec<Vec<u8>>, file_list:Vec<String>, dir:Option<String>, dpage:Option<String>) {  
    tokio::spawn(async move {
            
            // create main buffer to store the request in 
            let mut buffer = [0; 1024];
           
            // read the buffer into the array
            stream.read(&mut buffer).await.expect("failed to read buffer");

            let check:String = format!(
                "{}/{}", 
                dir.as_ref().expect("invalid html directory"), 
                &parse_buffer(buffer, &dir)
            );

            // iterate through the list of files and check if it exist
            let mut file_index:Option<usize> = file_list
                .iter()
                .position(|x| x.eq(&check));

            //TODO - check options for 404 page response
            // if the file does not exist, send the 404 page
            if file_index == None {  
                if dpage.as_ref().unwrap().is_empty() {
                    file_index = file_list
                        .iter()
                        .position(|x| x.eq(
                            &format!(
                                "{}/404.html", 
                                dir.as_ref()
                                .unwrap())
                            .to_string()
                        )
                    );    
                }
                
                else {
                    file_index = file_list
                        .iter()
                        .position(|x| x.eq(
                            &format!(
                                "{}/{}",
                                dir.as_ref()
                                    .unwrap(),
                                dpage.as_ref()
                                    .unwrap()
                        )
                    ))
                }
            } 

            let mut response:Vec<u8> = Vec::new();  

            // Starting content for the HTTP response 
            response.append(&mut "HTTP/1.1 200 OK\r\nContent-Length: "
                .as_bytes()
                .to_owned());

            let size:String = format!("{}\r\nserver: shttpd/0.2\r\n\r\n", 
                file_data[file_index.expect("invalid file index")].len());
           
            // Add the rest of the header for the response
            response.append(&mut size.as_bytes()
                .to_owned());
            
            // Add the page data
            response.append(&mut file_data[file_index.expect("failed to fetch page data")].clone());
            
            // Coerce the Vector of u8 into an array of bytes
            let response_bytes:&[u8] = &response;
            
            // Write/respond to the request
            if let Err(e) = stream.write_all(response_bytes).await {
                eprintln!("failed to write to socket; err = {:?}", e);

                return;
            }
        });
}


/*
    Initial loading of cache, this creates the struct that page data and page names are
    stored in and calls a function to load the data from the file system
*/
pub fn load_cache(dir:&Option<String>) -> Pages { 
    let page_sum = Pages {
        page_data: Vec::new(), 
        page_list: Vec::new(),
    };

    if dir.as_ref() == None 
        || dir.as_ref()
        .expect("failed to read html directory, please check config file")
        .len() < 1 {
        return load_directories("./web", page_sum);        
    } 
    else {
        load_directories(&dir.as_ref()
            .expect("invalid html directory"), page_sum)
    }
}


/*
    Recursively iterates through the directory marked for html to load it into the main
    page struct for responses
*/
pub fn load_directories(dir:&str, mut page_sum:Pages) -> Pages {  
    let dir_rec:String = format!(
        "{}/**/*",
        dir
    );

    // iterate through all directories and sub directories using glob
    for e in glob(&dir_rec).expect("failed to read file system") {  

        // if the file is a directory ignore adding it to the page data
        if !e.as_ref()
            .ok()
            .expect("failed to dereference file")
            .is_dir() { 

            println!("loading: {}", e
                .as_ref()
                .ok()
                .unwrap()
                .as_path()
                .to_string_lossy());
            
            // add the data as bytes
            page_sum.page_data.push(fs::read(&e
                .as_ref()
                .ok()
                .expect("failed to read file system"))
                .ok()
                .expect("failed to read bytes")
            );

            // add the name as a string
            page_sum.page_list.push(format!("./{}",
                e.ok()
                .expect("failed to push page to cache")
                .as_path()
                .to_string_lossy())
            );
        }
    }

    page_sum
}
