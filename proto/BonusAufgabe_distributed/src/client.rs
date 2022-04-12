use BonusAufgabe_proto::{processing::*, math::BinomC, structs::{HashMapStore, CombStore}};
use hyper::{Client, Uri, body::HttpBody};
use std::{env::args};
use BonusAufgabe_distributed::*;
use tokio::sync::{broadcast, oneshot, mpsc, Semaphore};
use hyper::{Method, Body, Request};
use std::sync::Arc;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref MYID: Mutex<u32> = Mutex::new(0);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Arc::new(Client::new());
    let server_name = args().nth(1).unwrap();
    let jcount: usize = args().nth(2).unwrap().parse().unwrap();
    let mmultiplier: usize = args().nth(3).unwrap().parse().unwrap();

    //Get tinput
    let mut resp = client.get((server_name.clone()+"/tinput").parse()?).await?;
    let mut b = bytes::BytesMut::new();
    while let Some(chunk) = resp.body_mut().data().await {
        b.extend_from_slice(&chunk?)
    }
    let msg = decode_incoming(&b[..]);
    let input = match msg {
        Message::TINPUT(input) => input,
        _ => {panic!("AAAAhhh");}
    };
    
    let nums = Arc::new(input.nums.clone());
    let binomc = BinomC::default();
    let n = input.n;let k = input.k;
    let recap = binomc.binom(n/2, k/2) as usize;
    let cap = mmultiplier * 1e7 as usize;
    let rjcount = jcount.min(cap/recap);
    //Register
    let (tx, mut incoming_messages) = mpsc::channel(16);
    let (stx, srx) = broadcast::channel(16);
    let req = Request::builder()
        .method(Method::POST)
        .uri((server_name.clone()+"/listen").parse::<Uri>()?)
        .header("content-type", "text/plain")
        .body(Body::from(rjcount.to_string()))?;
    let resp = client.request(req).await?;
    let mut re = Recv {resp, buf: vec![], message_channel: tx, _shutdown: false};
    let s2 = stx.subscribe();
    tokio::spawn(async move {
        re.lloop(s2).await
    });
    println!("Registered listener");

    let mut handles = vec![];
    let remaining = Arc::new(Semaphore::new(rjcount));
    let (result_send, mut result_rcv) = mpsc::channel(100);
    let myclient = client.clone();
    tokio::spawn(async move {
        loop {
            let nresult = result_rcv.recv().await.unwrap();
            let req = Request::builder()
                        .method(Method::POST)
                        .uri((server_name.clone()+"/new_result").parse::<Uri>().unwrap())
                        .header("content-type", "application/octet-stream")
                        .body(Body::from(encode_message(Message::RESPONSE(nresult)))).unwrap();
            myclient.request(req).await.unwrap();
        }
    });
    loop {
        let nreq = incoming_messages.recv().await.unwrap();
        match nreq {
            Message::SCHEDULE(s) => {
                println!("Got schedule: {}", s);
                let cnums = nums.clone();
                let mut store = HashMapStore::new(recap);
                let permit = remaining.clone().acquire_owned().await.unwrap();
                let my_res_send = result_send.clone();
                let handle = tokio::task::spawn_blocking(move || {
                    let theid = *MYID.lock().unwrap();
                    let res = (theid, search_single_shift(&cnums, Segment(0, n), k+1, s as usize, 0, &mut store));
                    if res.1.is_some() {
                        println!("Found it");
                    }
                    my_res_send.blocking_send(res).map_err(|x|{"Oh no"}).unwrap();
                    println!("Sent result for {}", s);
                    drop(permit)
                });
                handles.push(handle);
            },
            Message::SETID(i) => {
                println!("Got id: {}", i);
                *MYID.lock().unwrap() = conv!(i);
            }
            _ => {panic!("Received invalid message!")}
        }
    }
    Ok(())
}