// Evelyn: Your personal assistant, project manager and calendar
// Copyright (C) 2017 Gregory Jensen
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

#[macro_use(bson, doc)]
extern crate bson;
#[macro_use]
extern crate serde_derive;
extern crate mongodb;
extern crate serde_json;
extern crate jsonwebtoken as jwt;
extern crate chrono;
extern crate config;
extern crate uuid;
#[macro_use]
extern crate log;
extern crate log4rs;
extern crate hyper;
extern crate hyper_openssl;
extern crate unicase;
extern crate time;

mod server;
pub mod data;
mod processing;
pub mod model;
pub mod core;

use std::env;
use mongodb::{Client, ThreadedClient};
use processing::ProcessorData;
use server::http::HttpServer;
use server::routing::Router;

pub fn hello_evelyn() {
    println!("Starting...");
    println!("Server executable location: {}", std::env::current_exe().unwrap().display());
    println!("Running in directory: {}", std::env::current_dir().unwrap().display());

    // Initialise the logging back end.
    log4rs::init_file("./configs/log4rs.yml", Default::default()).unwrap();

    let mut conf_file = String::from("./configs/evelyn.json");

    // Prints each argument on a separate line
    for mut argument in env::args() {
        if argument.starts_with("-conf") {
            let pos = argument.find("=").unwrap() + 1;
            conf_file = argument.split_off(pos);
            break;
        }
    }

    let conf = data::conf::Conf::new(conf_file.as_str());
    let uri = conf.get_db_connnection_string();
    // Note this will not fail if MongoDB is not available.
    let client = match Client::with_uri(uri.as_str()) {
        Ok(client) => client,
        Err(e) => panic!("Connection to the database failed {}", e),
    };

    let token_service = core::token_service::TokenService::new(String::from("a_very_important_secret"));
    let server_session_token = token_service.create_server_session_token();

    let processor_data = ProcessorData {
        data_store: client,
        token_service: token_service,
        conf: conf,
        server_session_token: server_session_token,
    };

    let mut router = Router::new();
    processing::load_processors(&mut router);

    let http_server = HttpServer::new(router, processor_data);
    println!("Ready");
    http_server.start();
}
