use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use std::{net::IpAddr, time::SystemTime};

#[derive(Clone)]
pub struct Redis {
    con: ConnectionManager,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Id {
    Public(IpAddr),
    Protected(String),
}

#[derive(Debug)]
pub struct Bucket {
    pub tokens: u64,
    pub expires_at: SystemTime,
}
