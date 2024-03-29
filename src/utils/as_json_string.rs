use serde_json;
use serde::ser::{Serialize, Serializer};
use serde::de::{Deserialize, DeserializeOwned, Deserializer};

pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
{
    use serde::ser::Error;
    let j = serde_json::to_string(value).map_err(Error::custom)?;
    j.serialize(serializer)
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: DeserializeOwned,
        D: Deserializer<'de>,
{
    use serde::de::Error;
    let j = String::deserialize(deserializer)?;
    serde_json::from_str(&j).map_err(Error::custom)
}
