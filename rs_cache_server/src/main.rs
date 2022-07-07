#![deny(warnings)]
use warp::Filter;
use std::sync::Arc;
use rs_cache::Cache;
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct PostRequest {
    pub value: String,
    pub duration_secs: u32
}

macro_rules! get_handler {
    ($cache:ident) => {
        move |key: String| {
            match $cache.get(&key[..]) {
                Some(value) => value,
                None        => "Not Found".to_owned()
            }
        }
    }
}

macro_rules! post_handler {
    ($cache:ident) => {
        move |key: String, request: PostRequest| {
            $cache.set(
                &key[..],
                request.value,
                request.duration_secs
            );

            "Ok".to_owned()
        }
    }
}

#[tokio::main]
async fn main() {
    let cache = Arc::new(
        Cache::default()
    );

    let path = warp::path!("v1" / "cache" / String);

    let cache_get = cache.clone();
    let get_route = warp::get().and(
        path.map(get_handler!(cache_get))
    );

    let body = warp::body::content_length_limit(1024).and(warp::body::json());

    let cache_post = cache.clone();
    let post_route = warp::post().and(
        path.and(body).map(post_handler!(cache_post))
    );

    let routes = get_route.or(post_route);
    warp::serve(routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}