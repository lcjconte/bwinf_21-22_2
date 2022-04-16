use BonusAufgabe_proto::structs::SearchRes;
use hyper::{Body, body::HttpBody};
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