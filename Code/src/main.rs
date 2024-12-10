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

fn read_csv(file_path: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let headers = rdr.headers()?.clone();
    let mut rows = Vec::new();

    for result in rdr.records() {
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

fn compute_all_scores_for_team_in_season(
    data: &Vec<HashMap<String, String>>,
    team_name: &str,
    season: &str,
) -> (f64, f64, f64, f64) {
    // Define benchmarks for all categories
    let offense_benchmarks = vec![
        (30.0, 0.2, 6.880765, true), //shots_on_goal
        (3.0, 0.45, 1.680705, true), //goals
        (0.45, 0.1, 0.137061, true), //cont_in_zone_after_shot
        (0.25, 0.05, 0.137306, false), //exit_zone_after_shot
        (3.5, 0.1, 0.962112, true), //exp_goals
        (4.0, 0.1, 1.933012, true), //dangerous_shots
    ];
    let defense_benchmarks = vec![
        (10.0, 0.15, 3.818616, true), //takeaways
        (26.0, 0.1, 6.880765, false), //opp_shots_on_goal
        (3.0, 0.1, 3.647803, false), //d_zone_giveaways
        (10.0, 0.05, 4.266148, true), //opp_missed_shots
        (2.0, 0.4, 1.680705, false), //opp_goals
        (2.33, 0.1, 0.962112, false), //opp_exp_goals
        (2.5, 0.1, 1.933012, false), //opp_dangerous_shots
    ];
    let grit_benchmarks = vec![
        (25.0, 0.45, 8.754365, true), //hits
        (10.0, 0.3, 5.175412, true), //blocks
        (0.4, 0.05, 0.143277, false), //penalties_for_perc
        (10.0, 0.15, 4.952046, true), //opp_giveaways
        (5.0, 0.05, 3.647803, true), //opp_d_zone_giveaways
    ];
    let transition_benchmarks = vec![
        (0.55, 0.1, 0.073265, true), //faceoff_win_perc
        (0.35, 0.2, 0.137061, false), //opp_cont_in_zone_after_shot
        (0.35, 0.2, 0.137306, true), //opp_exit_zone_after_shot
        (8.0, 0.4, 4.952046, false), //giveaways
        (13.0, 0.1, 3.818616, false), //opp_takeaways
    ];

    // Compute the four scores
    let offense_score = compute_offense_score(&data, &team_name, &season, &offense_benchmarks);
    let defense_score = compute_defense_score(&data, &team_name, &season, &defense_benchmarks);
    let grit_score = compute_grit_score(&data, &team_name, &season, &grit_benchmarks);
    let transition_score = compute_transition_score(&data, &team_name, &season, &transition_benchmarks);

    (offense_score, defense_score, grit_score, transition_score)
}

fn prompt_for_input(prompt: &str) -> String {
    let mut input = String::new();
    print!("{}", prompt);
    io::stdout().flush().unwrap(); // Ensure prompt is displayed immediately
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn main() {
    let data = read_csv("/opt/app-root/src/Project/all_game_stats.csv").unwrap();

    loop {
        println!("\nChoose an option:");
        println!("1: Print the four scores of a single team in a single season");
        println!("2: Print the four scores of a single team in every season");
        println!("3: Print the four scores of every team in a single season");
        println!("4: Print the average of every teams' scores in every seasons");
        println!("5: Exit");

        let choice = prompt_for_input("Enter your choice: ");
        
        match choice.as_str() {
            "1" => {
                // Option 1: Print the four scores of a single team in a single season.
                let team_name = prompt_for_input("Enter team name: ");
                let season = prompt_for_input("Enter season: ");
                
                let (offense_score, defense_score, grit_score, transition_score) =
                    compute_all_scores_for_team_in_season(&data, &team_name, &season);
                
                println!("\nFor {} in {}:", team_name, season);
                println!("Offensive Score: {:.2}", offense_score);
                println!("Defensive Score: {:.2}", defense_score);
                println!("Grit Score: {:.2}", grit_score);
                println!("Transition Score: {:.2}", transition_score);
            },
            "2" => {
                // Option 2: Print the four scores of a single team in every season.
                let team_name = prompt_for_input("Enter team name: ");
                
                let seasons: Vec<String> = data.iter().filter_map(|row| row.get("season").map(|s| s.to_string())).collect();
                
                println!(" ");
                for season in seasons.iter().unique() {  // Unique seasons only
                    let (offense_score, defense_score, grit_score, transition_score) =
                        compute_all_scores_for_team_in_season(&data, &team_name, season);
                    
                    println!("Scores in {}: {:.2} Offense, {:.2} Defense, {:.2} Grit, {:.2} Transition", 
                        season, offense_score, defense_score, grit_score, transition_score);
                }
            },
            "3" => {
                // Option 3: Print the four scores of every team in a single season.
                let season = prompt_for_input("Enter season: ");
                
                let teams: Vec<String> = data.iter().filter_map(|row| row.get("team").map(|t| t.to_string())).collect();

                println!(" ");
                for team in teams.iter().unique() {  // Unique teams only
                    let (offense_score, defense_score, grit_score, transition_score) =
                        compute_all_scores_for_team_in_season(&data, team, &season);
                    
                    println!("Scores for {}: {:.2} Offense, {:.2} Defense, {:.2} Grit, {:.2} Transition",
                        team, offense_score, defense_score, grit_score, transition_score);
                }
            },
            "4" => {
                // Option 4: For each season, print the average of all teams' four scores.
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
                        let (offense_score, defense_score, grit_score, transition_score) =
                            compute_all_scores_for_team_in_season(&data, team, season);
            
                        // Ensure that it doesn't increment team_count if scores are NaN (invalid)
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
            },
            "5" => {
                println!("Exiting");
                break;
            },
            _ => {
                println!("Invalid choice, please select a valid option");
            }
        }
    }
}