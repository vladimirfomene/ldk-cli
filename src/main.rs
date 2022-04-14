use std::env;
use std::error::Error;
use std::collections::HashMap;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You must pass atleast one argument to the progam");
    }

    let command = &args[1].trim();

    let mut req_body = HashMap::new();
    req_body.insert("command", command);

    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:8888").json(&req_body).send().await?;

    println!("{:#?}", res.text().await);

    Ok(())
}
