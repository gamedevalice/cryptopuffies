use clap::Parser;
use serde::{Deserialize, Serialize};

mod cmd;

#[derive(Parser)]
struct Cli {
    /// Fetch puffy data
    #[arg(long)]
    fetch_data: bool,

    /// Fetch puffy images
    #[arg(long)]
    fetch_images: bool,

    /// Generate missing puffy data
    #[arg(short, long)]
    data: bool,

    /// Generate puffy images
    #[arg(short, long)]
    images: bool,

    /// List all variants
    #[arg(long)]
    variants: bool,

    /// Get puffy by id
    #[arg(long)]
    get: Option<i64>,

    /// Query with criteria
    #[arg(long)]
    with: Vec<String>,

    /// Query without criteria
    #[arg(long)]
    without: Vec<String>,

    /// Compare rarity
    #[arg(long)]
    rarity_cmp: bool,

    /// List attribute rarity
    #[arg(long)]
    rarity: bool,
}

fn main() {
    let args = Cli::parse();
    if args.fetch_data {
        cmd::fetch::download_data();
    }
    if args.fetch_images {
        cmd::fetch::download_images();
    }
    if args.data {
        cmd::generate::build_data();
    }
    if args.images {
        cmd::generate::build_images();
    }
    if args.variants {
        cmd::analyze::list_variants();
    }
    if let Some(id) = args.get {
        cmd::analyze::query_by_id(id);
    }
    if args.with.len() != 0 || args.without.len() != 0 {
        cmd::analyze::query(args.with, args.without);
    }
    if args.rarity_cmp {
        cmd::analyze::compare_rarity();
    }
    if args.rarity {
        cmd::analyze::attribute_rarity();
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone, PartialEq)]
pub struct ImageData {
    pub id: i64,
    pub background: String,
    pub color: String,
    pub face: String,
    pub hairstyle: String,
    pub hat: String,
    pub tail: String,
    pub accessory: String,
}
