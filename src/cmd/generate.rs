use image::{io::Reader as ImageReader, DynamicImage};
use std::{collections::HashMap, fs};

use rand::Rng;
use rand::SeedableRng;
use sha2::Digest;
use sha2::Sha256;

use crate::cmd::fetch::Attribute;
use crate::cmd::fetch::GetERC721TokenById;
use crate::ImageData;

pub const SHADOW_POSITION: u8 = 120;
pub const COLOR_POSITION: u8 = 139;
pub const HIGHLIGHT_POSITION: u8 = 201;

pub const BLACK_COLOR: &[u8] = &[64, 64, 64];
pub const BLACK_HIGHLIGHT: &[u8] = &[163, 163, 163];
pub const BLACK_SHADOW: &[u8] = &[48, 57, 52];

pub const BLUE_COLOR: &[u8] = &[0, 170, 232];
pub const BLUE_HIGHLIGHT: &[u8] = &[135, 214, 244];
pub const BLUE_SHADOW: &[u8] = &[0, 113, 225];

pub const GOLD_COLOR: &[u8] = &[255, 216, 63];
pub const GOLD_HIGHLIGHT: &[u8] = &[254, 236, 164];
pub const GOLD_SHADOW: &[u8] = &[231, 184, 42];

pub const GREEN_COLOR: &[u8] = &[108, 202, 56];
pub const GREEN_HIGHLIGHT: &[u8] = &[187, 230, 161];
pub const GREEN_SHADOW: &[u8] = &[96, 135, 54];

pub const ORANGE_COLOR: &[u8] = &[254, 124, 28];
pub const ORANGE_HIGHLIGHT: &[u8] = &[255, 193, 152];
pub const ORANGE_SHADOW: &[u8] = &[222, 83, 28];

pub const PINK_COLOR: &[u8] = &[253, 157, 203];
pub const PINK_HIGHLIGHT: &[u8] = &[254, 209, 230];
pub const PINK_SHADOW: &[u8] = &[222, 105, 196];

pub const PURPLE_COLOR: &[u8] = &[160, 112, 221];
pub const PURPLE_HIGHLIGHT: &[u8] = &[210, 187, 241];
pub const PURPLE_SHADOW: &[u8] = &[139, 75, 215];

pub const RED_COLOR: &[u8] = &[248, 33, 30];
pub const RED_HIGHLIGHT: &[u8] = &[253, 151, 149];
pub const RED_SHADOW: &[u8] = &[217, 22, 30];

