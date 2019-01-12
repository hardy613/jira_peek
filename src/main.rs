extern crate futures;
extern crate reqwest;
extern crate tokio_core;
#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_json;

use std::io::{self, Read};
use serde_json::Value;
use reqwest::StatusCode;
mod params;
use params::Params;

mod jira;

fn main() {
    let args = clap_app!(jira_peek =>
                         (version: "0.1.2")
                         (author: "Scott Hardy <hardy613+jira_peek@gmail.com>")
                         (about: "Look at Jira tickets")
                         (@arg user: --user +takes_value "Jira username")
                         (@arg password: --password +takes_value "Jira password")
                         (@arg project: -p --project +takes_value "Jira project indentifier")
                         (@arg ticket: -t --ticket +takes_value "Jira ticket number")
                         (@arg host: -h --host +takes_value "Jira URL")
                        ).get_matches();

    let params = Params::new(args);

    match jira::fetch_top_5(&params) {
        Ok(mut resp)    => {
            match resp.status() {
                StatusCode::Ok => {

                    let mut body = String::new();

                    resp.read_to_string(&mut body).unwrap();

                    let v: Value = serde_json::from_str(&body).unwrap();

                    loop {
                        let mut counter: usize = 0;

                        for issue in v["issues"].as_array().unwrap().iter() {

                            counter += 1;

                            let key = issue["key"].as_str().unwrap();
                            let summary = issue["fields"]["summary"].as_str()
                                .unwrap();

                            println!("\n{}) [{}] {}", 
                                     counter, key, summary);
                        }

                        println!("Select your ticket");

                        let mut input = String::new();
                        io::stdin().read_line(&mut input)
                            .expect("Failed to read line");

                        let parsed_input = match input.trim().parse::<usize>() {
                            Ok(num) => {
                                if num <= 0 {
                                    println!("Number is too low");
                                    continue;
                                } else if num - 1 >= counter {
                                    println!("Number is too high");
                                    continue;
                                }
                                num - 1
                            }
                            _ => {
                                println!("Numbers only");
                                continue;
                            }
                        };

                        let ticket = &v["issues"][parsed_input];
                        let t_fields = &ticket["fields"];
                        let t_key = &ticket["key"].as_str().unwrap();
                        let t_summary = &t_fields["summary"].as_str().unwrap();
                        let t_desc = &t_fields["description"].as_str().unwrap();

                        println!("\n[{}] {}\n\nDescription:\n{}",
                                 t_key, t_summary, t_desc);
                        println!("\nCommands:\nPress Any key: Go back"); 
                        println!("\ns: Start ticket\nq: Quit");

                        let mut command = String::new();
                        io::stdin().read_line(&mut command)
                            .expect("Failed to read line");

                        match command.trim() {
                            "s" => jira::start_ticket(&t_key).unwrap(),
                            "q" => break,
                            _   => continue,
                        };

                        break;
                    }

                },
                code => {
                    let mut text = String::new();
                    resp.read_to_string(&mut text).unwrap();
                    println!("Error: {:?}", code);
                    println!("response: {:?}", text);
                }
            }

        },
        Err(error)      => {
            println!("Error: {}", error);
        }
    };
}



