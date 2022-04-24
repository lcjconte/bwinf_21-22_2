/// Server binary | expects cmd args <input file name>
use std::convert::Infallible;
use std::env::args;
use std::net::SocketAddr;
use std::time::Instant;
use Bonusaufgabe::conv;
use Bonusaufgabe::io::*;
use Bonusaufgabe::structs::SearchRes;
use Bonusaufgabe::processing::combination_nums;
use tokio::sync::{broadcast, mpsc};
use std::sync::{Arc, Mutex};
use hyper::{Body, Request, Response, Server, Method, StatusCode};
use hyper::service::{make_service_fn, service_fn};

#[derive(Clone)]
struct Stuff {
    input: Arc<TInput>,
    cursor: Arc<Mutex<usize>>,
    searched: Arc<Mutex<Vec<bool>>>,
    result_tx: mpsc::UnboundedSender<ShiftResult>,
}


#[tokio::main]
async fn main() {
    let fname = args().nth(1).unwrap();
    println!("Serving assignments on port 3000");
    let input = Arc::new(TInput::read_from(fname).unwrap());
    let n = input.n;
    let start_time = Instant::now();
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    let (shutdown_tx, shutdown_rx) = broadcast::channel::<()>(100);
    let max_s = ((n as f64/2.0).floor()+1.0) as usize;
    let mut res_out = max_s-1;//Outstanding results
    let searched = Arc::new(Mutex::new(vec![false;res_out]));
    let cursor = Arc::new(Mutex::new(0));

    let (result_tx, mut result_rx) = mpsc::unbounded_channel::<ShiftResult>();
    let mut mshutdown_rx = shutdown_tx.subscribe();
    let mshutdown_tx = shutdown_tx.clone();
    let minput = input.clone();
    let stuff = Stuff {input: minput, cursor, searched, result_tx};

    let mstuff = stuff.clone();
    let result_handle: tokio::task::JoinHandle<SearchRes> = tokio::spawn(async move {   
        loop {
            tokio::select! {
                res = result_rx.recv() => {
                    let res = res.unwrap();
                    res_out -= 1;
                    mstuff.searched.lock().unwrap()[res.1 as usize] = true;
                    match res.0 {
                        Some(c) => {
                            mshutdown_tx.send(()).ok();
                            return Some(c);
                        },
                        None => {}
                    }
                    if res_out == 0 {
                        mshutdown_tx.send(()).ok();
                    }
                },
                _ = mshutdown_rx.recv() => {return None;}
            }
        }
    });
    let make_svc = make_service_fn(move |_conn| {
        let stuff = stuff.clone();
        async move {
            Ok::<_, Infallible>(service_fn(move |req: Request<Body>| {
                let stuff = stuff.clone();
                serve_routes(req, stuff)
            }))
        }
    });
    let mut mshutdown_rx = shutdown_tx.subscribe();
    let shutdown_signal = async move {
        mshutdown_rx.recv().await.unwrap()
    };
    let server = Server::bind(&addr).serve(make_svc).with_graceful_shutdown(shutdown_signal);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
    let res = result_handle.await.unwrap();
    let minput = (*input).clone();
    let res = if let Some(c) = res {
        let v = combination_nums(&minput.nums, &c);
        let output = TOutput {input: minput, nums: v, runtime: start_time.elapsed().as_millis()};
        assert!(output.verify());
        Some(output)
    }
    else {
        None
    };
    println!("{}", res.is_some());
    println!("Finished!");
    println!("Took: {}", start_time.elapsed().as_millis());
}

async fn serve_routes(mut _req: Request<Body>, stuff: Stuff) -> Result<Response<Body>, Infallible> {
    let mut response = Response::new(Body::empty());
    let input = stuff.input;
    match (_req.method(), _req.uri().path()) {
        (&Method::GET, "/tinput") => {
            println!("Task input requested");
            *response.body_mut() = Body::from(serde_json::to_string(&(*input)).unwrap());
        },
        (&Method::GET, "/get_assignment") => { //Does nothing with request!
            println!("Assignment requested");
            let searched = stuff.searched.lock().unwrap();
            let mut cpos = stuff.cursor.lock().unwrap();

            let mut checked = 0;
            while checked <= searched.len() {
                *cpos = (*cpos+1)%searched.len();
                if !searched[*cpos] {
                    break;
                }
                checked += 1;
            }
            match checked > searched.len() {
                true => {*response.status_mut() = StatusCode::GONE;}
                false => {
                    *response.body_mut() = Body::from(serde_json::to_string(&ShiftAssignment(conv!(*cpos))).unwrap());
                },
            }
        },
        (&Method::POST, "/assignment_result") => {
            println!("Assignment result submitted");
            stuff.result_tx.send(get_json(_req.into_body()).await.unwrap()).ok();
        },
        _ => {
            *response.status_mut() = StatusCode::NOT_FOUND;
        }
    }
    Ok(response)
}