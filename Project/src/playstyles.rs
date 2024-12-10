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

//use csv::ReaderBuilder;
use std::collections::HashMap;

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

fn calculate_z_score(value: f64, benchmark: f64, std_dev: f64) -> f64 {
    (value - benchmark) / std_dev
}

// Implementations for scoring
impl Offense {
    /// Function to compute the offensive score for a team in a given season
    pub fn compute_offense_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, bool)]) -> f64 {
        // Filter data for the specified team and season
        let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
            row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

        // Compute z-scores for each statistic across all games
        let mut z_scores_sum = vec![0.0; benchmarks.len()];
        let num_games = team_data.len() as f64;

        for row in team_data {
            for (i, &(benchmark, std_dev, is_positive)) in benchmarks.iter().enumerate() {
                let stat_name = match i {
                    0 => "shots_on_goal",
                    1 => "goals",
                    2 => "cont_in_zone_after_shot",
                    3 => "exit_zone_after_shot",
                    4 => "exp_goals",
                    5 => "dangerous_shots",
                    _ => panic!("Unexpected index for benchmarks"),
                };

                if let Some(value_str) = row.get(stat_name) {
                    if let Ok(value) = value_str.parse::<f64>() {
                        let z_score = calculate_z_score(value, benchmark, std_dev);
                        if is_positive == true {
                            z_scores_sum[i] += z_score
                        }
                        else {
                            z_scores_sum[i] -= z_score
                        }
                    }
                }
            }
        }

        // Average z-scores for each statistic and sum them
        z_scores_sum.iter().map(|&sum| sum / num_games).sum()
    }
}

impl Defense {
    /// Function to compute the offensive score for a team in a given season
    pub fn compute_defense_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, bool)]) -> f64 {
        // Filter data for the specified team and season
        let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
            row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

        // Compute z-scores for each statistic across all games
        let mut z_scores_sum = vec![0.0; benchmarks.len()];
        let num_games = team_data.len() as f64;

        for row in team_data {
            for (i, &(benchmark, std_dev, is_positive)) in benchmarks.iter().enumerate() {
                let stat_name = match i {
                    0 => "takeaways",
                    1 => "opp_shots_on_goal",
                    2 => "d_zone_giveaways",
                    3 => "opp_missed_shots",
                    4 => "opp_goals",
                    5 => "opp_exp_goals",
                    6 => "opp_dangerous_shots",
                    _ => panic!("Unexpected index for benchmarks"),
                };

                if let Some(value_str) = row.get(stat_name) {
                    if let Ok(value) = value_str.parse::<f64>() {
                        let z_score = calculate_z_score(value, benchmark, std_dev);
                        if is_positive == true {
                            z_scores_sum[i] += z_score
                        }
                        else {
                            z_scores_sum[i] -= z_score
                        }
                    }
                }
            }
        }

        // Average z-scores for each statistic and sum them
        z_scores_sum.iter().map(|&sum| sum / num_games).sum()
    }
}

impl Grit {
    /// Function to compute the offensive score for a team in a given season
    pub fn compute_grit_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, bool)]) -> f64 {
        // Filter data for the specified team and season
        let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
            row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

        // Compute z-scores for each statistic across all games
        let mut z_scores_sum = vec![0.0; benchmarks.len()];
        let num_games = team_data.len() as f64;

        for row in team_data {
            for (i, &(benchmark, std_dev, is_positive)) in benchmarks.iter().enumerate() {
                let stat_name = match i {
                    0 => "hits",
                    1 => "blocks",
                    2 => "penalties_for_perc",
                    3 => "opp_giveaways",
                    4 => "opp_d_zone_giveaways",
                    _ => panic!("Unexpected index for benchmarks"),
                };

                if let Some(value_str) = row.get(stat_name) {
                    if let Ok(value) = value_str.parse::<f64>() {
                        let z_score = calculate_z_score(value, benchmark, std_dev);
                        if is_positive == true {
                            z_scores_sum[i] += z_score
                        }
                        else {
                            z_scores_sum[i] -= z_score
                        }
                    }
                }
            }
        }

        // Average z-scores for each statistic and sum them
        z_scores_sum.iter().map(|&sum| sum / num_games).sum()
    }
}

impl Transition {
    /// Function to compute the offensive score for a team in a given season
    pub fn compute_transition_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, bool)]) -> f64 {
        // Filter data for the specified team and season
        let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
            row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

        // Compute z-scores for each statistic across all games
        let mut z_scores_sum = vec![0.0; benchmarks.len()];
        let num_games = team_data.len() as f64;

        for row in team_data {
            for (i, &(benchmark, std_dev, is_positive)) in benchmarks.iter().enumerate() {
                let stat_name = match i {
                    0 => "faceoff_win_perc",
                    1 => "opp_cont_in_zone_after_shot",
                    2 => "opp_exit_zone_after_shot",
                    3 => "giveaways",
                    4 => "opp_takeaways",
                    _ => panic!("Unexpected index for benchmarks"),
                };

                if let Some(value_str) = row.get(stat_name) {
                    if let Ok(value) = value_str.parse::<f64>() {
                        let z_score = calculate_z_score(value, benchmark, std_dev);
                        if is_positive == true {
                            z_scores_sum[i] += z_score
                        }
                        else {
                            z_scores_sum[i] -= z_score
                        }
                    }
                }
            }
        }

        // Average z-scores for each statistic and sum them
        z_scores_sum.iter().map(|&sum| sum / num_games).sum()
    }
}


/*
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
*/

//MAKE THIS A TRAIT FOR SIMPLICITY