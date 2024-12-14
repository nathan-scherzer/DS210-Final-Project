use std::collections::HashMap;
use std::error::Error;
use csv::ReaderBuilder;
use std::io::{self, Write};
use itertools::Itertools;

mod playstyles;
use crate::playstyles::compute_offense_score;
use crate::playstyles::compute_defense_score;
use crate::playstyles::compute_grit_score;
use crate::playstyles::compute_transition_score;

mod tests;

pub fn read_csv(file_path: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let headers = reader.headers()?.clone();
    let mut rows = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut row = HashMap::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                // Treat team and opponent as strings, parse others as floats
                if header == "team" || header == "opponent" {
                    row.insert(header.to_string(), field.to_string());
                } else {
                    let value: String = if let Ok(parsed) = field.parse::<f64>() {
                        parsed.to_string() // Store numeric fields as strings
                    } else {
                        field.to_string()
                    };
                    row.insert(header.to_string(), value);
                }
            }
        }
        rows.push(row);
    }
    Ok(rows)
}

pub fn compute_all_scores_for_team_in_season(data: &[HashMap<String, String>], team_name: &str, season: &str) -> (f64, f64, f64, f64) {
    let offense_benchmarks = vec![
        (32.8, 0.2, 6.880765, true), //shots_on_goal
        (3.0, 0.45, 1.680705, true), //goals
        (0.48, 0.1, 0.137061, true), //cont_in_zone_after_shot
        (0.49, 0.05, 0.137306, false), //exit_zone_after_shot
        (3.0, 0.1, 0.962112, true), //exp_goals
        (2.9, 0.1, 1.933012, true), //dangerous_shots
    ];
    let defense_benchmarks = vec![
        (7.4, 0.15, 3.818616, true), //takeaways
        (28.2, 0.1, 6.880765, false), //opp_shots_on_goal
        (4.6, 0.1, 3.647803, false), //d_zone_giveaways
        (12.7, 0.05, 4.266148, true), //opp_missed_shots
        (2.6, 0.4, 1.680705, false), //opp_goals
        (2.6, 0.1, 0.962112, false), //opp_exp_goals
        (2.5, 0.1, 1.933012, false), //opp_dangerous_shots
    ];
    let grit_benchmarks = vec![
        (24.2, 0.45, 8.754365, true), //hits
        (15.3, 0.3, 5.175412, true), //blocks
        (0.46, 0.05, 0.143277, false), //penalties_for_perc
        (9.3, 0.15, 4.952046, true), //opp_giveaways
        (5.3, 0.05, 3.647803, true), //opp_d_zone_giveaways
    ];
    let transition_benchmarks = vec![
        (0.54, 0.1, 0.073265, true), //faceoff_win_perc
        (0.41, 0.2, 0.137061, false), //opp_cont_in_zone_after_shot
        (0.57, 0.2, 0.137306, true), //opp_exit_zone_after_shot
        (8.0, 0.4, 4.952046, false), //giveaways
        (6.4, 0.1, 3.818616, false), //opp_takeaways
    ];

    let offense_score = compute_offense_score(&data, &team_name, &season, &offense_benchmarks);
    let defense_score = compute_defense_score(&data, &team_name, &season, &defense_benchmarks);
    let grit_score = compute_grit_score(&data, &team_name, &season, &grit_benchmarks);
    let transition_score = compute_transition_score(&data, &team_name, &season, &transition_benchmarks);

    (offense_score, defense_score, grit_score, transition_score)
}

fn prompt_for_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// option 1: print the four scores of a single team in a single season]
fn print_single_team_season_scores(data: &[HashMap<String, String>], team_name: &str, season: &str) {
    let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores_for_team_in_season(data, team_name, season);
    
    println!("\nFor {} in {}:", team_name, season);
    println!("Offensive Score: {:.2}", offense_score);
    println!("Defensive Score: {:.2}", defense_score);
    println!("Grit Score: {:.2}", grit_score);
    println!("Transition Score: {:.2}", transition_score);
}

// option 2: print the four scores of a single team in every season
fn print_team_scores_for_all_seasons(data: &[HashMap<String, String>], team_name: &str) {
    let seasons: Vec<String> = data.iter().filter_map(|row| row.get("season").map(|s| s.to_string())).collect();
    
    println!(" ");
    for season in seasons.iter().unique() {
        let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores_for_team_in_season(data, team_name, season);
        
        println!("Scores in {}: {:.2} Offense, {:.2} Defense, {:.2} Grit, {:.2} Transition", season, offense_score, defense_score, grit_score, transition_score);
    }
}

