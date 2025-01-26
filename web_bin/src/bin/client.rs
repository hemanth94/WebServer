use futures::future::join_all;
use tokio::{join, sync::mpsc};

#[tokio::main]
async fn main() -> Result<(), tokio::task::JoinError> {
    println!("Started client");
    let url = "http://127.0.0.1:8080";

    let client = reqwest::Client::new();

    let (t, mut rx) = mpsc::channel::<i32>(150);
    let (t2, mut rx2) = mpsc::channel::<String>(150);

    for i in 0..140 {
        let send = t.send(i).await;
        println!("Sending number : {}", i);
    }

    let handle = tokio::spawn(async move {
        let mut handles = Vec::new();

        loop {
            let num = rx.recv().await.unwrap();
            if num == 139 {
                break;
            }
            println!("got = {}", num);
            let url = url.to_owned() + "/number/" + &num.to_string();
            let x = client.get(url).send();
            handles.push(x);
        }
        let ees = join_all(handles).await;
        for p in ees {
            t2.send(p.unwrap().text().await.unwrap()).await;
        }
    });
    let rec_handle = tokio::spawn(async move {
        println!("Waiting to receive response");
        while let Some(i) = rx2.recv().await {
            println!("Response received from server is : {}", i);
        }
    });
    // let ss = handle.await;

    tokio::join!();
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
