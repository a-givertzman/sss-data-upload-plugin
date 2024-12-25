use api_server::*;
use data::*;
use log::info;
use parser::Parser;
use std::io;
use std::io::*;
use std::cell::RefCell;
use std::rc::Rc;

mod api_server;
mod data;
mod error;
mod parser;
mod parse_tests;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");
    let mut parser = Parser::new(
        "../sss/assets/fleet/9245263_sofia/", 
        "SSS_Sofia_",
        Rc::new(RefCell::new(ApiServer::new("sss-computing".to_owned()))),
    );
    if let Err(error) = parser.convert_data() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
 //       println!("{}", error.to_string());
        return;
    }
    if let Err(error) = parser.convert_tests() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
 //       println!("{}", error.to_string());
        return;
    }
    if let Err(error) = parser.write_to_file() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
 //       println!("{}", error.to_string());
        return;
    } 
 /*   if let Err(error) = parser.write_to_db() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
 //       println!("{}", error.to_string());
        return;
    }*/
}
