use std::fs;

use reqwest::{blocking::Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::{thread, time};

use crate::ImageData;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ImageUrl {
    pub id: i64,
    pub url: String,
}

pub fn download_data() {
    let mut image_urls = vec![];
    let mut token_data = vec![];

    let client = Client::new();
    for i in 0..5724 {
        let data = Body::new(i);
        let body = serde_json::to_string(&data).unwrap();

        let mut resp_text = String::new();
        while resp_text == "" {
            let req = client
                .post("https://nftkey.app/api")
                .body(body.clone())
                .header("Content-Type", "application/json");
            match req.send() {
                Ok(resp) => {
                    if resp.status() == StatusCode::OK {
                        if let Ok(text) = resp.text() {
                            resp_text = text.clone();
                        }
                    }
                }
                Err(err) => {
                    println!("Request Failed: {}", err);
                    println!("Retrying...");
                    thread::sleep(time::Duration::from_millis(1000));
                }
            };
        }

        let res: Result = serde_json::from_str(&resp_text).unwrap();

        let id = res.data.get_erc721_token_by_id.token_id.parse().unwrap();
        let url = res.data.get_erc721_token_by_id.image_cdn_url;
        image_urls.push(ImageUrl { id, url });

        let mut image_data = ImageData::default();
        image_data.id = id;
        for attribute in res.data.get_erc721_token_by_id.attributes {
            if attribute.trait_type == "Background" {
                image_data.background = attribute.value;
            } else if attribute.trait_type == "Color" {
                image_data.color = attribute.value;
            } else if attribute.trait_type == "Face" {
                image_data.face = attribute.value;
            } else if attribute.trait_type == "Hairstyle" {
                image_data.hairstyle = attribute.value;
            } else if attribute.trait_type == "Hat" {
                image_data.hat = attribute.value;
            } else if attribute.trait_type == "Tail" {
                image_data.tail = attribute.value;
            } else if attribute.trait_type == "Accessory" {
                image_data.accessory = attribute.value;
            }
        }
        token_data.push(image_data);

        println!("Downloaded data: {}", id);
    }

    //save files
    let contents = serde_json::to_string(&image_urls).unwrap();
    fs::write("docs/legacy/image-urls.json", contents).unwrap();
    println!("Saved: legacy/image-urls.json");

    let contents = serde_json::to_string(&token_data).unwrap();
    fs::write("docs/legacy/token-data.json", contents).unwrap();
    println!("Saved: legacy/token-data.json");

    println!("OK!");
}

pub fn download_images() {
    let image_urls = &fs::read("docs/legacy/image-urls.json").unwrap();
    let image_urls = String::from_utf8_lossy(image_urls).into_owned();
    let image_urls: Vec<ImageUrl> = serde_json::from_str(&image_urls).unwrap();

    for i in image_urls {
        let mut file = std::fs::File::create(format!("docs/legacy/images/{}.jpg", &i.id)).unwrap();
        let mut img_written = false;
        while !img_written {
            match reqwest::blocking::get(&i.url) {
                Ok(mut img) => match img.copy_to(&mut file) {
                    Ok(_) => img_written = true,
                    Err(e) => println!("Writing Failed: {}. Retrying...", e),
                },
                Err(e) => {
                    println!("Request Failed: {}", e);
                    println!("Retrying...");
                    thread::sleep(time::Duration::from_millis(1000));
                }
            };
        }
        println!("Downloaded image: {}", i.id);
    }
    println!("OK!");
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Body {
    pub operation_name: String,
    pub variables: Variables,
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Variables {
    pub collection_id: String,
    pub token_id: String,
}
impl Body {
    pub fn new(id: u32) -> Self {
        let id = id.to_string();
        Body {
            operation_name: "GetERC721TokenById".to_string(), 
            variables: Variables{
                collection_id:"0x457c224e4a2db059071f01ee2ff43078ac021597_43114".to_string(), 
                token_id: id
            },
            query: "query GetERC721TokenById($collectionId: String!, $tokenId: String!) {\n  getERC721TokenById(collectionId: $collectionId, tokenId: $tokenId) {\n    ...ERC721TokenMetadata\n  }\n}\n\nfragment ERC721TokenMetadata on ERC721TokenMetadata {\n  tokenId\n  imageCdnUrl\n  attributes {\n    trait_type\n    value\n  }\n }".to_string() }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Result {
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Data {
    #[serde(rename = "getERC721TokenById")]
    pub get_erc721_token_by_id: GetERC721TokenById,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GetERC721TokenById {
    pub token_id: String,
    pub image_cdn_url: String,
    pub attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub trait_type: String,
    pub value: String,
}
