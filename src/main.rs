/**
 * data-pool is a file monitoring client that sends
 * HTTP requests to a server when files arrive on
 * the client system.
 *
 * @package   data-pool
 * @author    Matthew Cross <blacklightgfx@gmail.com>
 * @copyright 2015
 */
extern crate hyper;

use std::fs::{File, ReadDir};
use std::io::Read;
use std::thread;
use std::path::Path;
use hyper::Client;
use hyper::header::Connection;
use hyper::status::StatusCode;

/**
 * Simple HTTP client that performs POST
 * requests to a given endpoint containing information
 * about a file event occurring on the client machine.
 * All requests include a X-DATA-POOL-SECRET header containing
 * a predefined hash that can be used to verify the requests.
 */
pub struct HttpClient {
    api_secret: String,
    client: Client
}

impl HttpClient {
    fn new(secret: String) -> HttpClient {
        return HttpClient { api_secret: secret, client: Client::new() };
    }

    fn emitFileAdded(&self, path: String) {
        println!("Added {}", path);
    }

    fn emitFileRemoved(&self, path: String) {
        println!("Remove {}", path);
    }

    fn post(&mut self, url: String) -> StatusCode {
        let mut response = self.client.post(&*url)
            .header(Connection::close())
            .send().unwrap();

        return response.status;
    }
}

/**
 * FileRegistry queries the file directory and matches the
 * found files against a list of existing files. If a file
 * is added or removed an event is dispatched with the file
 * information.
 */
pub struct FileRegistry {
    path_to_watch: String,
    existing_files: Vec<String>,
}

impl FileRegistry {
    fn new(p: String) -> FileRegistry {
        return FileRegistry { path_to_watch: p, existing_files: Vec::new() };
    }

    fn check_directory(&mut self, client: &HttpClient) {
        let paths = std::fs::read_dir(&Path::new(&self.path_to_watch)).unwrap();
        for path in paths {
            let current_path = path.unwrap().path().display().to_string();
            if (!self.existing_files.contains(&current_path)) {
                self.existing_files.push(current_path.to_string());
                client.emitFileAdded(current_path);
            }
        }
    }
}

/**
 * The application starts a simple loop and checks the watch directory
 * every minute for updates.
 */
fn main() {
    let retry_timeout = 60000;
    let mut registry = FileRegistry::new(".\\samples".to_string());
    let mut client = HttpClient::new("my-secret-123".to_string());

    while (true) {
        registry.check_directory(&client);
        thread::sleep_ms(retry_timeout);
    }
}
