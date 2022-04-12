use std::convert::Infallible;
use std::env::args;
use std::io::Read;
use std::net::SocketAddr;
use std::time::Duration;
use BonusAufgabe_distributed::*;
use BonusAufgabe_proto::io::TInput;
use bytes::{Bytes, BytesMut};
use hyper::body::{Sender, HttpBody};
use tokio::sync::Mutex;
use std::sync::Arc;
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

lazy_static::lazy_static! {
    static ref rsend: Mutex<Option<tokio::sync::mpsc::Sender<usize>>> = Mutex::new(None);
}

fn to_frame(chunk: Bytes) -> Bytes {
    let mut b = BytesMut::from(&(chunk.len() as u16).to_be_bytes()[..]);
    println!("{:?}", &b);
    b.extend_from_slice(&chunk);
    Bytes::from(b)
}


#[tokio::main]
async fn main() {
    let fname = args().nth(1).unwrap();
    let input = Arc::new(TInput::read_from(fname).unwrap());
    let n = input.n;
    let senders = Arc::new(Mutex::new(vec![]));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let (ready_send, mut ready_core) = tokio::sync::mpsc::channel(100);
    *rsend.lock().await = Some(ready_send);
    let my_senders = senders.clone();
    let make_svc = make_service_fn(move |_conn| {
        let input = input.clone();
        let senders = senders.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let input = input.clone();
                let senders = senders.clone();
                hello_world(req, input, senders)
            }))
        }
    });

    let server = Server::bind(&addr).serve(make_svc);
    tokio::spawn(async move {
        for s_point in 0..(((n as f64/2.0).floor()+1.0) as usize) {
            let uuid: usize = ready_core.recv().await.unwrap();
            println!("Sending sched: {}", s_point);
            my_senders.lock().await[uuid].0.send_data(to_frame(Bytes::from(encode_message(Message::SCHEDULE(conv!(s_point)))))).await.unwrap();
        }
    });
    

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn hello_world(mut _req: Request<Body>, input: Arc<TInput>, senders: Arc<Mutex<Vec<(Sender, usize, usize)>>>) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());

    match (_req.method(), _req.uri().path()) {
        (&Method::GET, "/tinput") => {
            println!("Got tinput");
            *response.body_mut() = Body::from(encode_message(Message::TINPUT((*input).clone())))
        },
        (&Method::POST, "/listen") => {
            println!("Got listen");
            let jcount: usize = std::str::from_utf8(&hyper::body::to_bytes(_req.into_body()).await.unwrap()).unwrap().parse().unwrap();
            let (mut sender, body) = Body::channel();
            let mut senders = senders.lock().await;
            let slen = senders.len();
            let rsender = rsend.lock().await;
            for _ in 0..jcount {
                rsender.as_ref().unwrap().send(slen).await.unwrap();
            }
            sender.send_data(to_frame(Bytes::from(encode_message(Message::SETID(conv!(slen)))))).await.unwrap();
            senders.push((sender, slen, jcount));
            response.headers_mut().append("Content-Type", "text/eventstream".try_into().unwrap());
            *response.body_mut() = body;
        },
        (&Method::POST, "/new_result") => {
            println!("Got result");
            let mut b = bytes::BytesMut::new();
            while let Some(chunk) = _req.body_mut().data().await {
                b.extend_from_slice(&chunk.unwrap());
            }
            let res = match decode_incoming(&b) {
                Message::RESPONSE(res) => res,
                _ => {panic!("Wrong message");}
            };
            rsend.lock().await.as_ref().unwrap().send(res.0 as usize).await.ok();
            println!("{:?}", res.1.is_none());
            if let Some(c) = res.1 {
                println!("Found it");
                std::process::exit(0);
            }
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }
    Ok(response)
}