//!
//! [[Rust] Serdeのシリアライズ/デシリアライズを試してみる](https://dev.classmethod.jp/server-side/language/rust-serde-getting-started/)
//! [Rust】serde_jsonの使い方](https://www.amusement-creators.info/post/articles/advent_calendar/2019/02_0/)
//! [Serialize fields as camelCase](https://serde.rs/attr-rename.html)
//!
extern crate rand;
extern crate serde;
extern crate serde_json;

use rand::seq::SliceRandom;
use serde::Deserialize;
use std::cmp;
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

struct GameTank {
    pub hit_point: i8,
    pub shot: i8,
    pub balls: i8,
}
impl GameTank {
    pub fn new(hit_point1: i8, shot1: i8, balls1: i8) -> Self {
        GameTank {
            hit_point: hit_point1,
            shot: shot1,
            balls: balls1,
        }
    }
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

    let mut enable_game = true;
    {
        for fighting_nation in &linear_regression_wars.fighting_nations {
            let mut sum_cost = 0;
            // Cost check.
            sum_cost += fighting_nation.tanks["tako2000"].number
                * (2 + get_magazine_cost(&fighting_nation.tanks["tako2000"].magazine));
            sum_cost += fighting_nation.tanks["tako3000"].number
                * (6 + get_magazine_cost(&fighting_nation.tanks["tako3000"].magazine));
            sum_cost += fighting_nation.tanks["tako5000"].number
                * (10 + get_magazine_cost(&fighting_nation.tanks["tako5000"].magazine));
            /*
            println!(
                "    tank number={}",
                fighting_nation.tanks["tako2000"].number
            );
            */
            if 2000 < sum_cost {
                // Regulation violation.
                enable_game = false;
            }
            println!(
                "name={} cost={}{}",
                fighting_nation.name,
                sum_cost,
                if enable_game { "" } else { " Cost over 2000." }
            );
        }
    }

    if !enable_game {
        println!("Info    | No game. Finished.");
        return;
    }

    // Matching.
    // とりあえず [0] と [1] を戦わせてみようぜ☆（＾～＾）？
    let player1 = 0usize;
    let player2 = 1usize;

    // New game.
    let mut rng = rand::thread_rng();
    let tank_names = ["tako2000", "tako3000", "tako5000"];
    let mut player_tanks = [Vec::<GameTank>::new(), Vec::<GameTank>::new()];
    for i_player in [player1, player2].iter() {
        for tank_name in tank_names.iter() {
            let magazine =
                &linear_regression_wars.fighting_nations[*i_player].tanks[*tank_name].magazine;
            for _i_tank in
                0..linear_regression_wars.fighting_nations[*i_player].tanks[*tank_name].number
            {
                let tank = create_tank(tank_name, &magazine);
                player_tanks[*i_player].push(tank);
            }
        }

        player_tanks[*i_player].shuffle(&mut rng);
    }

    // とりあえず情報☆（＾～＾）
    println!(
        "Info    | player[{}] tanks={}. player[{}] tanks={}.",
        player1,
        player_tanks[player1].len(),
        player2,
        player_tanks[player2].len()
    );

    // Battle.
    // とりあえず繰り返せだぜ☆（＾～＾）
    let mut is_game_end = false;
    for i_time in 0..100 {
        println!("Trace   | time={}", i_time);
        // 両陣営☆（＾～＾）
        for (i_player, i_opponent) in [(player1, player2), (player2, player1)].iter() {
            // Annihilation. (全滅)
            if player_tanks[*i_player].is_empty() {
                is_game_end = true;
                break;
            }

            print!(
                "Trace   | {}'s attack!",
                &linear_regression_wars.fighting_nations[*i_player].name
            );
            // 戦闘の１０車両が弾を撃てだぜ☆（＾～＾）
            let mut sum_shot = 0;
            for i_front_line in 0..cmp::min(10, player_tanks[*i_player].len()) {
                // 撃てる弾数を集計☆（＾～＾）
                let attacker_tank = &player_tanks[*i_player][i_front_line];
                let shot = if attacker_tank.balls < attacker_tank.shot {
                    attacker_tank.balls
                } else {
                    attacker_tank.shot
                };
                print!(" {}", shot);
                player_tanks[*i_player][i_front_line].balls -= shot;
                sum_shot += shot;
            }

            println!(" ={}.", sum_shot);

            // TODO 弾が尽きたときのローテンション処理を書く☆（＾～＾）

            // 弾が当たるぜ☆（＾～＾）
            let mut i_target = 0;
            for _i_shot in 0..sum_shot {
                if player_tanks[*i_opponent].len() <= i_target {
                    break;
                }
                // println!("Trace   | target={}", i_target);
                let mut target_tank = &mut player_tanks[*i_opponent][i_target as usize];
                target_tank.hit_point -= 1;
                if cmp::min(9, player_tanks[*i_opponent].len()) <= i_target {
                    i_target = 0;
                } else {
                    i_target += 1;
                }
            }
        }

        // HPが0より大きい車両だけ残すぜ☆（＾～＾）
        for i_player in [player1, player2].iter() {
            player_tanks[*i_player].retain(|x| 0 < x.hit_point);
        }

        if is_game_end {
            break;
        }
    }

    // Result.
    // とりあえず勝敗☆（＾～＾）
    println!(
        "Info    | player[{}] tanks={}. player[{}] tanks={}.",
        player1,
        player_tanks[player1].len(),
        player2,
        player_tanks[player2].len()
    );

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

fn create_tank(tank_name: &str, magazine_name: &str) -> GameTank {
    let hit_point = match tank_name {
        "tako2000" => 2,
        "tako3000" => 4,
        "tako5000" => 7,
        _ => 0,
    };

    match magazine_name {
        "grape2x4-hole1" => GameTank::new(hit_point, 1, 8),
        "grape5x4-hole2" => GameTank::new(hit_point, 2, 20),
        "grape5x3-hole3" => GameTank::new(hit_point, 3, 15),
        _ => GameTank::new(0, 0, 0),
    }
}
