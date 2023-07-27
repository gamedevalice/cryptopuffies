use std::fs;

use serde::{Deserialize, Serialize};

use crate::ImageData;

pub fn query(with: Vec<String>, without: Vec<String>) {
    let mut with_props = vec![];
    for c in with {
        let parts: Vec<&str> = c.split(":").collect();
        let layer = ImageLayer::from(parts[0]).unwrap();
        let variant = parts[1];
        with_props.push((layer, variant.to_string()));
    }

    let mut without_props = vec![];
    for c in without {
        let parts: Vec<&str> = c.split(":").collect();
        let layer = ImageLayer::from(parts[0]).unwrap();
        let variant = parts[1];
        without_props.push((layer, variant.to_string()));
    }
    let mut results = query_properties(with_props, without_props)
        .into_iter()
        .map(|t| t.id)
        .collect::<Vec<i64>>();

    //print ids
    println!("Results:");
    println!("{:?}", results);

    //print ids formated as query for file explorer
    while results.len() > 26 {
        results.pop();
    }
    let query_str = results
        .iter()
        .map(|&i| format!("\"{}\"", i.to_string()))
        .collect::<Vec<String>>()
        .join(" OR ");
    println!("\nSearch str:");
    println!("{}", query_str);
}

pub fn query_properties(
    with: Vec<(ImageLayer, String)>,
    without: Vec<(ImageLayer, String)>,
) -> Vec<ImageData> {
    let token_db = &fs::read("docs/token-data.json").unwrap();
    let token_db = String::from_utf8_lossy(token_db).into_owned();
    let token_db: Vec<ImageData> = serde_json::from_str(&token_db).unwrap();

    let mut result = vec![];
    for i in 0..token_db.len() {
        let mut is_match = true;
        for property in &with {
            let (property, variant) = property;
            let variant = variant.to_string();
            match property {
                ImageLayer::Background => {
                    if token_db[i].background != variant {
                        is_match = false
                    }
                }
                ImageLayer::Color => {
                    if token_db[i].color != variant {
                        is_match = false
                    }
                }
                ImageLayer::Face => {
                    if token_db[i].face != variant {
                        is_match = false
                    }
                }
                ImageLayer::Hairstyle => {
                    if token_db[i].hairstyle != variant {
                        is_match = false
                    }
                }
                ImageLayer::Hat => {
                    if token_db[i].hat != variant {
                        is_match = false
                    }
                }
                ImageLayer::Tail => {
                    if token_db[i].tail != variant {
                        is_match = false
                    }
                }
                ImageLayer::Accessory => {
                    if token_db[i].accessory != variant {
                        is_match = false
                    }
                }
            }
        }
        for property in &without {
            let (property, variant) = property;
            let variant = variant.to_string();
            match property {
                ImageLayer::Background => {
                    if token_db[i].background == variant {
                        is_match = false
                    }
                }
                ImageLayer::Color => {
                    if token_db[i].color == variant {
                        is_match = false
                    }
                }
                ImageLayer::Face => {
                    if token_db[i].face == variant {
                        is_match = false
                    }
                }
                ImageLayer::Hairstyle => {
                    if token_db[i].hairstyle == variant {
                        is_match = false
                    }
                }
                ImageLayer::Hat => {
                    if token_db[i].hat == variant {
                        is_match = false
                    }
                }
                ImageLayer::Tail => {
                    if token_db[i].tail == variant {
                        is_match = false
                    }
                }
                ImageLayer::Accessory => {
                    if token_db[i].accessory == variant {
                        is_match = false
                    }
                }
            }
        }
        if is_match {
            result.push(token_db[i].clone());
        }
    }
    result
}