// option 3: print the four scores of every team in a single season
fn print_scores_for_all_teams_in_season(data: &[HashMap<String, String>], season: &str) {
    let teams: Vec<String> = data.iter().filter_map(|row| row.get("team").map(|t| t.to_string())).collect();

    println!(" ");
    for team in teams.iter().unique() {
        let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores_for_team_in_season(data, team, season);
        
        println!("Scores for {}: {:.2} Offense, {:.2} Defense, {:.2} Grit, {:.2} Transition",
            team, offense_score, defense_score, grit_score, transition_score);
    }
}

// option 4: print the average of every teams' scores in every seasons
fn print_average_scores_for_seasons(data: &[HashMap<String, String>]) {
    let seasons: Vec<String> = data.iter().filter_map(|row| row.get("season").map(|s| s.to_string())).collect();

    println!(" ");
    for season in seasons.iter().unique() {
        let mut offense_total = 0.0;
        let mut defense_total = 0.0;
        let mut grit_total = 0.0;
        let mut transition_total = 0.0;
        let mut team_count = 0;

        let teams: Vec<String> = data.iter().filter_map(|row| row.get("team").map(|t| t.to_string())).collect();

        for team in teams.iter().unique() {
            let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores_for_team_in_season(data, team, season);

            if !offense_score.is_nan() && !defense_score.is_nan() && !grit_score.is_nan() && !transition_score.is_nan() {
                offense_total += offense_score;
                defense_total += defense_score;
                grit_total += grit_score;
                transition_total += transition_score;
                team_count += 1;
            }
        }

        if team_count > 0 {
            println!("Average Scores in {}: {:.2} Offense, {:.2} Defense, {:.2} Grit, {:.2} Transition",
                season, offense_total / team_count as f64, defense_total / team_count as f64, 
                grit_total / team_count as f64, transition_total / team_count as f64
            );
        } else {
            println!("No valid teams in season {} to calculate averages", season);
        }
    }
}

// option 5: print the averages of the top 10 and bottom 10 teams for each score in each season
fn print_top_bottom_averages(data: &[HashMap<String, String>]) {
    let seasons: Vec<String> = data.iter().filter_map(|row| row.get("season").map(|s| s.to_string())).collect();

    println!(" ");
    for season in seasons.iter().unique() {
        let mut teams_scores: Vec<(String, f64, f64, f64, f64)> = Vec::new();

        let teams: Vec<String> = data.iter().filter_map(|row| row.get("team").map(|t| t.to_string())).collect();

        for team in teams.iter().unique() {
            let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores_for_team_in_season(data, team, season);

            if !offense_score.is_nan() && !defense_score.is_nan() && !grit_score.is_nan() && !transition_score.is_nan() {
                teams_scores.push((team.clone(), offense_score, defense_score, grit_score, transition_score));
            }
        }

        let top_bottom_teams = |score_index: usize| -> (Vec<(String, f64)>, Vec<(String, f64)>) {
            let mut sorted_teams: Vec<(String, f64)> = teams_scores.iter().map(|(team, offense_score, defense_score, grit_score, transition_score)| {
                let score = match score_index {
                    0 => *offense_score,
                    1 => *defense_score,
                    2 => *grit_score,
                    3 => *transition_score,
                    _ => 0.0,
                };
                (team.clone(), score)
            }).collect();

            sorted_teams.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

            let top_10 = sorted_teams.iter().skip(sorted_teams.len() - 10).cloned().collect::<Vec<(String, f64)>>();
            let bottom_10 = sorted_teams.iter().take(10).cloned().collect::<Vec<(String, f64)>>();

            (top_10, bottom_10)
        };

        let calculate_avg = |teams: Vec<(String, f64)>| -> f64 {
            let sum: f64 = teams.iter().map(|(_, score)| score).sum();
            sum / teams.len() as f64
        };

        let print_avg = |score_name: &str, score_index: usize| -> (f64, f64) {
            let (top_10, bottom_10) = top_bottom_teams(score_index);
            let top_avg = calculate_avg(top_10);
            let bottom_avg = calculate_avg(bottom_10);
            (top_avg, bottom_avg)
        };

        let (offense_top_avg, offense_bottom_avg) = print_avg("Offense", 0);
        let (defense_top_avg, defense_bottom_avg) = print_avg("Defense", 1);
        let (grit_top_avg, grit_bottom_avg) = print_avg("Grit", 2);
        let (transition_top_avg, transition_bottom_avg) = print_avg("Transition", 3);

        println!("For {}:", season);
        println!("Top 10 teams: {:.2} Offense, {:.2} Defense, {:.2} Grit, {:.2} Transition", offense_top_avg, defense_top_avg, grit_top_avg, transition_top_avg);
        println!("Bottom 10 teams: {:.2} Offense, {:.2} Defense, {:.2} Grit, {:.2} Transition", offense_bottom_avg, defense_bottom_avg, grit_bottom_avg, transition_bottom_avg);
    }
}

