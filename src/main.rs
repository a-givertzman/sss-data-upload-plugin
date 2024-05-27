use std::io::*;
use std::io;
//use api_server::*;
use data::*;
//mod api_server;
use crate::error::Error;
use log::info;

mod data;
mod error;

fn main() {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    info!("starting up");

    //    let data1 = include_str!("D:\\myProgr\\projects\\rust\\sss-data-upload-plugin\\src\\bin\\example.json");
    //    let data2 = include_str!("D:\\myProgr\\projects\\rust\\sss-data-upload-plugin\\src\\bin\\result1.json");
    let data3 = include_str!("D:\\myProgr\\projects\\rust\\sss-data-upload-plugin\\src\\bin\\result2.json");
    //   let mut stdout = io::stdout().lock();
    //   stdout.write_all(data.as_bytes()).unwrap();
    //  let mut data = String::new();
    //   io::stdin().read_line(&mut data).expect("Can't read_line");
    let mut parser = Parser::new(data3.to_owned());
    if let Err(error) = parser.convert() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        return;
    }
    if let Err(error) = parser.write_to_db() {
        let mut stdout = io::stdout().lock();
        stdout.write_all(error.to_string().as_bytes()).unwrap();
        return;
    }
}
