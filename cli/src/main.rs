
#[cfg(not(feature = "rand"))]
mod cli;

#[tokio::main]
async fn main() {
    #[cfg(not(feature = "rand"))]
    cli::run_cli_app().await;
    #[cfg(feature = "rand")]
    generate_collection_from_origin(1000).await;
}

#[cfg(feature = "rand")]
async fn generate_collection_from_origin(repeats: usize) {
    use rand::Rng;
    use rust_firestore_snapshot_core::firestore::seed::{CollectionData, DocumentData};
    use std::iter;
    use tokio::{fs::read_to_string, io::AsyncWriteExt};

    let filename = "data.json";
    let output_name = "test.json";
    let json_string = read_to_string(&filename)
        .await
        .expect(&format!("Could not read data from {filename}"));

    let origin: CollectionData =
        serde_json::from_str(&json_string).expect(&format!("Could not parse {filename}"));
    let documents = origin.documents;

    let mut rng = rand::thread_rng();
    let documents = documents
        .into_iter()
        .cycle()
        .take(repeats)
        .map(|document| {
            let id: String = iter::repeat(())
                .map(|()| rng.sample(rand::distributions::Alphanumeric))
                .take(20)
                .collect();
            DocumentData { id: id, ..document }
        })
        .collect::<Vec<_>>();
    let new_collection = CollectionData {
        id: "copy".into(),
        documents: documents,
    };
    let mut output = tokio::fs::File::create(output_name).await.unwrap();
    output
        .write_all(
            (serde_json::to_string_pretty(&new_collection)
                .expect("Could not convert to JSON String"))
            .as_bytes(),
        )
        .await
        .expect("Error while writing output");
}
