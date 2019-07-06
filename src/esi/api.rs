use reqwest;
use reqwest::StatusCode;

use crate::errors::*;
use crate::errors::ErrorKind;
use crate::esi::models;

pub fn get_character(id: i32) -> Result<Option<models::CharacterResponse>> {
    let url = format!("https://esi.evetech.net/latest/characters/{}/?datasource=tranquility", id);
    info!("GET {}", url);
    let mut resp = reqwest::get(&url)
        .chain_err(|| "failed getting character info")?;

    if resp.status().is_success() {
        let character: models::CharacterResponse = resp.json()?;
        Ok(Some(character))
    } else if resp.status().eq(&StatusCode::NOT_FOUND) {
        Ok(None)
    } else {
        println!("Character request failed. Status: {:?}", resp.status());
        resp.error_for_status_ref()?;
        Err(ErrorKind::Msg(String::from("Character request failed")).into())
    }
}

pub fn get_corporation(id: i32) -> Result<Option<models::CorporationResponse>> {
    let url = format!("https://esi.evetech.net/latest/corporations/{}/?datasource=tranquility", id);
    info!("GET {}", url);
    let mut resp = reqwest::get(&url)
        .chain_err(|| "failed getting corporation info")?;

    if resp.status().is_success() {
        let corporation: models::CorporationResponse = resp.json()
            .chain_err(|| "failed deserializing corporation info")?;

        Ok(Some(corporation))
    } else if resp.status().eq(&StatusCode::NOT_FOUND) {
        Ok(None)
    } else {
        println!("Corporation request failed. Status: {:?}", resp.status());
        resp.error_for_status_ref()?;
        Err(ErrorKind::Msg(String::from("Corporation request failed")).into())
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

pub fn get_skills(id: i32, token: &str) -> Result<Option<models::SkillsResponse>> {
    let url = format!("https://esi.evetech.net/latest/characters/{}/skills/?datasource=tranquility", id);
    info!("GET {}", url);
    let client = reqwest::Client::new();

    let mut resp = client.get(&url)
        .bearer_auth(token)
        .send()?;

    if resp.status().is_success() {
        let queue: models::SkillsResponse = resp.json()
            .chain_err(|| "failed deserializing skills")?;

        Ok(Some(queue))
    } else if resp.status().eq(&StatusCode::NOT_FOUND) {
        Ok(None)
    } else {
        println!("Skills request failed. Status: {:?}", resp.status());
        resp.error_for_status_ref()?;
        Err(ErrorKind::Msg(String::from("Skills request failed")).into())
    }
}