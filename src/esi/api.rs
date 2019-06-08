use reqwest;
use reqwest::StatusCode;
use std::error::Error;

use crate::esi::models;
use crate::errors::ErrorKind;
use crate::errors::*;


pub fn get_character(id: i32) -> Result<Option<models::CharacterResponse>> {
    let url = format!("https://esi.evetech.net/latest/characters/{}/?datasource=tranquility", id);
    info!("GET {}", url);
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

pub fn get_skill_queue(id: i32, token: &str) -> Result<Option<Vec<models::SkillQueueResponse>>> {
    let url = format!("https://esi.evetech.net/latest/characters/{}/skillqueue/?datasource=tranquility", id);
    info!("GET {}", url);
    let client = reqwest::Client::new();

    let mut resp = client.get(&url)
        .bearer_auth(token)
        .send()?;

    if resp.status().is_success() {
        let queue: Vec<models::SkillQueueResponse> = resp.json()
            .chain_err(|| "failed deserializing skill queue")?;

        Ok(Some(queue))
    } else if resp.status().eq(&StatusCode::NOT_FOUND) {
        Ok(None)
    } else {
        println!("Skill queue request failed. Status: {:?}", resp.status());
        resp.error_for_status_ref()?;
        Err(ErrorKind::Msg(String::from("Skill queue request failed")).into())
    }
}