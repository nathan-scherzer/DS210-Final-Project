use std::collections::HashMap;

pub fn calculate_z_score(value: f64, benchmark: f64, weight: f64, std_dev: f64) -> f64 {
    ((value - benchmark) / std_dev) * weight
}

pub fn compute_offense_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, f64, bool)]) -> f64 {
    // Filter data for the specified team and season
    let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
        row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

    // Compute z-scores for each statistic across all games
    let mut weighted_z_scores_sum = vec![0.0; benchmarks.len()];
    let num_games = team_data.len() as f64;

    for row in team_data {
        for (i, &(benchmark, weight, std_dev, is_positive)) in benchmarks.iter().enumerate() {
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
                    let z_score = calculate_z_score(value, benchmark, weight, std_dev);
                    if is_positive == true {
                        weighted_z_scores_sum[i] += z_score
                    }
                    else {
                        weighted_z_scores_sum[i] -= z_score
                    }
                }
            }
        }
    }
    // Average z-scores for each statistic and sum them
    weighted_z_scores_sum.iter().map(|&sum| sum / num_games).sum()
}

pub fn compute_defense_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, f64, bool)]) -> f64 {
    // Filter data for the specified team and season
    let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
        row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

    // Compute z-scores for each statistic across all games
    let mut weighted_z_scores_sum = vec![0.0; benchmarks.len()];
    let num_games = team_data.len() as f64;

    for row in team_data {
        for (i, &(benchmark, weight, std_dev, is_positive)) in benchmarks.iter().enumerate() {
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
                    let z_score = calculate_z_score(value, benchmark, weight, std_dev);
                    if is_positive == true {
                        weighted_z_scores_sum[i] += z_score
                    }
                    else {
                        weighted_z_scores_sum[i] -= z_score
                    }
                }
            }
        }
    }
    // Average z-scores for each statistic and sum them
    weighted_z_scores_sum.iter().map(|&sum| sum / num_games).sum()
}

pub fn compute_grit_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, f64, bool)]) -> f64 {
    // Filter data for the specified team and season
    let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
        row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

    // Compute z-scores for each statistic across all games
    let mut weighted_z_scores_sum = vec![0.0; benchmarks.len()];
    let num_games = team_data.len() as f64;

    for row in team_data {
        for (i, &(benchmark, weight, std_dev, is_positive)) in benchmarks.iter().enumerate() {
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
                    let z_score = calculate_z_score(value, benchmark, weight, std_dev);
                    if is_positive == true {
                        weighted_z_scores_sum[i] += z_score
                    }
                    else {
                        weighted_z_scores_sum[i] -= z_score
                    }
                }
            }
        }
    }
    // Average z-scores for each statistic and sum them
    weighted_z_scores_sum.iter().map(|&sum| sum / num_games).sum()
}

pub fn compute_transition_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, f64, bool)]) -> f64 {
    // Filter data for the specified team and season
    let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
        row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

    // Compute z-scores for each statistic across all games
    let mut weighted_z_scores_sum = vec![0.0; benchmarks.len()];
    let num_games = team_data.len() as f64;

    for row in team_data {
        for (i, &(benchmark, weight, std_dev, is_positive)) in benchmarks.iter().enumerate() {
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
                    let z_score = calculate_z_score(value, benchmark, weight, std_dev);
                    if is_positive == true {
                        weighted_z_scores_sum[i] += z_score
                    }
                    else {
                        weighted_z_scores_sum[i] -= z_score
                    }
                }
            }
        }
    }
    // Average z-scores for each statistic and sum them
    weighted_z_scores_sum.iter().map(|&sum| sum / num_games).sum()
}