use git2::Oid;
use serde::{self, de::Error, Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub catalog: String,
    pub issuer: String,
    pub client_id: String
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    #[serde(
        default,
        serialize_with = "serialize_oid",
        deserialize_with = "deserialize_oid"
    )]
    pub last_commit: Option<Oid>,
}

fn deserialize_oid<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Oid>, D::Error> {
    let s: Option<String> = Option::deserialize(deserializer)?;
    s.as_deref()
        .map(Oid::from_str)
        .transpose()
        .map_err(D::Error::custom)
}

fn serialize_oid<S: Serializer>(oid: &Option<Oid>, serializer: S) -> Result<S::Ok, S::Error> {
    oid.as_ref().map(ToString::to_string).serialize(serializer)
}
