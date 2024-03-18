pub mod cursor_to_vec;
pub mod model;

use core::fmt;
use mongodb::bson::{oid::ObjectId, to_bson, Document};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize_object_id<S>(oid: &Option<ObjectId>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match oid.as_ref().map(|x| x.to_hex()) {
        Some(v) => s.serialize_str(&v),
        None => s.serialize_none(),
    }
}

pub fn deserialize_object_id<'de, D>(deserializer: D) -> Result<Option<ObjectId>, D::Error>
where
    D: Deserializer<'de>,
{
    struct JsonOptionObjectIdVisitor;

    impl<'de> de::Visitor<'de> for JsonOptionObjectIdVisitor {
        type Value = Option<ObjectId>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("an object id hash value")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            if v.is_empty() {
                return Ok(None);
            }
            Ok(ObjectId::parse_str(v).ok())
        }
    }

    deserializer.deserialize_any(JsonOptionObjectIdVisitor)
}

pub fn struct_into_document<'a, T>(t: &T) -> Option<Document>
where
    T: Serialize + Deserialize<'a>,
{
    let mid: Option<Document> = to_bson(t).ok().map(|x| x.as_document().unwrap().to_owned());

    mid.map(|mut doc| {
        let keys = doc.keys();
        let rm: Vec<String> = keys
            .filter(|k| doc.is_null(k))
            .map(|x| x.to_owned())
            .collect();
        for x in rm {
            doc.remove(&x);
        }
        doc.to_owned()
    })
}
