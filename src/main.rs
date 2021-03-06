use std::env;
use std::error::Error;
use std::collections::HashMap;
use reqwest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>>{
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("You must pass atleast one argument to the progam");
        //Stop the process when this happens
    }

    let arguments = args[1..].join(" ");

    let mut req_body = HashMap::new();
    req_body.insert("command", arguments);

    let client = reqwest::Client::new();
    let res = client.post("http://127.0.0.1:8888").json(&req_body).send().await?;

    match res.text().await {
        Ok(res) => println!("{}", res),
        Err(e) => println!("{:?}", e)
    }

    Ok(())
}
