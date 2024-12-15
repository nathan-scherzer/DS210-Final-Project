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
use crate::playstyles::compute_all_scores;

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
                if header == "team" || header == "opponent" {
                    row.insert(header.to_string(), field.to_string());
                } else {
                    let value: String = if let Ok(parsed) = field.parse::<f64>() {
                        parsed.to_string()
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

fn prompt_for_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

// option 1: print the four scores of a single team in a single season]
fn single_team_and_season(data: &[HashMap<String, String>], team_name: &str, season: &str) {
    let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores(data, team_name, season);
    
    println!("\nFor the {} in {}:", team_name, season);
    println!("Offense Score: {:.2}", offense_score);
    println!("Defense Score: {:.2}", defense_score);
    println!("Grit Score: {:.2}", grit_score);
    println!("Transition Score: {:.2}", transition_score);
}

// option 2: print the four scores of a single team in every season
fn single_team_all_seasons(data: &[HashMap<String, String>], team_name: &str) {
    let seasons: Vec<String> = data.iter().filter_map(|row| row.get("season").map(|s| s.to_string())).collect();
    
    println!("\nFor the {}:", team_name);
    for season in seasons.iter().unique() {
        let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores(data, team_name, season);
        println!("Scores in {}: {:.2} Offense, {:.2} Defense, {:.2} Grit, {:.2} Transition", season, offense_score, defense_score, grit_score, transition_score);
    }
}

// option 3: print the four scores of every team in a single season
fn all_teams_single_season(data: &[HashMap<String, String>], season: &str) {
    let teams: Vec<String> = data.iter().filter_map(|row| row.get("team").map(|t| t.to_string())).collect();

    println!("\nIn {}:", season);
    for team in teams.iter().unique() {
        let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores(data, team, season);
        
        println!("Scores for {}: {:.2} Offense, {:.2} Defense, {:.2} Grit, {:.2} Transition",
            team, offense_score, defense_score, grit_score, transition_score);
    }
}

// option 4: print the average of every teams' scores in every seasons
fn team_averages_for_all_seasons(data: &[HashMap<String, String>]) {
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
            let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores(data, team, season);

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

// option 5: print the top 4 teams for each score in each season
fn top_teams_by_season(data: &[HashMap<String, String>]) {
    let seasons: Vec<String> = data.iter().filter_map(|row| row.get("season").map(|s| s.to_string())).collect();

    println!(" ");
    for season in seasons.iter().unique() {
        let mut teams_scores: Vec<(String, f64, f64, f64, f64)> = Vec::new();

        let teams: Vec<String> = data.iter().filter_map(|row| row.get("team").map(|t| t.to_string())).collect();

        for team in teams.iter().unique() {
            let (offense_score, defense_score, grit_score, transition_score) = compute_all_scores(&data, team, season);
            
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
        println!("4: Print the average of every teams' scores in every season");
        println!("5: Print the top 4 teams for each score in each season");
        println!("6: Exit");
    
        let choice = prompt_for_input("Enter your choice: ");
        
        match choice.as_str() {
            "1" => {
                let team_name = prompt_for_input("Enter team name: ");
                let season = prompt_for_input("Enter season: ");
                single_team_and_season(&data, &team_name, &season);
            },
            "2" => {
                let team_name = prompt_for_input("Enter team name: ");
                single_team_all_seasons(&data, &team_name);
            },
            "3" => {
                let season = prompt_for_input("Enter season: ");
                all_teams_single_season(&data, &season);
            },
            "4" => {
                team_averages_for_all_seasons(&data);
            },
            "5" => {
                top_teams_by_season(&data)
            },
            "6" => {
                println!("Exiting");
                break;
            },
            _ => {
                println!("Invalid choice, please select a valid option");
            }
        }
    }
}