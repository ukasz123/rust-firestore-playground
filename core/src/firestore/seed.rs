use std::{collections::HashMap, fmt::Display};

use firestore_grpc::v1::{write::Operation, BeginTransactionRequest, CommitRequest, Write};

pub use super::type_mapping::*;
use super::{type_mapping::to_firestore_value, FirestoreConnection};

pub type BoxError = Box<dyn std::error::Error + Sync + Send + 'static>;

#[derive(Debug)]
pub enum SeedError {
    InvalidPath,
    FirestoreClientError(BoxError),
}

unsafe impl Send for SeedError {}
unsafe impl Sync for SeedError {}

impl Display for SeedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SeedError::InvalidPath => writeln!(f, "Invalid path to the parent document. Path can either be a root path('/') or path to existing document."),
            SeedError::FirestoreClientError(boxed) => boxed.fmt(f),
        }
    }
}
impl std::error::Error for SeedError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SeedError::InvalidPath => None,
            SeedError::FirestoreClientError(internal) => Some(internal.as_ref()),
        }
    }
}

pub async fn seed_collection(
    conn: FirestoreConnection,
    collection: &CollectionData,
    parent_document_path: &str,
) -> Result<usize, SeedError> {
    if !validate_document_path(parent_document_path) {
        return Err(SeedError::InvalidPath);
    }
    let trimmed_parent_path = parent_document_path.trim_matches('/');
    let conn_clone = conn.clone();
    let FirestoreConnection(_, base_path) = conn;
    let database_path = base_path.trim_end_matches("/documents");
    seed_collection_in_transaction(
        conn_clone,
        collection,
        &format!("{}/{}", base_path, trimmed_parent_path),
        &database_path,
    )
    .await
    .map_err(|e| SeedError::FirestoreClientError(e))
    
}

const BATCH_UPDATE_MAX_SIZE: usize = 500;
async fn seed_collection_in_transaction(
    conn: FirestoreConnection,
    collection: &CollectionData,
    parent_path: &str,
    database_path: &str,
) -> Result<usize, BoxError> {
    let operations = generate_writes_for_collection(collection, parent_path);

    for batch in operations.chunks(BATCH_UPDATE_MAX_SIZE) {
        let transaction = begin_transaction(conn.clone(), database_path).await?;

        commit_transaction(conn.clone(), Vec::from(batch), transaction, database_path).await?;
    }
    Ok(operations.len())
}

fn generate_writes_for_collection<'a>(
    collection: &'a CollectionData,
    parent_path: &'a str,
) -> Vec<Operation> {
    let collection_id = &collection.id;
    let documents = &collection.documents;
    let collection_path = format!("{}/{}", parent_path.trim_end_matches('/'), collection_id);
    documents
        .into_iter()
        .map(|document| generate_writes_for_document(document, &collection_path))
        .flatten()
        .collect()
}

fn generate_writes_for_document(
    document: &super::type_mapping::DocumentData,
    collection_path: &str,
) -> Vec<Operation> {
    let mut updates = Vec::new();
    let firestore_doc = to_firestore_document(&document, &collection_path);
    let document_path = firestore_doc.name.clone();
    updates.push(Operation::Update(firestore_doc));
    if let Some(subcollections) = &document.subcollections {
        for subcollection in subcollections {
            let operations_for_subcollection =
                generate_writes_for_collection(subcollection, &document_path);
            updates.extend(operations_for_subcollection);
        }
    }
    updates
}

fn to_firestore_document(
    document: &super::type_mapping::DocumentData,
    parent_path: &str,
) -> firestore_grpc::v1::Document {
    let document_path = format!("{}/{}", parent_path.trim_end_matches('/'), document.id);
    
    firestore_grpc::v1::Document {
        name: document_path,
        fields: document
            .data
            .clone()
            .into_iter()
            .map(|(key, value)| {
                (
                    key,
                    firestore_grpc::v1::Value {
                        value_type: Some(to_firestore_value(value)),
                    },
                )
            })
            .collect::<HashMap<_, _>>(),
        create_time: None,
        update_time: None,
    }
}

async fn begin_transaction(
    conn: FirestoreConnection,
    database_path: &str,
) -> Result<Vec<u8>, BoxError> {
    let FirestoreConnection(mut client, _base_path) = conn;
    let transaction_request = BeginTransactionRequest {
        database: database_path.to_string(),
        options: None,
    };
    let transaction_response = client.begin_transaction(transaction_request).await?;
    let transaction_response = transaction_response.into_inner();
    Ok(transaction_response.transaction)
}

async fn commit_transaction(
    conn: FirestoreConnection,
    operations: Vec<Operation>,
    transaction: Vec<u8>,
    database_path: &str,
) -> Result<(), BoxError> {
    let FirestoreConnection(mut client, _base_path) = conn;
    
    let commit_request = CommitRequest {
        database: database_path.to_string(),
        writes: operations
            .into_iter()
            .map(|operation| Write {
                update_mask: None, // always override
                update_transforms: vec![],
                current_document: None,
                operation: Some(operation),
            })
            .collect::<Vec<_>>(),
        transaction,
    };
    let commit_response = client.commit(commit_request).await?;
    let _commit_response = commit_response.into_inner();
    Ok(())
}

fn validate_document_path(path: &str) -> bool {
    let trimmed = path.trim_matches('/');
    let parts = trimmed.split('/').filter(|t| !(*t).is_empty()).count();
    parts % 2 == 0
}
#[cfg(test)]
mod tests {
    use super::validate_document_path;

    #[test]
    fn test_path_validation() {
        assert!(validate_document_path("/"));
        assert!(validate_document_path("/coll/123"));
        assert!(validate_document_path("/coll/123/subcol/456"));
        assert!(!validate_document_path("/col"));
        assert!(!validate_document_path("/coll/123/col"));
        assert!(!validate_document_path("/coll/123/subcol/456/col"));
        assert!(validate_document_path("/coll/123/"));
        assert!(validate_document_path("/coll/123/subcol/456/"));
        assert!(!validate_document_path("/col/"));
        assert!(!validate_document_path("/coll/123/col/"));
        assert!(!validate_document_path("/coll/123/subcol/456/col/"));
    }
}
