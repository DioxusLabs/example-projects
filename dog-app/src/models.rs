#![allow(non_snake_case)]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct AllBreeds {
    pub message: HashMap<String, Vec<String>>,
    pub status: String,
}

pub async fn list_all_breeds() -> reqwest::Result<AllBreeds> {
    reqwest::get("https://dog.ceo/api/breeds/list/all")
        .await?
        .json::<AllBreeds>()
        .await
}

#[derive(Serialize, Deserialize)]
pub struct RandomImageByBreed {
    pub message: String,
    pub status: String,
}

pub async fn random_image_by_breed(breed: &str) -> reqwest::Result<RandomImageByBreed> {
    reqwest::get(format!("https://dog.ceo/api/breed/{breed}/images/random"))
        .await?
        .json::<RandomImageByBreed>()
        .await
}
