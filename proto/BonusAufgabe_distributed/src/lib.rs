use std::fmt;

use BonusAufgabe_proto::{io::TInput, structs::{SearchRes, Combination, u256}};
use hyper::{Body, body::HttpBody, Response};
use tokio::sync::{broadcast::Receiver, mpsc::Sender};
use serde::{Serialize, Deserialize};

/// Request processing Assignment
#[derive(Serialize, Deserialize)]
pub struct ShiftRequest(pub u128);

/// Assignment to process shift
#[derive(Serialize, Deserialize)]
pub struct ShiftAssignment(pub u32);

/// Result of single shift processing (uuid, result, shift)
#[derive(Serialize, Deserialize)]
pub struct ShiftResult(pub u128, pub SearchRes, pub u32);

/*impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "A message")
    }
}*/

//TODO:
//Rest are http requests
//Check functional

pub async fn get_json<T: serde::de::DeserializeOwned>(mut body: Body) -> Result<T, Box<dyn std::error::Error + Send + Sync>> {
    let mut data = Vec::with_capacity(body.size_hint().lower() as usize);
    while let Some(chunk) = body.data().await {
        data.extend(&chunk?);
    }
    let re = serde_json::from_slice(&data);
    Ok(re?)
}

/// Shorthand for .try_into().unwrap()
#[macro_export]
macro_rules! conv {
    ($a:expr) => {
        $a.try_into().unwrap()
    };
}