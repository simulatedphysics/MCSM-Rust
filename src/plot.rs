use std::fs::File;
use std::io::prelude::*;

use serde_json::{Value, Error};
use serde_json;
use models::Observables;


#[derive(Serialize, Deserialize, Debug)]
pub struct CartesianPoint {
    pub x: f64,
    pub y: f64,
    pub z: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PlotType {
    VectorField,
}

impl PlotType {
    pub fn as_str(&self) -> &str {
        match self {
            &PlotType::VectorField => "vector_field",
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Chart {
    pub data: Vec<CartesianPoint>,
    pub plot_type: PlotType,
    pub name: String,
}

pub fn write_plot(charts: Vec<Chart>) {
    for chart in charts {
        let mut file = File::create(format!("web/{}.json", chart.name)).unwrap();
        write!(file, "{}", serde_json::to_string(&chart).unwrap());
    }
}