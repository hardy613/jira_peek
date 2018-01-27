extern crate serde;
extern crate serde_json;
extern crate clap;

use std::path::Path;
use std::fs::File;
use std::env;
use serde_json::Value;

pub struct Params {
    pub user: String,
    pub pass: String,
    pub host: String,
    pub project: String,
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

    pub fn new(args: clap::ArgMatches) -> Params {
        let mut parsed_user = String::new();

        if let Some(user) = args.value_of("user") {
           parsed_user = user.to_string();
        }



        println!("{:?}", parsed_user);

        Params {
            user: parsed_user,
            pass: env::var("JIRA_PASS").expect("$JIRA_PASS is not set."),
            host: env::var("JIRA_HOST").expect("$JIRA_HOST is not set."),
            project: Params::project_from_file().unwrap(), 
        }
    }
}

