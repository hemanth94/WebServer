use std::thread;

use tokio::{io::join, join, sync::mpsc};

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    println!("Started client");
    let url = "http://127.0.0.1:8080";

    let client = reqwest::Client::new();

    let (t, mut rx) = mpsc::channel::<String>(150);
    let (t2, mut rx2) = mpsc::channel::<String>(150);

    for i in 0..140 {
        let send = t.send(i.to_string()).await;
        println!("Sending number : {}", i);
    }

    let handle = tokio::spawn(async move {
        while let Some(i) = rx.recv().await {
            println!("got = {}", i);
            let url = url.to_owned() + "/number/" + &i;
            let x = client.get(url).send().await;
            t2.send(x.unwrap().text().await.unwrap()).await;
        }
    });
    let rec_handle = tokio::spawn(async move {
        println!("Waiting to receive response");
        while let Some(i) = rx2.recv().await {
            println!("Response received from server is : {}", i);
        }
    });

    join!(handle, rec_handle);
    // // // Check the response status
    // if response.status().is_success() {
    //     // Parse the response text
    //     let body = response.text().await.unwrap();
    //     println!("Response: {}", body);
    // } else {
    //     println!("Request failed with status: {}", response.status());
    // }
    Ok(())
}

async fn send_req() {}
