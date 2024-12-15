use std::collections::HashMap;

pub fn calculate_z_score(value: f64, benchmark: f64, weight: f64, std_dev: f64) -> f64 {
    ((value - benchmark) / std_dev) * weight
}

pub fn compute_offense_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, f64, bool)]) -> f64 {
    let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
        row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

    let mut weighted_z_scores_sum = vec![0.0; benchmarks.len()];
    let num_games = team_data.len() as f64;

    for row in team_data {
        for (i, &(mean, weight, std_dev, is_positive)) in benchmarks.iter().enumerate() {
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
                    let z_score = calculate_z_score(value, mean, weight, std_dev);
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
    weighted_z_scores_sum.iter().map(|&sum| sum / num_games).sum()
}

pub fn compute_defense_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, f64, bool)]) -> f64 {
    let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
        row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

    let mut weighted_z_scores_sum = vec![0.0; benchmarks.len()];
    let num_games = team_data.len() as f64;

    for row in team_data {
        for (i, &(mean, weight, std_dev, is_positive)) in benchmarks.iter().enumerate() {
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
                    let z_score = calculate_z_score(value, mean, weight, std_dev);
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
    weighted_z_scores_sum.iter().map(|&sum| sum / num_games).sum()
}

pub fn compute_grit_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, f64, bool)]) -> f64 {
    let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
        row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

    let mut weighted_z_scores_sum = vec![0.0; benchmarks.len()];
    let num_games = team_data.len() as f64;

    for row in team_data {
        for (i, &(mean, weight, std_dev, is_positive)) in benchmarks.iter().enumerate() {
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
                    let z_score = calculate_z_score(value, mean, weight, std_dev);
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
    weighted_z_scores_sum.iter().map(|&sum| sum / num_games).sum()
}

pub fn compute_transition_score(data: &[HashMap<String, String>], team_name: &str, season: &str, benchmarks: &[(f64, f64, f64, bool)]) -> f64 {
    let team_data: Vec<&HashMap<String, String>> = data.iter().filter(|row| {
        row.get("team") == Some(&team_name.to_string()) && row.get("season") == Some(&season.to_string())}).collect();

    let mut weighted_z_scores_sum = vec![0.0; benchmarks.len()];
    let num_games = team_data.len() as f64;

    for row in team_data {
        for (i, &(mean, weight, std_dev, is_positive)) in benchmarks.iter().enumerate() {
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
                    let z_score = calculate_z_score(value, mean, weight, std_dev);
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
    weighted_z_scores_sum.iter().map(|&sum| sum / num_games).sum()
}

pub fn compute_all_scores(data: &[HashMap<String, String>], team_name: &str, season: &str) -> (f64, f64, f64, f64) {
    let offense_benchmarks = vec![
        (30.5, 0.2, 6.880765, true), //shots_on_goal
        (2.9, 0.45, 1.680705, true), //goals
        (0.45, 0.1, 0.137061, true), //cont_in_zone_after_shot
        (0.53, 0.05, 0.137306, false), //exit_zone_after_shot
        (2.8, 0.1, 0.962112, true), //exp_goals
        (2.7, 0.1, 1.933012, true), //dangerous_shots
    ];
    let defense_benchmarks = vec![
        (6.9, 0.15, 3.818616, true), //takeaways
        (30.5, 0.1, 6.880765, false), //opp_shots_on_goal
        (4.9, 0.1, 3.647803, false), //d_zone_giveaways
        (11.9, 0.05, 4.266148, true), //opp_missed_shots
        (2.9, 0.4, 1.680705, false), //opp_goals
        (2.8, 0.1, 0.962112, false), //opp_exp_goals
        (2.7, 0.1, 1.933012, false), //opp_dangerous_shots
    ];
    let grit_benchmarks = vec![
        (22.6, 0.45, 8.754365, true), //hits
        (14.3, 0.3, 5.175412, true), //blocks
        (0.5, 0.05, 0.143277, false), //penalties_for_perc
        (8.7, 0.15, 4.952046, true), //opp_giveaways
        (4.9, 0.05, 3.647803, true), //opp_d_zone_giveaways
    ];
    let transition_benchmarks = vec![
        (0.5, 0.1, 0.073265, true), //faceoff_win_perc
        (0.45, 0.2, 0.137061, false), //opp_cont_in_zone_after_shot
        (0.53, 0.2, 0.137306, true), //opp_exit_zone_after_shot
        (8.7, 0.4, 4.952046, false), //giveaways
        (6.9, 0.1, 3.818616, false), //opp_takeaways
    ];

    let offense_score = compute_offense_score(&data, &team_name, &season, &offense_benchmarks);
    let defense_score = compute_defense_score(&data, &team_name, &season, &defense_benchmarks);
    let grit_score = compute_grit_score(&data, &team_name, &season, &grit_benchmarks);
    let transition_score = compute_transition_score(&data, &team_name, &season, &transition_benchmarks);

    (offense_score, defense_score, grit_score, transition_score)
}