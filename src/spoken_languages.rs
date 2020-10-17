use crate::Language;
use serde_json::Value;

pub fn get_data() -> Vec<Language> {
    let bytes = include_bytes!("../spoken_languages.json");
    let data = Language::get_data(bytes);

    // println!("spoken_language: {:?}", data[0]);
    data
}

pub fn get_data_json() -> Value {
    let data = get_data();

    let data_json: Value = Language::get_data_json(data);
    data_json
}
