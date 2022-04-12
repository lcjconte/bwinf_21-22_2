use std::fmt;

use BonusAufgabe_proto::{io::TInput, structs::{SearchRes, Combination, u256}};
use hyper::{Body, body::HttpBody, Response};
use tokio::sync::{broadcast::Receiver, mpsc::Sender};

pub enum Message {
    TINPUT(TInput),
    SCHEDULE(u32),
    RESPONSE((u32, SearchRes)),
    AVAILABILITY(u32),
    SETID(u32),
}
impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A message")
    }
}

#[repr(u8)]
pub enum FLAGS {
    /// Carries task input
    INPUT = 1,
    /// How many schedules?
    SETAV = 2,
    /// One Request
    SCHED = 3,
    /// Response to request
    RESP = 4,
    /// Set id
    SETID = 5
}

pub struct Recv {
    pub resp: Response<Body>,
    pub buf: Vec<u8>,
    pub message_channel: Sender<Message>,
    pub _shutdown: bool,
}

impl Recv {
    pub fn stream(&mut self) -> &mut Body{
        self.resp.body_mut()
    }
    pub async fn listen_on(&mut self) {
        if let Some(chunk) = self.stream().data().await {
            self.buf.extend(chunk.unwrap());
            self.parse_buf().await;
        }
        else {
            self._shutdown = true;
        }
    }
    pub async fn parse_buf(&mut self) {
        if self.buf.len() < 2 {
            return;
        }
        let frame_size = u16::from_be_bytes(conv!(self.buf[0..2])) as usize;
        if self.buf.len()-2 >= frame_size {
            self.message_channel.send(decode_incoming(&self.buf[2..2+frame_size])).await.unwrap();
            self.buf.drain(0..2+frame_size);
        }
    }
    pub async fn lloop(&mut self, mut shutdown_signal: Receiver<()>) {
        while !self._shutdown {
            tokio::select! {
                _ = self.listen_on() => {},
                _ = shutdown_signal.recv() => {return; }
            }
        }
    }
}
/// Shorthand for .try_into().unwrap()
#[macro_export]
macro_rules! conv {
    ($a:expr) => {
        $a.try_into().unwrap()
    };
}


pub fn decode_incoming(b: &[u8]) -> Message {
    match unsafe { std::mem::transmute(b[0]) } {
        FLAGS::INPUT => {
            let mut pointer = 1;
            let n = conv!(u64::from_be_bytes(conv!(b[pointer..pointer+8])));
            pointer += 8;
            let k = conv!(u64::from_be_bytes(conv!(b[pointer..pointer+8])));
            pointer += 8;
            let m = conv!(u64::from_be_bytes(conv!(b[pointer..pointer+8])));
            pointer += 8;
            assert_eq!((b.len()-pointer) % 16, 0);
            let mut nums = vec![];
            for i in (pointer..b.len()).step_by(16) {
                nums.push(u128::from_be_bytes(b[i..i+16].try_into().unwrap()));
            }
            Message::TINPUT(TInput {n, k, m, nums})
        },
        FLAGS::SCHED => {
            Message::SCHEDULE(u32::from_be_bytes(conv!(b[1..1+4])))
        },
        FLAGS::RESP => {
            let mut pointer = 1;
            let shift = u32::from_be_bytes(conv!(b[2..2+4]));
            if b[pointer] == 0 {
                return Message::RESPONSE((shift, None));
            }
            pointer += 4+1;
            let x = u128::from_be_bytes(conv!(b[pointer..pointer+16]));
            pointer += 16;
            let m1 = u128::from_be_bytes(conv!(b[pointer..pointer+16]));
            pointer += 16;
            let m2 = u128::from_be_bytes(conv!(b[pointer..pointer+16]));
            Message::RESPONSE((shift, Some(Combination(x, u256(m1, m2)))))
        },
        FLAGS::SETAV => {
            Message::AVAILABILITY(u32::from_be_bytes(conv!(b[1..1+4])))
        },
        FLAGS::SETID => {
            Message::SETID(u32::from_be_bytes(conv!(b[1..1+4])))
        }
    }
}

pub fn encode_message(m: Message) -> Vec<u8> {
    let mut v = vec![];
    match m {
        Message::TINPUT(input) => {
            v.push(FLAGS::INPUT as u8);
            v.extend_from_slice(&(input.n as u64).to_be_bytes()[..]);
            v.extend_from_slice(&(input.k as u64).to_be_bytes()[..]);
            v.extend_from_slice(&(input.m as u64).to_be_bytes()[..]);
            for i in input.nums {
                v.extend_from_slice(&i.to_be_bytes()[..]);
            }
        },
        Message::SCHEDULE(w) => {
            v.push(FLAGS::SCHED as u8);
            v.extend_from_slice(&w.to_be_bytes()[..])
        },
        Message::RESPONSE(r) => {
            v.push(FLAGS::RESP as u8);
            match r.1 {
                Some(c) => {
                    v.push(1);
                    v.extend_from_slice(&r.0.to_be_bytes()[..]);
                    v.extend_from_slice(&c.0.to_be_bytes()[..]);
                    v.extend_from_slice(&c.1.0.to_be_bytes()[..]);
                    v.extend_from_slice(&c.1.1.to_be_bytes()[..]);
                },
                None => {v.push(0);v.extend_from_slice(&r.0.to_be_bytes()[..]);}
            }
        },
        Message::AVAILABILITY(a) => {
            v.push(FLAGS::SETAV as u8);
            v.extend_from_slice(&a.to_be_bytes()[..])
        },
        Message::SETID(i) => {
            v.push(FLAGS::SETID as u8);
            v.extend_from_slice(&i.to_be_bytes()[..])
        }
    }
    v
}