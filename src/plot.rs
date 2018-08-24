use std::fs::File;
use std::io::prelude::*;
use serde_json::{Value, Error};
use serde_json;
use models::Observables;

#[macro_use]
extern crate rust_embed;

#[derive(RustEmbed)]
#[folder = "web/"]
struct Asset;

#[derive(Serialize, Deserialize, Debug)]
pub struct CartesianPoint {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chart {
    pub data: Vec<CartesianPoint>,
    pub plot_type: String,
}

pub fn render_web() {
    let index_html = Asset::get("index.html").unwrap();
    println!("{:?}", std::str::from_utf8(&index_html));
}

pub fn write_plot(observables: Vec<Observables>) {
    let mut file = File::create("plots/data.json").unwrap();
    write!(file, "{}", serde_json::to_string(&chart).unwrap());
}