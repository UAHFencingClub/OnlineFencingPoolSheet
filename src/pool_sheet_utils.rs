use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// #[path = "./pool_orders.rs"]
// mod pool_orders

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Fencer {
    name: String,
    club: Option<Club>,
    wins: u8,
    losses: u8,
    touches_scored: u8,
    touches_recieved: u8,
    indicator: u8,
    place: u8,
}

// fn _sort_fencers_for_bouts(mut fencers: Vec<Fencer>) {
//     todo!();
//     let mut teammate_counter = HashMap::<Club, u8>::new();
//     let mut teammate_sets: u8 = 0;
//     let fencers_in_pool = fencers.len();

//     for fencer in fencers {
//         match teammate_counter.get(&fencer.club) {
//             Some(current_count) => {teammate_counter.insert(fencer.club, current_count+1);},
//             _ => {teammate_counter.insert(fencer.club, 1);}
//         }
//     }

//     for (_, teammate_count) in teammate_counter {
//         if teammate_count > 1 {
//             teammate_sets += 1;
//         }
//     }
// }

#[derive(Debug)]
#[derive(Hash)]
#[derive(PartialEq, Eq)]
#[derive(Serialize, Deserialize)]

struct Club{
    full_name: String,
    shortname: String
}

#[derive(Debug)]
struct Bout {
    fencer_a: Box<Fencer>,
    fencer_b: Box<Fencer>,
    score_a: Option<u8>,
    score_b: Option<u8>,
    finished: bool
}

impl Bout {
    fn update_score(mut self, score_a: u8, score_b: u8) {
        self.score_a = Some(score_a);
        self.score_b = Some(score_b);
    } 
}

#[derive(Debug, Default)]
pub struct PoolSheet {
    fencers: Vec<Fencer>,
    bouts: Vec<Bout>
}

impl PoolSheet {
    pub fn test() -> PoolSheet {
        PoolSheet {
            fencers: Vec::new(),
            bouts: Vec::new(),
        }
    }

    fn add_fencers<I>(&mut self, fencers: I)
    where
        I: Iterator<Item = Fencer>,
    {
        self.fencers.extend(fencers);
    }

    pub fn add_fencer_from_str(&mut self, fencer_name: String) {
        let new_fencer = Fencer {
            name: fencer_name,
            club: None,
            wins: 0,
            losses: 0,
            touches_recieved: 0,
            touches_scored: 0,
            indicator: 0,
            place: 0,
        };
        self.fencers.push(new_fencer);
    }

    pub fn add_fencer_from_json(&mut self, fencer_json: String) -> Result<(), serde_json::Error> {
        let fencer_result: Result<Fencer, serde_json::Error>  = serde_json::from_str(fencer_json.as_str());
        match fencer_result {
            Ok(fencer) => {
                self.fencers.push(fencer);
                Ok(())
            },
            Err(error_message) => Err(error_message),
        }
    }

    pub fn get_fencers(self) -> String {
        match serde_json::to_string(&self.fencers) {
            Ok(fencer_json) => fencer_json,
            Err(message) => message.to_string(),
        }
    }

    fn render(&self) {
        let mut output_html = String::new();
        output_html.push_str("")
    }

    fn create_bouts() {
        // How many fencers per each team

    }

    fn finalize_results() {

    }
}
