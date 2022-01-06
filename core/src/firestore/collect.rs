use std::iter::FromIterator;

use firestore_grpc::v1::{Document, ListCollectionIdsRequest, ListDocumentsRequest};

use super::{BoxError, FirestoreConnection};
use async_recursion::async_recursion;
use futures::{stream::FuturesUnordered, StreamExt};

use super::type_mapping::*;

#[async_recursion]
pub async fn collect_collection(
    conn: FirestoreConnection,
    full_path: String,
) -> Result<CollectionData, BoxError> {
    let conn_clone = conn.clone();
    let FirestoreConnection(mut client, _base_path) = conn;

    let (parent_path, collection_id) = split_path(&full_path);

    let request = ListDocumentsRequest {
        parent: parent_path.trim_matches('/').to_string(),
        collection_id: collection_id.to_string(),
        page_size: 400,
        page_token: "".to_string(),
        order_by: "".to_string(),
        mask: None,
        show_missing: true,
        consistency_selector: None,
    };

    let result = client.list_documents(request).await?;
    let result = result.into_inner();

    let collection_path = full_path;

    let document_futures =
        FuturesUnordered::from_iter(result.documents.into_iter().map(|item| {
            firestore_doc_to_document_data(conn_clone.clone(), item, &collection_path)
        }));

    let documents_results = document_futures
        .collect::<Vec<Result<DocumentData, BoxError>>>()
        .await;
    let (_, collection_id) = split_path(&collection_path);

    let documents: Vec<DocumentData> = documents_results
        .into_iter()
        .filter_map(|v| v.ok())
        .collect();
    let collection_data = CollectionData {
        id: collection_id,
        documents: documents,
    };
    Ok(collection_data)
}

#[async_recursion]
async fn firestore_doc_to_document_data(
    conn: FirestoreConnection,
    item: Document,
    _document_parent_path: &str,
) -> Result<DocumentData, BoxError> {
    let (_, id) = split_path(&item.name);
    let subcollections = collect_document_collections(conn, &item.name).await?;
    Ok(DocumentData {
        id: id,
        data: item
            .fields
            .into_iter()
            .filter_map(|(key, val)| {
                if val.value_type.is_none() {
                    return None;
                } else {
                    Some((key, val.value_type.unwrap()))
                }
            })
            .map(|(key, raw_value)| {
                let converted = from_firestore_value(raw_value);
                (key, converted)
            })
            .collect(),
        subcollections: subcollections,
    })
}

#[async_recursion]
async fn collect_document_collections(
    conn: FirestoreConnection,
    doc_path: &str,
) -> Result<Option<Vec<CollectionData>>, BoxError> {
    let conn_clone = conn.clone();
    let FirestoreConnection(mut client, _base_path) = conn;
    let document_full_path = doc_path.to_string();
    println!(
        "collect_document_collections: document_full_path = {}",
        document_full_path
    );
    let request = ListCollectionIdsRequest {
        parent: document_full_path.clone(),
        page_size: 400,
        page_token: "".to_string(),
    };
    let response = client.list_collection_ids(request).await?;
    let collection_ids = response.get_ref();
    println!(
        "collect_document_collections: received {} subcollections for {}",
        collection_ids.collection_ids.len(),
        document_full_path
    );

    let futures = FuturesUnordered::new();
    for id in &collection_ids.collection_ids {
        let collection_path = format!("{}/{}", document_full_path, id);
        futures.push(collect_collection(conn_clone.clone(), collection_path));
    }

    let subcollections = futures
        .collect::<Vec<Result<CollectionData, BoxError>>>()
        .await;
    println!(
        "collect_document_collections {} -> {:?} subcollections",
        document_full_path, subcollections
    );
    if subcollections.is_empty() {
        Ok(None)
    } else {
        Ok(Some(
            subcollections
                .into_iter()
                .filter_map(|val| val.ok())
                .collect(),
        ))
    }
}

type PathSegments = (String, String);

fn split_path(path: &str) -> PathSegments {
    let path_vec = path.trim_matches('/').split("/").collect::<Vec<_>>();
    let segments_count = path_vec.len();

    let collection_id = path_vec.last().unwrap().to_string();
    let base_path = path_vec
        .into_iter()
        .enumerate()
        .filter_map(|(index, value)| {
            if index < segments_count - 1 {
                return Some(value);
            }
            None
        })
        .fold(String::from(""), |part, segment| {
            format!("{}/{}", part, segment)
        });
    let base_path = if base_path.is_empty() {
        String::from("/")
    } else {
        base_path
    };
    (base_path, collection_id)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_path_test() {
        let (base_path, collection_id) = split_path("collection");
        assert_eq!(base_path, "/");
        assert_eq!(collection_id, "collection");
        let (base_path, collection_id) = split_path("/collection");
        assert_eq!(base_path, "/");
        assert_eq!(collection_id, "collection");

        let (base_path, collection_id) = split_path("base/id/collection");
        assert_eq!(base_path, "/base/id");
        assert_eq!(collection_id, "collection");

        let (base_path, collection_id) = split_path("/base/id/collection");
        assert_eq!(base_path, "/base/id");
        assert_eq!(collection_id, "collection");
    }
}
