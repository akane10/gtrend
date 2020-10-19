use crate::{By, Language};
use serde_json::Value;

pub fn get_data() -> Vec<Language> {
    let bytes = include_bytes!("../languages.json");
    let data = Language::get_data(bytes);

    // println!("language: {:?}", data[0]);
    data
}

pub fn get_data_json() -> Value {
    let data = get_data();

    let data_json: Value = Language::get_data_json(data);
    data_json
}

pub fn find(by: By) -> Option<Language> {
    let mut lang_lists = get_data().into_iter();

    match by {
        By::Name(lang) => lang_lists.find(|x| x.name.to_lowercase() == lang.to_lowercase()),
        By::UrlParam(lang) => {
            lang_lists.find(|x| x.url_param.to_lowercase() == lang.to_lowercase())
        }
        By::Both(lang) => lang_lists.find(|x| {
            let l = lang.to_lowercase();

            x.url_param.to_lowercase() == l || x.name.to_lowercase() == l
        }),
    }
}
