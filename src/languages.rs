use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all(serialize = "camelCase"))]
pub struct Language {
    #[serde(alias = "urlParam")]
    pub url_param: String,
    pub name: String,
}

pub fn get_data() -> Vec<Language> {
    let bytes = include_bytes!("../languages.json");
    let data = String::from_utf8_lossy(bytes);

    let data_json: Vec<Language> = serde_json::from_str(&data).unwrap();

    // print!("{:?}", data_json);
    data_json
}

pub fn get_data_json() -> Value {
    let data = get_data();

    let x: Vec<Value> = data.into_iter().map(|x| json!(x)).collect();

    let data_json: Value = Value::Array(x);
    // println!("{:?}", data_json);
    data_json
}
