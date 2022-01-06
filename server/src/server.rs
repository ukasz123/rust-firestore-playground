

use futures::future::TryFutureExt;
use futures::{try_join, StreamExt};
use rust_firestore_snapshot_core::firestore::collect::collect_collection;
use rust_firestore_snapshot_core::firestore::seed::{seed_collection, CollectionData};
use rust_firestore_snapshot_core::firestore::{BoxError, FirestoreClient, FirestoreConnection};

use std::{convert::Infallible, env};
use std::{net::SocketAddr};

use hyper::{
    service::{make_service_fn, service_fn},
    Method,
};

use hyper::{Body, Request, Response, Server, StatusCode};

mod compute_metadata;
pub async fn run_http_server() {
    // setup connection to Firestore
    let (client, project_id) =
        try_join!(get_client_with_fallback(), get_project_id_with_fallback(),)
            .expect("Could not connect to Firestore. Make sure the environment variables ");
    let parent = format!("projects/{}/databases/(default)/documents", project_id);
    let firestore_conn = FirestoreConnection(client, parent);

    let addr = SocketAddr::from(([0, 0, 0, 0], get_port()));
    let make_svc = make_service_fn(move |_conn| {
        let firestore_conn = firestore_conn.clone();
        let service = service_fn(move |req| {
            let future = root(firestore_conn.clone(), req);
            let future = future.or_else(|problem| async move {
                // basic error handling...
                let body = hyper::Body::from(format!("{}", problem));
                let response = Response::builder()
                    .status(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(body)
                    .unwrap();
                Ok::<Response<Body>, BoxError>(response)
            });
            future
        });

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }
    });
    let server = Server::bind(&addr).serve(make_svc);
    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

async fn root(
    firestore_conn: FirestoreConnection,
    req: Request<Body>,
) -> Result<Response<Body>, BoxError> {
    let res = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => String::from(
            r#"
            Usage:
            GET (/{path_to_collection}) - returns a JSON file containing data of the collection
            POST (/{path_to_collection}) - updates the collection with data from JSON passed as a body of request
            "#,
        ),
        (&Method::GET, path) => {
            println!("GET {}", path);
            let path = format!("{}{}", firestore_conn.1, path);
            let collection = collect_collection(firestore_conn, path.to_string()).await?;
            serde_json::to_string_pretty(&collection)?
        }
        // (&Method::POST, _) => {
        //     let r = post_greeting(firestore_conn, req).await;
        //     r.unwrap_or(String::from_str("~~ error happened~~").expect("Unable to unwrap string"))
        // }
        (&Method::POST, _) => {
            update_collection(firestore_conn, req).await?;
            String::from("")
        }
        _ => "Unrecognizable command".to_string(),
    };

    Ok(Response::new(res.into()))
}

async fn update_collection(
    firestore_conn: FirestoreConnection,
    mut req: Request<Body>,
) -> Result<(), BoxError> {
    println!("Updating collection...");
    // asynchronously concatenate chunks of the body
    let mut body = Vec::new();
    while let Some(chunk) = req.body_mut().next().await {
        body.extend_from_slice(&chunk?);
    }
    println!("received {} bytes", body.len());
    // try to parse as json with serde_json
    let post_body: CollectionData = serde_json::from_slice(&body)?;

    println!("parsed body:\n{:?} ", post_body);

    let collection_path = req.uri().path();
    println!("seeding collection at {collection_path}");
    seed_collection(firestore_conn, &post_body, &collection_path)
        .await
        .map_err(|err| err.into()).map(|_|())
}

fn get_port() -> u16 {
    std::env::var("PORT")
        .ok()
        .and_then(|x| x.parse().ok())
        .unwrap_or(8080)
}

async fn get_client_with_fallback() -> Result<FirestoreClient, BoxError> {
    get_remote_client()
        .or_else(|_| async { get_local_client().await })
        .await
}

async fn get_project_id_with_fallback() -> Result<String, BoxError> {
    let d = compute_metadata::get_project_id()
        .or_else(|_| async { get_project_id().await })
        .await?;
    println!("project id = {}", d);
    Ok(d)
}

// copy pasted from
async fn get_remote_client() -> Result<FirestoreClient, BoxError> {
    let token = compute_metadata::get_token().await?;
    rust_firestore_snapshot_core::firestore::get_client(&token).await
}

async fn get_local_client() -> Result<FirestoreClient, BoxError> {
    let token = get_token();
    rust_firestore_snapshot_core::firestore::get_client(&token).await
}

fn get_token() -> String {
    let token = env::var("TOKEN").expect("TOKEN env variable has not been set. Please run 'export TOKEN=`gcloud auth print-access-token`' before starting a server.");
    token
}

async fn get_project_id() -> Result<String, BoxError> {
    Ok(std::env::var("PROJECT_ID").map_err(Box::new)?)
}
