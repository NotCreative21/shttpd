## shttpd

Please note: This is very incomplete and messy

shttpd is a extremely simple webserver that I've made in Rust. 
Unfortunately, I'm quite new to Rust so my code is not very good. 
Any suggestions for improvements or features would be appreciated.

The goal of this project for me is to learn more about Rust and systems level programming while also building a functional webserver. The webserver is intended to be small and fast and not be used for a 'general purpose' website.

### current features:
* file caching in memory for fast response
* optimized response to GET request to increase performance
* default 404 page for bad request

### planned features
* reloading file cache of fs update
* logging options
* more file caching options
* assume file extension when visiting urls
* possible dash board

There are many better projects that do this as well, but writing my own code is fun
