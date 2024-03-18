use futures::StreamExt;
use mongodb::{
    bson::{self, to_document, Document},
    Cursor,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{future::Future, pin::Pin};

pub trait CursorIntoVec<T> {
    fn into_vec(self) -> Pin<Box<dyn Future<Output = Vec<T>> + Unpin>>
    where
        T: 'static + Serialize + DeserializeOwned + Sync + Send + Unpin;
}

impl<T> CursorIntoVec<T> for Cursor<T> {
    fn into_vec(self) -> Pin<Box<dyn Future<Output = Vec<T>> + Unpin>>
    where
        T: 'static + Serialize + DeserializeOwned + Sync + Send + Unpin,
    {
        let fut = StreamExt::map(self, |item| {
            let doc: Document = to_document(&item.unwrap()).unwrap();
            let bson = bson::Bson::Document(doc);
            return bson::from_bson(bson).unwrap();
        })
        .collect();
        Pin::new(Box::new(fut))
    }
}
