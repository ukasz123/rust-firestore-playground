use rust_firestore_snapshot_core::firestore::FirestoreConnection;

use std::{
    fs::{read_to_string, File},
    io::Write, collections::HashMap,
};

use anyhow::Result;
use rust_firestore_snapshot_core::firestore::{
    collect::collect_collection,
    get_client,
    seed::{seed_collection, CollectionData},
};

// pub fn init(project_id: String, token: String) -> *mut c_void {
//     block_on(async {
//         let (client, project_id) = (
//             get_client(&token)
//                 .await
//                 .expect("Could not connect to Firestore"),
//             &project_id,
//         );
//         let parent = format!("projects/{}/databases/(default)/documents", project_id);
//         let firestore_conn = FirestoreConnection(client, parent);
//         Box::into_raw(Box::new(firestore_conn)) as *mut c_void
//     })
// }
#[tokio::main()]
pub async fn get_collection(
    project_id: String,
    token: String,
    collection_path: String,
    output_path: String,
) -> Result<()> {
    let firestore_connection = obtain_connection(project_id, token).await;
    let filename = output_path;
    let path = format!("{}{}", firestore_connection.1, collection_path);
    let collection = collect_collection(firestore_connection, path.to_string())
        .await
        .unwrap();
    let json_string =
        serde_json::to_string_pretty(&collection).expect("The data could not be parsed");
    let mut file = File::create(filename).unwrap();
    file.write_all(json_string.into_bytes().as_slice())
        .expect("Could not write a file");
    Ok(())
}

#[tokio::main()]
pub async fn update_collection(
    project_id: String,
    token: String,
    collection_path: String,
    input_file_path: String,
) -> Result<()> {
    let firestore_connection = obtain_connection(project_id, token).await;

    let filename = input_file_path;
    let json_string = read_to_string(filename).unwrap();

    let post_body: CollectionData = serde_json::from_str(&json_string).unwrap();

    match seed_collection(firestore_connection, &post_body, &collection_path).await {
        Ok(_) => println!("Collection updated successfully"),
        Err(_error) => panic!(),
    };
    Ok(())
}

async fn obtain_connection(project_id: String, token: String) -> FirestoreConnection {
    let (client, project_id) = (
        get_client(&token)
            .await
            .expect("Could not connect to Firestore"),
        &project_id,
    );
    let parent = format!("projects/{}/databases/(default)/documents", project_id);
    let firestore_conn = FirestoreConnection(client, parent);
    firestore_conn
}
