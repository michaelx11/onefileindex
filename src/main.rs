extern crate chrono;
extern crate iron;
extern crate params;
extern crate router;

use chrono::prelude::*;
use iron::prelude::*;
use iron::status;
use router::Router;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::prelude::*;
use std::process::Command;
use params::{Params, Value};

fn main() {

    let mut router = Router::new();

    router.get("/", move |r: &mut Request| {
        get_lines(r)
    }, "get");

    router.post("/add", move |r: &mut Request| {
        add_line(r)
    }, "set");

    fn get_lines(req: &mut Request) -> IronResult<Response> {
        let map = req.get_ref::<Params>().unwrap();

        let search_term;

        match map.find(&["term"]) {
            Some(&Value::String(ref term)) => {
                search_term = term.clone()
            },
            _ =>  {
                search_term = String::from("");
            }
        }
        let search_string = format!("rg '{}' database.txt", search_term);
        let result = Command::new("sh")
                              .arg("-c")
                              .arg(&search_string)
                              .output()
                              .expect("Failed!");
        let stdout = String::from_utf8(result.stdout).expect("Failed to unpack valid utf-8 from stdout");

        Ok(Response::with((status::Ok, stdout)))
    }

    // Receive a message by POST and play it back.
    fn add_line(request: &mut Request) -> IronResult<Response> {
        let mut file =
            OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open("database.txt")
            .unwrap();

        let mut payload = String::new();
        request.body.read_to_string(&mut payload).unwrap();
        let line = format!("{} {}", Utc::now().to_rfc3339(), payload);
        if let Err(e) = writeln!(file, "{}", line) {
            println!("{}", e);
        }
        Ok(Response::with(status::Ok))
    }

    Iron::new(router).http("localhost:3000").unwrap();
}
