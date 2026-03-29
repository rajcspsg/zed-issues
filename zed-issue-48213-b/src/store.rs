use std::{
    sync::Arc,
    time::{Duration, SystemTime},
};

use crate::cfg::{self, Bucket, Id};
use dashmap::DashMap;
use dashmap::mapref::multiple::RefMulti;
use futures::future::join_all;
use log::debug;
use tokio::sync::broadcast::Sender;

pub struct Store {
    buckets: DashMap<Id, Bucket>,
    redis: cfg::Redis,
}

// TODO: make configurable based on number of nodes to not exceed bucket size
const CHUNK_SIZE: usize = 200;
const MAX_CONCURRENCY: usize = 5;

impl Store {
    pub fn new(redis: cfg::Redis, shutdown_tx: Sender<String>) -> Arc<Self> {
        let buckets = DashMap::with_capacity(10000);
        let mut shutdown_rx = shutdown_tx.subscribe();

        let s = Arc::new(Self {
            buckets,
            redis: redis.clone(),
        });

        let s_clone = s.clone();

        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));

            loop {
                tokio::select! {
                    Ok(msg) = shutdown_rx.recv() => {
                        if msg == "Shutdown" {
                            debug!("Stop cleanup task and return leased tokens");
                            break;
                        }
                    },
                    _ = interval.tick() => {
                        if s_clone.buckets.is_empty() {
                            continue;
                        }

                        debug!("Cleanup tick");

                        let now = SystemTime::now();

                        let expired_keys: Vec<Id> = s_clone
                            .buckets
                            .iter()
                            .filter(|e: &RefMulti<Id, Bucket>| e.value().expires_at <= now)
                            .map(|e: RefMulti<Id, Bucket>| e.key().clone())
                            .collect();

                        if expired_keys.is_empty() {
                            continue;
                        }

                        debug!("Found {} expired buckets to cleanup", expired_keys.len());

                        let chunks: Vec<Vec<Id>> = expired_keys
                            .chunks(CHUNK_SIZE)
                            .map(|c: &[Id]| c.to_vec())
                            .collect();

                        let handles = chunks
                            .into_iter()
                            .take(MAX_CONCURRENCY)
                            .map(|chunks| {
                                let s_for_task = s_clone.clone();
                                async move {
                                    for key in chunks {
                                        s_for_task.buckets.remove(&key);
                                    }
                                }
                            });

                        join_all(handles).await;

                    },
                }
            }
        });

        s
    }
}

pub fn new_r(redis: cfg::Redis) {
    let buckets = DashMap::with_capacity(10000);
    let now = SystemTime::now();
    let s = Arc::new(Store {
        buckets,
        redis: redis.clone(),
    });

    let s_clone = s.clone();
    let expired_keys: Vec<Id> = s_clone
        .buckets
        .iter()
        .filter(|e: &RefMulti<Id, Bucket>| e.value().expires_at <= now)
        .map(|e: RefMulti<Id, Bucket>| e.key().clone())
        .collect();
    debug!("Found {} expired buckets to cleanup", expired_keys.len());

    let chunks: Vec<Vec<Id>> = expired_keys
        .chunks(CHUNK_SIZE)
        .map(|c: &[Id]| c.to_vec())
        .collect();

    let handles = chunks.into_iter().take(MAX_CONCURRENCY).map(|chunks| {
        let s_for_task = s_clone.clone();
        async move {
            for key in chunks {
                s_for_task.buckets.remove(&key);
            }
        }
    });
}