pub fn query_by_id(id: i64) {
    let token_db = &fs::read("docs/token-data.json").unwrap();
    let token_db = String::from_utf8_lossy(token_db).into_owned();
    let token_db: Vec<ImageData> = serde_json::from_str(&token_db).unwrap();
    println!("{:?}", token_db[id as usize]);
}
pub fn list_variants() {
    let token_db = &fs::read("docs/token-data.json").unwrap();
    let token_db = String::from_utf8_lossy(token_db).into_owned();
    let token_db: Vec<ImageData> = serde_json::from_str(&token_db).unwrap();

    let mut backgrounds = vec![];
    let mut colors = vec![];
    let mut faces = vec![];
    let mut hairstyles = vec![];
    let mut hats = vec![];
    let mut tails = vec![];
    let mut accessories = vec![];
    for i in 0..5724 {
        if !backgrounds.contains(&token_db[i].background) {
            backgrounds.push(token_db[i].background.clone());
        }
        if !colors.contains(&token_db[i].color) {
            colors.push(token_db[i].color.clone());
        }
        if !faces.contains(&token_db[i].face) {
            faces.push(token_db[i].face.clone());
        }
        if !hairstyles.contains(&token_db[i].hairstyle) {
            hairstyles.push(token_db[i].hairstyle.clone());
        }
        if !hats.contains(&token_db[i].hat) {
            hats.push(token_db[i].hat.clone());
        }
        if !tails.contains(&token_db[i].tail) {
            tails.push(token_db[i].tail.clone());
        }
        if !accessories.contains(&token_db[i].accessory) {
            accessories.push(token_db[i].accessory.clone());
        }
    }
    println!("-------------------------------------");
    println!("backgrounds: ({})", backgrounds.len());
    println!("-------------------------------------");
    backgrounds.sort();
    for variant in backgrounds {
        let result: Vec<i64> =
            query_properties(vec![(ImageLayer::Background, variant.clone())], vec![])
                .into_iter()
                .map(|t| t.id)
                .collect();
        println!(
            "=== Background: {:?} ({:?}): {:?}",
            variant,
            result.len(),
            &result[0..3]
        );
    }
    println!("-------------------------------------");
    println!("colors: ({})", colors.len());
    println!("-------------------------------------");
    colors.sort();
    for variant in colors {
        let result: Vec<i64> = query_properties(vec![(ImageLayer::Color, variant.clone())], vec![])
            .into_iter()
            .map(|t| t.id)
            .collect();
        println!(
            "=== Color: {:?} ({:?}): {:?}",
            variant,
            result.len(),
            &result[0..3]
        );
    }
    println!("-------------------------------------");
    println!("faces: ({})", faces.len());
    println!("-------------------------------------");
    faces.sort();
    for variant in faces {
        let result: Vec<i64> = query_properties(vec![(ImageLayer::Face, variant.clone())], vec![])
            .into_iter()
            .map(|t| t.id)
            .collect();
        println!(
            "=== Face: {:?} ({:?}): {:?}",
            variant,
            result.len(),
            &result[0..3]
        );
    }
    println!("-------------------------------------");
    println!("hairstyles: ({})", hairstyles.len());
    println!("-------------------------------------");
    hairstyles.sort();
    for variant in hairstyles {
        let result: Vec<i64> =
            query_properties(vec![(ImageLayer::Hairstyle, variant.clone())], vec![])
                .into_iter()
                .map(|t| t.id)
                .collect();
        println!(
            "=== Hairstyle: {:?} ({:?}): {:?}",
            variant,
            result.len(),
            &result[0..3]
        );
    }
    println!("-------------------------------------");
    println!("hats: ({})", hats.len());
    println!("-------------------------------------");
    hats.sort();
    for variant in hats {
        let result: Vec<i64> = query_properties(vec![(ImageLayer::Hat, variant.clone())], vec![])
            .into_iter()
            .map(|t| t.id)
            .collect();
        println!(
            "=== Hat: {:?} ({:?}): {:?}",
            variant,
            result.len(),
            &result[0..3]
        );
    }
    println!("-------------------------------------");
    println!("tails: ({})", tails.len());
    println!("-------------------------------------");
    tails.sort();
    for variant in tails {
        let result: Vec<i64> = query_properties(vec![(ImageLayer::Tail, variant.clone())], vec![])
            .into_iter()
            .map(|t| t.id)
            .collect();
        println!(
            "=== Tail: {:?} ({:?}): {:?}",
            variant,
            result.len(),
            &result[0..3]
        );
    }
    println!("-------------------------------------");
    println!("accessories: ({})", accessories.len());
    println!("-------------------------------------");
    accessories.sort();
    for variant in accessories {
        let result: Vec<i64> =
            query_properties(vec![(ImageLayer::Accessory, variant.clone())], vec![])
                .into_iter()
                .map(|t| t.id)
                .collect();
        println!(
            "=== Accessory: {:?} ({:?}): {:?}",
            variant,
            result.len(),
            &result[0..3]
        );
    }
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
pub enum ImageLayer {
    Color,
    Background,
    Tail,
    Hairstyle,
    Face,
    Hat,
    Accessory,
}
impl ImageLayer {
    pub fn from(s: &str) -> Result<Self, ()> {
        match s {
            "color" => Ok(ImageLayer::Color),
            "background" => Ok(ImageLayer::Background),
            "tail" => Ok(ImageLayer::Tail),
            "hairstyle" => Ok(ImageLayer::Hairstyle),
            "face" => Ok(ImageLayer::Face),
            "hat" => Ok(ImageLayer::Hat),
            "accessory" => Ok(ImageLayer::Accessory),
            _ => Err(()),
        }
    }
}