pub fn build_data() {
    // 1. Read legacy token data to use as rarity reference
    let token_data = &fs::read("docs/legacy/token-data.json").unwrap();
    let token_data = String::from_utf8_lossy(token_data).into_owned();
    let token_data: Vec<ImageData> = serde_json::from_str(&token_data).unwrap();

    // 2. Create random generator with seed
    let mut hasher = Sha256::new();
    hasher.update(b"CryptoPuffies");
    let seed = hasher.finalize().into();
    let mut rng = rand_chacha::ChaCha20Rng::from_seed(seed);

    // 3. Generate deterministically using same rarity.
    let mut rand_token_data = vec![];
    for i in 0..8888 {
        let hair_hat = &token_data[rng.gen_range(0..token_data.len())];
        rand_token_data.push(ImageData {
            id: i,
            background: token_data[rng.gen_range(0..token_data.len())]
                .background
                .clone(),
            color: token_data[rng.gen_range(0..token_data.len())].color.clone(),
            face: token_data[rng.gen_range(0..token_data.len())].face.clone(),
            hairstyle: hair_hat.hairstyle.clone(),
            hat: hair_hat.hat.clone(),
            tail: token_data[rng.gen_range(0..token_data.len())].tail.clone(),
            accessory: token_data[rng.gen_range(0..token_data.len())]
                .accessory
                .clone(),
        });
    }

    // 4. Make sure there are no duplicates
    for i in 0..rand_token_data.len() {
        for j in 0..rand_token_data.len() {
            if i != j && rand_token_data[i] == rand_token_data[j] {
                panic!(
                    "Error: found duplicate data! => rand_id1: {} rand_id2: {}",
                    i, j
                );
            }
        }
        for j in 0..token_data.len() {
            if i != j && rand_token_data[i] == token_data[j] {
                panic!(
                    "Error: found duplicate data! => rand_id: {} legacy_id: {}",
                    i, j
                );
            }
        }
    }

    // 5. Overwrite random token data with legacy ones
    for i in 0..token_data.len() {
        rand_token_data[i] = token_data[i].clone();
    }
    let token_data = rand_token_data;

    // 6. Store complete token data
    let contents = serde_json::to_string(&token_data).unwrap();
    fs::write("docs/token-data.json", contents).unwrap();

    // 7. Generate api token data
    for i in 0..token_data.len() {
        let data = GetERC721TokenById {
            token_id: i.to_string(),
            image_cdn_url: format!("https://puffies.cloudtip.me/api/token-data/{}", i),
            attributes: vec![
                Attribute {
                    trait_type: "Background".to_string(),
                    value: token_data[i].background.clone(),
                },
                Attribute {
                    trait_type: "Color".to_string(),
                    value: token_data[i].color.clone(),
                },
                Attribute {
                    trait_type: "Face".to_string(),
                    value: token_data[i].face.clone(),
                },
                Attribute {
                    trait_type: "Hairstyle".to_string(),
                    value: token_data[i].hairstyle.clone(),
                },
                Attribute {
                    trait_type: "Hat".to_string(),
                    value: token_data[i].hat.clone(),
                },
                Attribute {
                    trait_type: "Tail".to_string(),
                    value: token_data[i].tail.clone(),
                },
                Attribute {
                    trait_type: "Accessory".to_string(),
                    value: token_data[i].accessory.clone(),
                },
            ],
        };
        let contents = serde_json::to_string(&data).unwrap();
        fs::write(format!("docs/api/token-data/{}", i), contents).unwrap();
        println!("Generated data: {}", i);
    }
    println!("OK!");
}
pub fn build_images() {
    generate_images();
}
fn generate_images() {
    let token_data = &fs::read("docs/token-data.json").unwrap();
    let token_data = String::from_utf8_lossy(token_data).into_owned();
    let token_data: Vec<ImageData> = serde_json::from_str(&token_data).unwrap();

    let mut image_layers = ImageLayers::new();
    image_layers.load();

    for id in 0..token_data.len() {
        let token = token_data[id as usize].clone();
        create_image(&mut image_layers, token);
        println!("Generated image...{}", id);
    }
    println!("OK!");
}
fn create_image(image_layers: &mut ImageLayers, token: ImageData) {
    let mut puffy_image = image_layers.tails[&token.tail].clone();
    image::imageops::overlay(
        &mut puffy_image,
        &image_layers.hairstyles[&token.hairstyle],
        0,
        0,
    );
    image::imageops::overlay(&mut puffy_image, &image_layers.faces[&token.face], 0, 0);
    image::imageops::overlay(&mut puffy_image, &image_layers.hats[&token.hat], 0, 0);

    let (regular_color, hightlight_color, shadow_color) = if token.color == "Black" {
        (BLACK_COLOR, BLACK_HIGHLIGHT, BLACK_SHADOW)
    } else if token.color == "Blue" {
        (BLUE_COLOR, BLUE_HIGHLIGHT, BLUE_SHADOW)
    } else if token.color == "Gold" {
        (GOLD_COLOR, GOLD_HIGHLIGHT, GOLD_SHADOW)
    } else if token.color == "Green" {
        (GREEN_COLOR, GREEN_HIGHLIGHT, GREEN_SHADOW)
    } else if token.color == "Orange" {
        (ORANGE_COLOR, ORANGE_HIGHLIGHT, ORANGE_SHADOW)
    } else if token.color == "Pink" {
        (PINK_COLOR, PINK_HIGHLIGHT, PINK_SHADOW)
    } else if token.color == "Purple" {
        (PURPLE_COLOR, PURPLE_HIGHLIGHT, PURPLE_SHADOW)
    } else if token.color == "Red" {
        (RED_COLOR, RED_HIGHLIGHT, RED_SHADOW)
    } else {
        panic!("Unknown Color!");
    };

    let data = puffy_image.as_mut_rgba8().unwrap();
    for pixel in data.pixels_mut() {
        let mut r = pixel.0[0];
        let mut g = pixel.0[1];
        let mut b = pixel.0[2];
        let a = pixel.0[3];

        if r == g && g == b {
            let position = r;

            if position <= SHADOW_POSITION {
                // 0 (black) => shadow_position
                let ratio = position as f32 / SHADOW_POSITION as f32;
                r = (shadow_color[0] as f32 * ratio) as u8;
                g = (shadow_color[1] as f32 * ratio) as u8;
                b = (shadow_color[2] as f32 * ratio) as u8;
            } else if position <= COLOR_POSITION {
                // shadow_position => color_position
                let ratio =
                    (position - SHADOW_POSITION) as f32 / (COLOR_POSITION - SHADOW_POSITION) as f32;
                r = ((regular_color[0] - shadow_color[0]) as f32 * ratio) as u8 + shadow_color[0];
                g = ((regular_color[1] - shadow_color[1]) as f32 * ratio) as u8 + shadow_color[1];
                b = ((regular_color[2] - shadow_color[2]) as f32 * ratio) as u8 + shadow_color[2];
            } else if position <= HIGHLIGHT_POSITION {
                // color_position => highlight_position
                let ratio = (position - COLOR_POSITION) as f32
                    / (HIGHLIGHT_POSITION - COLOR_POSITION) as f32;
                r = ((hightlight_color[0] - regular_color[0]) as f32 * ratio) as u8
                    + regular_color[0];
                g = ((hightlight_color[1] - regular_color[1]) as f32 * ratio) as u8
                    + regular_color[1];
                b = ((hightlight_color[2] - regular_color[2]) as f32 * ratio) as u8
                    + regular_color[2];
            } else {
                // highlight_position => 255 (white)
                let ratio =
                    (position - HIGHLIGHT_POSITION) as f32 / (255 - HIGHLIGHT_POSITION) as f32;
                r = ((255 - hightlight_color[0]) as f32 * ratio) as u8 + hightlight_color[0];
                g = ((255 - hightlight_color[1]) as f32 * ratio) as u8 + hightlight_color[1];
                b = ((255 - hightlight_color[2]) as f32 * ratio) as u8 + hightlight_color[2];
            }
        }
        pixel.0 = [r, g, b, a];
    }

    let mut final_image = image_layers.backgrounds[&token.background].clone();
    image::imageops::overlay(&mut final_image, &puffy_image, 0, 0);
    image::imageops::overlay(
        &mut final_image,
        &image_layers.accessories[&token.accessory],
        0,
        0,
    );

    final_image
        .save(format!("docs/images/{}.jpg", token.id))
        .unwrap();
}

