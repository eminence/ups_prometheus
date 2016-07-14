extern crate hyper;

use std::io::Write;

use std::process::Command;
use hyper::server::{Server, Request, Response};
use hyper::header::ContentLength;

fn get(line: &str) -> (String, String) {
    let mut s = line.split(":");
    let a = s.next().expect("Can't get first elem");
    let b = s.next().expect("Can't get second elem");
    (a.trim().to_owned(), b.trim().to_owned())
}

fn get_status(body: &mut String, machine: &str) {
    let output = Command::new("upsc").arg(&format!("UPS@{}", machine)).output().expect("Failed to run upsc command");
    let ups_output = std::str::from_utf8(&output.stdout).unwrap();


    for line in ups_output.lines() {
        if line.starts_with("battery.charge:") ||
           line.starts_with("battery.runtime:") ||
           line.starts_with("battery.runtime:") ||
           line.starts_with("input.voltage:") ||
           line.starts_with("input.frequency:") ||
           line.starts_with("output.voltage:") {
               let (k, v) = get(line);
               let key = k.replace(".", "_");
               body.push_str(&format!("ups_{}{{machine=\"{}\"}} {}\n", key, machine, v))
           }

    }

}

fn metrics(req: Request, mut res: Response) {
    let mut body = String::new();
    get_status(&mut body, "bigbox");
    get_status(&mut body, "freenas");
    get_status(&mut body, "rtr");

    res.headers_mut().set(ContentLength(body.len() as u64));
    let mut res = res.start().unwrap();
    res.write_all(body.as_bytes()).unwrap();
    

}



fn main() {
    Server::http("0.0.0.0:9101").unwrap().handle(metrics).unwrap();
}
