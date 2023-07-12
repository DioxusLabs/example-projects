// List all breeds
// Random image
// By breed
// By sub-breed
// Browse breed list

#![allow(non_snake_case)]
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AllBreeds {
    pub message: HashMap<String, Vec<String>>,
    pub status: String,
}


pub async fn list_all_breeds() -> reqwest::Result<AllBreeds> {
    reqwest::get("https://dog.ceo/api/breeds/list/all")
        .await
        ?
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
    .await
    ?
    .json::<RandomImageByBreed>()
    .await
}


