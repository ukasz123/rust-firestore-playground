use firestore_grpc::tonic::{
    codegen::InterceptedService,
    metadata::{Ascii, MetadataValue},
    service::Interceptor,
    transport::{Channel, ClientTlsConfig},
};

pub mod collect;
pub mod seed;
mod type_mapping;

#[derive(Clone)]
pub struct FirestoreConnection(pub FirestoreClient, pub String);

const URL: &'static str = "https://firestore.googleapis.com";
const DOMAIN: &'static str = "firestore.googleapis.com";

pub type BoxError = Box<dyn std::error::Error + Sync + Send + 'static>;
pub type FirestoreClient = firestore_grpc::v1::firestore_client::FirestoreClient<InterceptedService<Channel, AuthInterceptor>>;

pub async fn get_client(
    token: &str,
) -> Result<FirestoreClient, BoxError> {
    let endpoint =
        Channel::from_static(URL).tls_config(ClientTlsConfig::new().domain_name(DOMAIN))?;

    let bearer_token = format!("Bearer {}", token);
    let header_value = MetadataValue::from_str(&bearer_token)?;

    let interceptor = AuthInterceptor { header_value };
    let channel = endpoint.connect().await?;

    let service = firestore_grpc::v1::firestore_client::FirestoreClient::with_interceptor(channel, interceptor.clone());
    Ok(service)
}

/// Intercepts the channel to provide authorization headers
#[derive(Clone)]
pub struct AuthInterceptor {
    header_value: MetadataValue<Ascii>,
}

impl Interceptor for AuthInterceptor {
    fn call(
        &mut self,
        mut req: firestore_grpc::tonic::Request<()>,
    ) -> Result<firestore_grpc::tonic::Request<()>, firestore_grpc::tonic::Status> {
        req.metadata_mut()
            .insert("authorization", self.header_value.clone());
        Ok(req)
    }
}