pub struct ImageLayers {
    pub backgrounds: HashMap<String, DynamicImage>,
    pub tails: HashMap<String, DynamicImage>,
    pub hairstyles: HashMap<String, DynamicImage>,
    pub faces: HashMap<String, DynamicImage>,
    pub hats: HashMap<String, DynamicImage>,
    pub accessories: HashMap<String, DynamicImage>,
}
impl ImageLayers {
    pub fn new() -> ImageLayers {
        Self {
            backgrounds: HashMap::new(),
            tails: HashMap::new(),
            hairstyles: HashMap::new(),
            faces: HashMap::new(),
            hats: HashMap::new(),
            accessories: HashMap::new(),
        }
    }
    pub fn load(&mut self) {
        for background in BACKGROUNDS {
            self.backgrounds.insert(
                background.to_string(),
                ImageReader::open(format!("docs/image-layers/backgrounds/{}.png", background))
                    .unwrap()
                    .decode()
                    .unwrap(),
            );
        }
        for tail in TAILS {
            self.tails.insert(
                tail.to_string(),
                ImageReader::open(format!("docs/image-layers/tails/{}.png", tail))
                    .unwrap()
                    .decode()
                    .unwrap(),
            );
        }
        for hairstyle in HAIRSTYLES {
            self.hairstyles.insert(
                hairstyle.to_string(),
                ImageReader::open(format!("docs/image-layers/hairstyles/{}.png", hairstyle))
                    .unwrap()
                    .decode()
                    .unwrap(),
            );
        }
        for face in FACES {
            self.faces.insert(
                face.to_string(),
                ImageReader::open(format!("docs/image-layers/faces/{}.png", face))
                    .unwrap()
                    .decode()
                    .unwrap(),
            );
        }
        for hat in HATS {
            self.hats.insert(
                hat.to_string(),
                ImageReader::open(format!("docs/image-layers/hats/{}.png", hat))
                    .unwrap()
                    .decode()
                    .unwrap(),
            );
        }
        for accessory in ACCESSORIES {
            self.accessories.insert(
                accessory.to_string(),
                ImageReader::open(format!("docs/image-layers/accessories/{}.png", accessory))
                    .unwrap()
                    .decode()
                    .unwrap(),
            );
        }
    }
}

