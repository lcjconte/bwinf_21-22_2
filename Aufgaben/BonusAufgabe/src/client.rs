use Bonusaufgabe::{conv, processing::*, math::BinomC, structs::{HashMapStore, CombStore}, io::*};
use hyper::{Client, StatusCode};
use std::{env::args};
use tokio::sync::{mpsc, Semaphore};
use hyper::{Method, Body, Request};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Arc::new(Client::new());
    let server_name = args().nth(1).unwrap();
    let jcount: usize = args().nth(2).unwrap().parse().unwrap();
    let mmultiplier: usize = args().nth(3).unwrap().parse().unwrap();
    println!("Requesting input");
    let input: TInput = get_json(client.get((server_name.to_owned()+"/tinput").parse()?).await?.into_body()).await?;
    let nums = Arc::new(input.nums.clone());
    let binomc = BinomC::default();
    let n = input.n;let k = input.k;
    let recap = binomc.binom(n/2, k/2) as usize;
    let cap = mmultiplier * 1e7 as usize;
    let rjcount = jcount.min(cap/recap);

    let (result_tx, mut result_rx) = mpsc::unbounded_channel();
    let sem = Arc::new(Semaphore::new(rjcount));
    let mserver_name = server_name.clone();
    let mclient = client.clone();
    tokio::spawn(async move {
        loop {
            let permit = sem.clone().acquire_owned().await.unwrap();
            let req = Request::builder()
                    .method(Method::POST)
                    .uri(mserver_name.to_owned()+"/get_assignment")
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_string(&ShiftRequest(0)).unwrap())).unwrap();
            let resp = mclient.request(req).await;
            if resp.is_err() {
                println!("Error getting assignment!");
                break;
            }
            let resp = resp.unwrap();
            if resp.status() != StatusCode::OK {
                break;
            }
            let assignment: ShiftAssignment = get_json(resp.into_body()).await.unwrap();
            let mresult_tx = result_tx.clone();
            let mnums = nums.clone();
            tokio::task::spawn_blocking(move || {
                let res = search_single_shift(&mnums, Segment(0, n), k, conv!(assignment.0), 0, &mut HashMapStore::new(recap));
                mresult_tx.send(res).map_err(|x| {"Couldn't pass result!"}).unwrap();
                drop(permit);
            });
        }
    });

    while let Some(r) = result_rx.recv().await {
        let req = Request::builder()
            .method(Method::POST)
            .uri(server_name.to_owned()+"/assignment_result")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&ShiftResult(0, r, 0)).unwrap())).unwrap();
        client.request(req).await.ok();
    }
    println!("Done!");
    Ok(())
}