// option 6: print the top 4 teams for each score in each season
fn print_top_teams_by_score(data: &[HashMap<String, String>]) {
    let seasons: Vec<String> = data.iter().filter_map(|row| row.get("season").map(|s| s.to_string())).collect();

    println!(" ");
    for season in seasons.iter().unique() {
        let mut teams_scores: Vec<(String, f64, f64, f64, f64)> = Vec::new();

        let teams: Vec<String> = data.iter().filter_map(|row| row.get("team").map(|t| t.to_string())).collect();

        for team in teams.iter().unique() {
            let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores_for_team_in_season(&data, team, season);
            
            if !offense_score.is_nan() && !defense_score.is_nan() && !grit_score.is_nan() && !transition_score.is_nan() {
                teams_scores.push((team.clone(), offense_score, defense_score, grit_score, transition_score));
            }
        }
        
        let mut top_teams = |score_index: usize| -> Vec<String> {
            let mut sorted_teams: Vec<(String, f64)> = teams_scores.iter().map(|(team, offense_score, defense_score, grit_score, transition_score)| {
                let score = match score_index {
                    0 => *offense_score,
                    1 => *defense_score,
                    2 => *grit_score,
                    3 => *transition_score,
                    _ => 0.0,
                };
                (team.clone(), score)
            }).collect();
            
            sorted_teams.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            
            sorted_teams.iter().take(4).map(|(team, _)| team.clone()).collect::<Vec<String>>()
        };

        println!("In {}:", season);
        let top_offense = top_teams(0);
        let top_defense = top_teams(1);
        let top_grit = top_teams(2);
        let top_transition = top_teams(3);

        println!("Top offense teams: {}", top_offense.join(", "));
        println!("Top defense teams: {}", top_defense.join(", "));
        println!("Top grit teams: {}", top_grit.join(", "));
        println!("Top transition teams: {}", top_transition.join(", "));
    }
}

fn main() {
    let data = read_csv("/opt/app-root/src/Project/all_game_stats.csv").unwrap();

    loop {
        println!("\nChoose an option:");
        println!("1: Print the four scores of a single team in a single season");
        println!("2: Print the four scores of a single team in every season");
        println!("3: Print the four scores of every team in a single season");
        println!("4: Print the average of every teams' scores in every seasons");
        println!("5: Print the averages of the top 10 and bottom 10 teams for each score in each season");
        println!("6: Print the top 4 teams for each score in each season");
        println!("7: Exit");
    
        let choice = prompt_for_input("Enter your choice: ");
        
        match choice.as_str() {
            "1" => {
                let team_name = prompt_for_input("Enter team name: ");
                let season = prompt_for_input("Enter season: ");
                print_single_team_season_scores(&data, &team_name, &season);
            },
            "2" => {
                let team_name = prompt_for_input("Enter team name: ");
                print_team_scores_for_all_seasons(&data, &team_name);
            },
            "3" => {
                let season = prompt_for_input("Enter season: ");
                print_scores_for_all_teams_in_season(&data, &season);
            },
            "4" => {
                print_average_scores_for_seasons(&data);
            },
            "5" => {
                print_top_bottom_averages(&data);
            },
            "6" => {
                print_top_teams_by_score(&data)
            },
            "7" => {
                println!("Exiting");
                break;
            },
            _ => {
                println!("Invalid choice, please select a valid option");
            }
        }
    }
}