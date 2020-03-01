//!
//! [[Rust] Serdeのシリアライズ/デシリアライズを試してみる](https://dev.classmethod.jp/server-side/language/rust-serde-getting-started/)
//! [Rust】serde_jsonの使い方](https://www.amusement-creators.info/post/articles/advent_calendar/2019/02_0/)
//! [Serialize fields as camelCase](https://serde.rs/attr-rename.html)
//!
extern crate serde;
extern crate serde_json;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[serde(rename_all = "camelCase")]
#[derive(Deserialize, Debug)]
struct LinearRegressionWars {
    fighting_nations: Vec<FightingNation>,
}

#[derive(Deserialize, Debug)]
struct FightingNation {
    name: String,
    win: u32,
    draw: u32,
    lose: u32,
    tanks: HashMap<String, Tank>,
}

#[derive(Deserialize, Debug)]
struct Tank {
    number: u16,
    magazine: String,
}

fn main() {
    // Open file.
    let mut file = match File::open("./linear-regression-wars.json") {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    // Read file.
    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    // Desirialize.
    let linear_regression_wars = match serde_json::from_str::<LinearRegressionWars>(&contents) {
        Ok(n) => n,
        Err(err) => panic!("File open error. {:?}", err),
    };

    for fighting_nation in linear_regression_wars.fighting_nations {
        let mut sum_cost = 0;
        // Cost check.
        sum_cost += fighting_nation.tanks["tako2000"].number
            * (2 + get_magazine_cost(&fighting_nation.tanks["tako2000"].magazine));
        sum_cost += fighting_nation.tanks["tako3000"].number
            * (6 + get_magazine_cost(&fighting_nation.tanks["tako3000"].magazine));
        sum_cost += fighting_nation.tanks["tako5000"].number
            * (10 + get_magazine_cost(&fighting_nation.tanks["tako5000"].magazine));
        println!("name={} cost={}", fighting_nation.name, sum_cost);
        /*
        println!(
            "    tank number={}",
            fighting_nation.tanks["tako2000"].number
        );
        */
    }

    println!("Info    | Finished.");
}

fn get_magazine_cost(name: &str) -> u16 {
    match name {
        "grape2x4-hole1" => 2,
        "grape5x4-hole2" => 4,
        "grape5x3-hole3" => 3,
        _ => 0,
    }
}
