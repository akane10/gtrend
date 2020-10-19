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

pub fn find_by_name(lang: &str) -> Option<Language> {
    let lang_lists = get_data();

    lang_lists
        .into_iter()
        .find(|x| x.name.to_lowercase() == lang.to_lowercase())
}

pub fn find_by_url_param(lang: &str) -> Option<Language> {
    let lang_lists = get_data();

    lang_lists
        .into_iter()
        .find(|x| x.url_param.to_lowercase() == lang.to_lowercase())
}

pub fn find_by_both(lang: &str) -> Option<Language> {
    let lang_lists = get_data();

    lang_lists.into_iter().find(|x| {
        let l = lang.to_lowercase();

        x.url_param.to_lowercase() == l || x.name.to_lowercase() == l
    })
}
