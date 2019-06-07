use reqwest;
use reqwest::StatusCode;
use std::error::Error;

use crate::esi::models;
use crate::errors::ErrorKind;
use crate::errors::*;


pub fn get_character(id: i32) -> Result<Option<models::CharacterResponse>> {
    let url = format!("https://esi.evetech.net/latest/characters/{}/?datasource=tranquility", id);
    let mut resp = reqwest::get(&url)
        .chain_err(|| "failed getting character info")?;

    if resp.status().is_success() {
        let character: models::CharacterResponse = resp.json()
            .chain_err(|| "failed deserializing character info")?;

        Ok(Some(character))
    } else if resp.status().eq(&StatusCode::NOT_FOUND) {
        Ok(None)
    } else {
        println!("Character request failed. Status: {:?}", resp.status());
        resp.error_for_status_ref()?;
        Err(ErrorKind::Msg(String::from("Character request failed")).into())
    }
}
