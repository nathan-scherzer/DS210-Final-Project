//cleaan the data - remove all observations other than all

//offense = shots_on_goal + goals + cont_in_zone_after_shot - exit_zone_after_shot + exp_goals + dangerous_shots
//defensive = takeaways - opp_shots_on_goal - d_zone_giveaways + opp_missed_shots - opp_goals - opp_exp_goals - opp_dangerous_shots
//grit =  hits + blocks - penalties_for_perc + opp_giveaways + opp_d_zone_giveaways
//transition = faceoff_win_perc - opp_cont_in_zone_after_shot + opp_exit_zone_after_shot - gveaways_for - opp_takeaways

//offense:
//shots_on_goal = 30
//goals = 3
//cont_in_zone_after_shot = 0.45
//exit_zone_after_shot = 0.25
//exp_goals = 3.5
//dangerous_shots = 4

//defense:
//takeaways = 10
//opp_shots_on_goal = 26
//d_zone_giveaways = 3
//opp_missed_shots = 10
//opp_goals = 2
//opp_exp_goals = 2.33
//opp_dangerous_shots = 2.5

//grit
//hits = 25
//blocks = 10
//penalties_for_perc = 0.4
//opp_giveaways = 10
//opp_d_zone_giveaways = 5

//transition
//faceoff_win_perc = 0.55
//opp_cont_in_zone_after_shot = 0.35
//opp_exit_zone_after_shot = 0.35
//gveaways = 8
//opp_takeaways = 13

use std::collections::HashMap;
//use csv::ReaderBuilder;

// Structs for the different categories of statistics
pub struct Offense {
    pub shots_on_goal: f64,
    pub goals: f64,
    pub cont_in_zone_after_shot: f64,
    pub exit_zone_after_shot: f64,
    pub exp_goals: f64,
    pub dangerous_shots: f64,
}

pub struct Defense {
    pub takeaways: f64,
    pub opp_shots_on_goal: f64,
    pub d_zone_giveaways: f64,
    pub opp_missed_shots: f64,
    pub opp_goals: f64,
    pub opp_exp_goals: f64,
    pub opp_dangerous_shots: f64,
}

pub struct Grit {
    pub hits: f64,
    pub blocks: f64,
    pub penalties_for_perc: f64,
    pub opp_giveaways: f64,
    pub opp_d_zone_giveaways: f64,
}

pub struct Transition {
    pub faceoff_win_perc: f64,
    pub opp_cont_in_zone_after_shot: f64,
    pub opp_exit_zone_after_shot: f64,
    pub giveaways: f64,
    pub opp_takeaways: f64,
}

// Function to read the CSV file
//pub fn read_csv(file_path: &str) -> Result<(Vec<String>, Vec<HashMap<String, String>>), Box<dyn std::error::Error>> {
  //  let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
   // let headers = rdr.headers()?.clone(); // Clone headers here
   // let mut rows = Vec::new();
    
   // for result in rdr.records() {
    //    let record = result?;
    //    let mut row = HashMap::new();
        
    //    for (i, field) in record.iter().enumerate() {
   //         row.insert(headers.get(i).unwrap().to_string(), field.to_string());
   //     }
        
   //     rows.push(row);
   // }
    
   // Ok((headers, rows))
//}

/*
// Function to compute standard deviations
pub fn compute_std_devs<'a>(data: &'a Vec<HashMap<&'a str, f64>>) -> HashMap<&'a str, f64> {
    let mut stats_values: HashMap<&str, Vec<f64>> = HashMap::new();
    for entry in data {
        for (stat, value) in entry {
            stats_values.entry(stat.clone()).or_insert_with(Vec::new).push(*value);
        }
    }
    let mut standard_deviations: HashMap<&str, f64> = HashMap::new();
    for (stat, values) in stats_values {
        let mean = values.iter().copied().sum::<f64>() / values.len() as f64;
        let variance = values.iter().map(|&x| (x - mean).powi(2)).sum::<f64>() / values.len() as f64;
        let std_dev = variance.sqrt();
        standard_deviations.insert(stat, std_dev);
    }
    standard_deviations
}
*/

// Implementations for scoring
impl Offense {
    pub fn compute_offense_score(&self, benchmarks: &[(f64, f64, bool)]) -> f64 {
        benchmarks.iter().enumerate().map(|(i, &(benchmark, std_dev, is_positive))| {
            let value = match i {
                0 => self.shots_on_goal,
                1 => self.goals,
                2 => self.cont_in_zone_after_shot,
                3 => self.exit_zone_after_shot,
                4 => self.exp_goals,
                5 => self.dangerous_shots,
                _ => 0.0,
            };
            let z_score = (value as f64 - benchmark) / std_dev;
            if is_positive { z_score } else { -z_score }
        }).sum()
    }
}

impl Defense {
    pub fn compute_defense_score(&self, benchmarks: &[(f64, f64, bool)]) -> f64 {
        benchmarks.iter().enumerate().map(|(i, &(benchmark, std_dev, is_positive))| {
            let value = match i {
                0 => self.takeaways,
                1 => self.opp_shots_on_goal,
                2 => self.d_zone_giveaways,
                3 => self.opp_missed_shots,
                4 => self.opp_goals,
                5 => self.opp_exp_goals,
                6 => self.opp_dangerous_shots,
                _ => 0.0,
            };
            let z_score = (value as f64 - benchmark) / std_dev;
            if is_positive { z_score } else { -z_score }
        }).sum()
    }
}

impl Grit {
    pub fn compute_grit_score(&self, benchmarks: &[(f64, f64, bool)]) -> f64 {
        benchmarks.iter().enumerate().map(|(i, &(benchmark, std_dev, is_positive))| {
            let value = match i {
                0 => self.hits,
                1 => self.blocks,
                2 => self.penalties_for_perc,
                3 => self.opp_giveaways,
                4 => self.opp_d_zone_giveaways,
                _ => 0.0,
            };
            let z_score = (value as f64 - benchmark) / std_dev;
            if is_positive { z_score } else { -z_score }
        }).sum()
    }
}

impl Transition {
    pub fn compute_transition_score(&self, benchmarks: &[(f64, f64, bool)]) -> f64 {
        benchmarks.iter().enumerate().map(|(i, &(benchmark, std_dev, is_positive))| {
            let value = match i {
                0 => self.faceoff_win_perc,
                1 => self.opp_cont_in_zone_after_shot,
                2 => self.opp_exit_zone_after_shot,
                3 => self.giveaways,
                4 => self.opp_takeaways,
                _ => 0.0,
            };
            let z_score = (value as f64 - benchmark) / std_dev;
            if is_positive { z_score } else { -z_score }
        }).sum()
    }
}

//MAKE THIS A TRAIT FOR SIMPLICITY