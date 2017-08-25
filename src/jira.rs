extern crate futures;
extern crate reqwest;
use std::process::Command;
use std::path::Path;
use reqwest::header::ContentType;
use tom::Params;

pub struct Jira {}

impl Jira {
    pub fn fetch_top_5(params: &Params) -> Result<reqwest::Response, reqwest::Error> {
        let mut url = String::new();
        let host: &str = &params.host;
        let user: &str = &params.user;
        let pass: &str = &params.pass;
        let project: &str = &params.project;
        let mut user_request: String =  
            "/rest/api/2/search?jql=assignee=".to_owned();
        user_request.push_str(&user);
        user_request.push_str(" AND project=");
        user_request.push_str(&project);
        user_request.push_str(" AND type=sub-task AND status='to do' ORDER BY priority");
        user_request.push_str("&fields=id,key,summary,description&maxResults=5");

        url.push_str(&host);
        url.push_str(&user_request);

        let client = reqwest::Client::new().unwrap();
        let mut request = client.get(&url).unwrap();
        request.basic_auth(user, Some(pass));
        request.header(ContentType::json());
        request.send()
    }

    pub fn start_ticket(key: &str) -> Result<(), &str>{
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
                            println!("New branch {} created", key);
                            Ok(())
                        } else {

                            let checkout_cmd = Command::new("git")
                                .arg("checkout")
                                .arg(key)
                                .current_dir(dir)
                                .output()
                                .expect("git checkout failed to start");

                            if checkout_cmd.status.success() {
                                println!("Changed to branch {}", key);
                                Ok(())
                            } else {
                                println!("Failed to create and switch to branch");
                                Err("Commit or stash your changes and try again.")
                            }
                        }
                    },
                    Err(_) => Err("Failed fetching origin"),
                }

            },
            None => Err("Could not find the right path to change branches")
        }
    }


}

