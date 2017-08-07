
extern crate futures;
extern crate reqwest;
extern crate tokio_core;

extern crate serde;
extern crate serde_json;

use std::env;
use std::io::{self, Read};
use serde_json::Value;
use reqwest::StatusCode;
use reqwest::header::ContentType;
use std::path::Path;
use std::fs::File;
use std::process::Command;

struct Params {
    user: String,
    pass: String,
    host: String,
    project: String,
}

impl Params {

    fn json_from_file(path: &str) -> Result<String, &str> {
        match File::open(path) {
            Ok(file) => {
                match serde_json::from_reader::<File, Value>(file) {
                    Ok(contents) => {
                           match contents["bugs"]["jiraIdentifier"].as_str() {
                               Some(project) => Ok(project.to_owned()),
                               None => Err("Could not find bugs.jiraIdentifier")
                           }
                    },
                    Err(_) => Err("Error reading from package.json")
                }
            },
            Err(_) => Err("Failed to open package.json")
        }
    }

    fn project_from_file<'s>() -> Result<String, &'s str> {
        let mut package_path: Option<&str> = None;
        if Path::new("package.json").exists() {
            package_path = Some("package.json");
        } else if Path::new("./config/package.json").exists() {
            package_path = Some("./config/package.json");
        }

        match package_path {
            Some(path) => Params::json_from_file(path),
            None => Err("Could not find package.json")
        }
    }

    fn new() -> Params {
        Params {
            user: env::var("JIRA_USER").expect("$JIRA_USER is not set."),
            pass: env::var("JIRA_PASS").expect("$JIRA_PASS is not set."),
            host: env::var("JIRA_HOST").expect("$JIRA_HOST is not set."),
            project: Params::project_from_file().unwrap(), 
        }
    }
}

fn main() {
    let params = Params::new();

    match fetch_top_5(&params) {
        Ok(mut resp)    => {
            match resp.status() {
                StatusCode::Ok => {

                    let mut body = String::new();

                    resp.read_to_string(&mut body).unwrap();

                    let v: Value = serde_json::from_str(&body).unwrap();
                    
                    let sep = "-------------------------------------------";
                    
                    loop {
                        println!("\n{}", sep);

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
                                println!("\n{}", sep);
                                println!("\nError: Please enter a number");
                                continue;
                            },
                        };

                        let ticket = &v["issues"][input];
                        let t_fields = &ticket["fields"];
                        let t_key = &ticket["key"].as_str().unwrap();
                        let t_summary = &t_fields["summary"].as_str().unwrap();
                        let t_desc = &t_fields["description"].as_str().unwrap();

                        println!("\n{}", sep);
                        println!("\n[{}] {}\n\nDescription:\n{}", t_key, t_summary, t_desc);
                        println!("\n{}", sep);
                        println!("\nCommands:\n\nPress Any key: Go back,\nst: Start ticket,\nq: Quit");
                       
                        let mut command = String::new();
                        io::stdin().read_line(&mut command)
                            .expect("Failed to read line");

                        match command.trim() {
                            "st"    => start_ticket(&t_key).unwrap(),
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



fn fetch_top_5(params: &Params) -> Result<reqwest::Response, reqwest::Error> {


    let mut url = String::new();
    let host: &str = &params.host;
    let user: &str = &params.user;
    let pass: &str = &params.pass;
    let project: &str = &params.project;
    let mut user_request: String =  
        "/rest/api/2/search?jql=project=".to_owned();
   
    user_request.push_str(&project);
    user_request.push_str("&fields=id,key,summary,description&maxResults=5");

    //user_request.push_str(&params.user);
    //user_request.push_str(")");
    url.push_str(&host);
    url.push_str(&user_request);

    let client = reqwest::Client::new().unwrap();
    let mut request = client.get(&url).unwrap();
    request.basic_auth(user, Some(pass));
    request.header(ContentType::json());
    request.send() 
}

fn start_ticket(key: &str) -> Result<(), &str>{
    let mut path: Option<&str> = None;
       
    if Path::new("./.git").is_dir() {
        path = Some("./");
    } else if Path::new("./config/.git").is_dir() {
        path = Some("./config/");
    }

    match path {
        Some(dir) => {

            let fetch = Command::new("git")
                .arg("fetch")
                .arg("origin")
                .current_dir(dir)
                .output();

            match fetch {
                Ok(_) => {
                    let branch_cmd = Command::new("git") 
                        .arg("checkout")
                        .arg("-b")
                        .arg(key)
                        .arg("origin/develop")
                        .current_dir(dir)
                        .output()
                        .expect("git checkout -b command failed to start");
                    
                    if branch_cmd.status.success() {
                        Ok(())
                    } else {

                        let checkout_cmd = Command::new("git")
                            .arg("checkout")
                            .arg(key)
                            .current_dir(dir)
                            .output()
                            .expect("git checkout failed to start");

                        if checkout_cmd.status.success() {
                            Ok(())
                        } else {
                            Err("Failed to create and switch to branch")
                        }
                    }
                },
                Err(_) => Err("Failed fetching origin"),
            }

        },
        None => Err("Could not find the right path to change branches")
    }
}


