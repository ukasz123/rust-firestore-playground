use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use firestore_grpc::google::r#type::LatLng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ValueType {
    NullValue,
    BooleanValue(bool),
    IntegerValue(i64),
    DoubleValue(f64),
    TimestampValue(i64),
    StringValue(String),
    BytesValue(Vec<u8>),
    ReferenceValue(String),
    GeoPointValue((f64, f64)),
    ArrayValue(Vec<Box<ValueType>>),
    MapValue(HashMap<String, Box<ValueType>>),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentData {
    pub id: String,
    pub data: HashMap<String, ValueType>,
    pub subcollections: Option<Vec<CollectionData>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CollectionData {
    pub id: String,
    pub documents: Vec<DocumentData>,
}

pub fn from_firestore_value(raw_value: firestore_grpc::v1::value::ValueType) -> ValueType {
    match raw_value {
        firestore_grpc::v1::value::ValueType::NullValue(_) => ValueType::NullValue,
        firestore_grpc::v1::value::ValueType::BooleanValue(v) => ValueType::BooleanValue(v),
        firestore_grpc::v1::value::ValueType::IntegerValue(i) => ValueType::IntegerValue(i),
        firestore_grpc::v1::value::ValueType::DoubleValue(d) => ValueType::DoubleValue(d),
        firestore_grpc::v1::value::ValueType::TimestampValue(t) => {
            ValueType::TimestampValue(t.seconds.into())
        }
        firestore_grpc::v1::value::ValueType::StringValue(s) => ValueType::StringValue(s),
        firestore_grpc::v1::value::ValueType::BytesValue(val) => ValueType::BytesValue(val),
        firestore_grpc::v1::value::ValueType::ReferenceValue(path) => {
            ValueType::ReferenceValue(path)
        }
        firestore_grpc::v1::value::ValueType::GeoPointValue(latlng) => {
            ValueType::GeoPointValue((latlng.latitude, latlng.longitude))
        }
        firestore_grpc::v1::value::ValueType::ArrayValue(av) => ValueType::ArrayValue(
            av.values
                .into_iter()
                .map(|val| {
                    val.value_type
                        .map(|inner_val| Box::new(from_firestore_value(inner_val)))
                })
                .filter(|option| option.is_some())
                .map(|option| option.unwrap())
                .collect::<Vec<Box<ValueType>>>(),
        ),
        firestore_grpc::v1::value::ValueType::MapValue(mv) => ValueType::MapValue(
            mv.fields
                .into_iter()
                .filter_map(|(key, value)| match value.value_type {
                    Some(inner_value) => Some((key, Box::new(from_firestore_value(inner_value)))),
                    None => None,
                })
                .collect::<HashMap<String, Box<ValueType>>>(),
        ),
    }
}

pub fn to_firestore_value(value: ValueType) -> firestore_grpc::v1::value::ValueType {
    match value {
        ValueType::NullValue => firestore_grpc::v1::value::ValueType::NullValue(0),
        ValueType::BooleanValue(val) => firestore_grpc::v1::value::ValueType::BooleanValue(val),
        ValueType::IntegerValue(val) => firestore_grpc::v1::value::ValueType::IntegerValue(val),
        ValueType::DoubleValue(val) => firestore_grpc::v1::value::ValueType::DoubleValue(val),
        ValueType::TimestampValue(val) => {
            let timestamp = prost_types::Timestamp {
                seconds: val,
                nanos: 0,
            };
            firestore_grpc::v1::value::ValueType::TimestampValue(timestamp)
        }
        ValueType::StringValue(val) => firestore_grpc::v1::value::ValueType::StringValue(val),
        ValueType::BytesValue(val) => firestore_grpc::v1::value::ValueType::BytesValue(val),
        ValueType::ReferenceValue(val) => firestore_grpc::v1::value::ValueType::ReferenceValue(val),
        ValueType::GeoPointValue((lat, long)) => {
            firestore_grpc::v1::value::ValueType::GeoPointValue(LatLng {
                latitude: lat,
                longitude: long,
            })
        }
        ValueType::ArrayValue(val) => 
            firestore_grpc::v1::value::ValueType::ArrayValue(
                firestore_grpc::v1::ArrayValue{ values: val.into_iter().map(|item| {
                let value: ValueType = *item;
                firestore_grpc::google::firestore::v1::Value{value_type: Some(to_firestore_value(value))}
            }).collect::<Vec<_>>()}),
        
        ValueType::MapValue(val) => firestore_grpc::v1::value::ValueType::MapValue(
            firestore_grpc::google::firestore::v1::MapValue {
                fields: val.into_iter().map(|(key, value)| {
                    let inner_val = *value;
                    (key, firestore_grpc::google::firestore::v1::Value{value_type: Some(to_firestore_value(inner_val))})
                }).collect::<HashMap<_,_>>()
            }
        ),
    }
}
