use clap::*;
use tokio::{
    fs::{read_to_string, File},
    io::AsyncWriteExt,
};

use rust_firestore_snapshot_core::firestore::{
    collect::collect_collection,
    get_client,
    seed::{seed_collection, CollectionData},
    FirestoreConnection,
};

/// Simple program to get/update a collection
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct CliArgs {
    /// What mode to run the program in
    #[clap(short, long, arg_enum)]
    mode: Mode,

    /// Path to the collection in Firestore
    /// Required in `get` mode.
    collection: Option<String>,

    /// Path to the parent document for the collection in Firestore.
    ///
    /// Collection from the JSON file would be saved as a subcollection of a document found on this path.
    /// Required in `post` mode.
    parent_document: Option<String>,

    /// Path to the file
    file: Option<String>,

    /// The Firebase project id
    #[clap(short, long)]
    project_id: String,

    /// The Firebase Auth Access Token
    /// It can be obtained by calling `gcloud auth print-access-token`
    #[clap(short, long)]
    token: String,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum, Debug)]
enum Mode {
    GET,
    POST,
}

pub async fn run_cli_app() {
    let args = CliArgs::parse();

    // setup connection to Firestore
    let (client, project_id) = (
        get_client(&args.token)
            .await
            .expect("Could not connect to Firestore"),
        args.project_id,
    );
    let parent = format!("projects/{}/databases/(default)/documents", project_id);
    let firestore_conn = FirestoreConnection(client, parent);
    let filename = args.file.unwrap_or(String::from("data.json"));

    match args.mode {
        Mode::GET => {
            let path = format!(
                "{}{}",
                firestore_conn.1,
                args.collection
                    .expect("`collection` is required in `get` mode.")
            );
            let collection = collect_collection(firestore_conn, path.to_string())
                .await
                .unwrap();
            let json_string =
                serde_json::to_string_pretty(&collection).expect("The data could not be parsed");
            let mut file = File::create(filename).await.unwrap();
            file.write_all(json_string.into_bytes().as_slice())
                .await
                .expect("Could not write a file");
        }
        Mode::POST => {
            let json_string = read_to_string(&filename)
                .await
                .expect(&format!("Could not read data from {filename}"));

            let post_body: CollectionData =
                serde_json::from_str(&json_string).expect(&format!("Could not parse {filename}"));

            let parent_path = args
                .parent_document
                .expect("`parent_document` is required in `post` mode.");

            match seed_collection(firestore_conn, &post_body, &parent_path).await {
                Ok(count) => println!("Collection updated successfully. {count} records written."),
                Err(error) => panic!(
                    "Error while trying to seed a collection for {}: {}",
                    &parent_path, error
                ),
            };
        }
    }
}
