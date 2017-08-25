extern crate futures;
extern crate reqwest;
extern crate tokio_core;
extern crate serde;
extern crate serde_json;

use std::io::{self, Read};
use serde_json::Value;
use reqwest::StatusCode;

mod tom;
use tom::Params;

mod jira;
use jira::Jira;

fn main() {
    let params = Params::new();

    match Jira::fetch_top_5(&params) {
        Ok(mut resp)    => {
            match resp.status() {
                StatusCode::Ok => {

                    let mut body = String::new();

                    resp.read_to_string(&mut body).unwrap();

                    let v: Value = serde_json::from_str(&body).unwrap();

                    loop {
                        let mut counter = 0;

                        for issue in v["issues"].as_array().unwrap().iter() {

                            counter = counter + 1;

                            let key = issue["key"].as_str().unwrap();
                            let summary = issue["fields"]["summary"].as_str().unwrap();

                            println!("\n{}) [{}] {}", 
                                     counter, 
                                     key, 
                                     summary                            
                                    );
                        }


                        println!("Select your ticket");

                        let mut input = String::new();
                        io::stdin().read_line(&mut input)
                            .expect("Failed to read line");

                        let input = match input.trim().parse() {
                            Ok(num) => match num {
                                1 => 0,
                                2 => 1,
                                3 => 2,
                                4 => 3,
                                5 => 4,
                                _ => 0,
                            },
                            Err(_)  => {
                                println!("\nError: Please enter a number");
                                continue;
                            },
                        };

                        let ticket = &v["issues"][input];
                        let t_fields = &ticket["fields"];
                        let t_key = &ticket["key"].as_str().unwrap();
                        let t_summary = &t_fields["summary"].as_str().unwrap();
                        let t_desc = &t_fields["description"].as_str().unwrap();

                        println!("\n[{}] {}\n\nDescription:\n{}", t_key, t_summary, t_desc);
                        println!("\nCommands:\n\nPress Any key: Go back,\ns: Start ticket,\nq: Quit");

                        let mut command = String::new();
                        io::stdin().read_line(&mut command)
                            .expect("Failed to read line");

                        match command.trim() {
                            "s"    => Jira::start_ticket(&t_key).unwrap(),
                            "q"     => break,
                            _       => continue,
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