pub const BACKGROUNDS: &[&str] = &[
    "Avarioworld",
    "Blue",
    "DragonQuest",
    "Farm",
    "Gray",
    "Green",
    "Orange",
    "Pink",
    "Purple",
    "Red",
    "Room",
    "Ship",
    "Skateboard",
    "StarSpace",
    "UnderSea",
    "Wavarioworld",
    "Yellow",
];

pub const FACES: &[&str] = &[
    "Alien",
    "Angry1",
    "Angry2",
    "Anime",
    "AnimePet",
    "Avario",
    "Awkward",
    "BigSmile",
    "Bored",
    "Bunny",
    "Cartoon",
    "Cat",
    "Concentrated",
    "Concerned",
    "CoolGuy",
    "Curious",
    "Cute",
    "CuteSmile",
    "DarkGlasses",
    "Disgusted",
    "Dog",
    "Doll",
    "Dragon",
    "Dummy",
    "Elegant",
    "Farmer",
    "Full",
    "Funny",
    "Grin",
    "HappyCat",
    "Horned",
    "Inventor",
    "Irritated",
    "LittleCrazy",
    "Madscientist",
    "Mascot",
    "Monster",
    "Mouth",
    "Playful",
    "Propeller",
    "Relaxed",
    "Reptile",
    "Rogue",
    "Scared",
    "Scouter",
    "Shy",
    "Sleepy",
    "SleepyHappy",
    "SmallSmile1",
    "SmallSmile2",
    "Smart",
    "Smile",
    "Smooch",
    "SoHappy",
    "Star",
    "Suspicious1",
    "Suspicious2",
    "TongueOut1",
    "TongueOut2",
    "Visor",
    "Wavario",
    "Wise",
    "Wizard",
];

pub const HAIRSTYLES: &[&str] = &[
    "Anime", "Bart", "Cool", "Curved", "Elegant", "Fire", "Mohawk", "Original", "Spiky", "Stylish",
    "Wild",
];

pub const HATS: &[&str] = &[
    "Artist",
    "Astronaut",
    "Avalanche Cap",
    "Avario",
    "Baby",
    "Bandana",
    "BigHorns",
    "Black Mage",
    "Bomber",
    "BunnyEars",
    "Cape",
    "Captain",
    "CatEars",
    "Cowboy",
    "Crown",
    "Cubiz",
    "Demon",
    "DogEars",
    "DragonEars",
    "DragonWings",
    "Elf",
    "Explorer",
    "Farmer",
    "Ghost",
    "Halo",
    "Headband",
    "Headphones",
    "Hoodie",
    "Ice Crown",
    "Joker",
    "Lightning",
    "Lofi",
    "Lucky",
    "Mage",
    "MageRobes",
    "Mcpuffy",
    "Megapuff",
    "MouseEars",
    "None",
    "Pajama",
    "Pilot",
    "Pirate",
    "Propeller",
    "Puffies",
    "PuffyCap",
    "Ricehat",
    "Robot",
    "SmallHorns",
    "Soldier",
    "Sombrero",
    "Stuntman Helmet",
    "Trainer",
    "Viking",
    "Wavario",
    "Wise",
];

pub const TAILS: &[&str] = &[
    "Anklyosaur",
    "Cat",
    "Devil",
    "Dinosaur",
    "Dragon",
    "Firey",
    "Fox",
    "Genie",
    "Haku",
    "Hare",
    "Kitty",
    "Lightning",
    "Monkey",
    "Monster",
    "Mouse",
    "None",
    "Pig",
    "Platypus",
    "Puffdragon",
    "Scorpion",
    "Slinky",
    "Tails",
    "Turtle",
];

pub const ACCESSORIES: &[&str] = &[
    "BowTie",
    "BubbleTeaLeft",
    "BubbleTeaRight",
    "DiceLeft",
    "DiceRight",
    "Drawn",
    "LaptopLeft",
    "LaptopRight",
    "MageStaff",
    "MiniPuffyLeft",
    "MiniPuffyRight",
    "None",
    "Pitchfork",
    "Ramen",
    "Shield",
    "Surf",
    "Tamagotchi",
];
