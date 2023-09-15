use async_std::task;
use serde_json::Value;
use std::{error::Error, time::Instant};

async fn async_get_coins_ids(url: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let body = reqwest::get(url).await?.json::<Value>().await?;
    let mut ids = Vec::new();
    if let Value::Array(list) = body {
        list.into_iter().for_each(|mut item| {
            if let Value::String(id) = item["id"].take() {
                ids.push(id);
            }
        });
    }
    println!("Get ids ok");
    Ok(ids)
}

async fn async_get_coins_data(ids: Vec<String>) -> Result<Vec<Value>, Box<dyn Error>> {
    let mut tasks = Vec::new();
    for id in ids {
        let url = format!(
            "https://api.coingecko.com/api/v3/simple/price?ids={}&vs_currencies=usd",
            id
        );
        tasks.push(task::spawn(async move {
            println!("Making request for coin: {}", id);
            let body = reqwest::get(&url).await?.json::<Value>().await;
            println!("Request for coin: {} ok", id);
            body
        }));
    }
    let mut data = Vec::new();
    for task in tasks {
        data.push(task.await?);
    }
    Ok(data)
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    let body = task::block_on(async_get_coins_ids(
        "https://api.coingecko.com/api/v3/coins/list",
    ))?;
    let data = task::block_on(async_get_coins_data(body[300..400].to_vec()))?;
    println!("{:?}", data[0]);
    println!("Time elapsed: {:?}", start.elapsed());
    Ok(())
}
