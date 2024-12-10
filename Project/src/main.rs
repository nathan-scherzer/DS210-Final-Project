//use std::collections::HashMap;
//use std::io;
//mod playstyles;
//use crate::playstyles::read_csv;
//use crate::playstyles::compute_std_devs;

//fn main() {
    // Load CSV data
   // let (headers, rows) = read_csv("path/to/your/all_game_stats.csv").expect("Failed to read CSV file");

    // Print the headers to ensure they are being read correctly
   // println!("Headers: {:?}", headers);

    // Print the first few rows to verify the data
  //  let num_rows_to_print = 5; // Print first 5 rows
  //  for (i, row) in rows.iter().take(num_rows_to_print).enumerate() {
      //  println!("Row {}: {:?}", i + 1, row);
  //  }

    // Parse user input
    //let mut input = String::new();
    //println!("Enter team:");
    //io::stdin().read_line(&mut input).unwrap();
    //let team = input.trim();

    //input.clear();
   //println!("Enter season:");
   // io::stdin().read_line(&mut input).unwrap();
    //let season: u32 = input.trim().parse().expect("Invalid season input");

    // Map rows to structured data (assumes proper CSV column alignment)
   // let parsed_data: Vec<HashMap<String, f32>> = rows.iter()
        //.map(|row| {
        //    headers.iter()
        //        .zip(row.iter())
         //       .filter_map(|(header, value)| value.parse::<f32>().ok().map(|v| (header.clone(), v)))
         //       .collect()
       // })
       // .collect();

    // Find the specific team's data
    //let team_data = parsed_data.iter()
    //.find(|row| row.get("team").map_or(false, |t: &str| t == team) &&
    //            row.get("season").map_or(false, |s| s.parse::<u32>().ok() == Some(season)))
   // .expect("Team and season combination not found");

    // Compute standard deviations
   // let std_devs = compute_std_devs(&parsed_data);

    // Define benchmarks
   // let offense_benchmarks = Vec<(f64, f64, bool)> = vec![
      //  (30.0, *std_devs.get("shots_on_goal"), true),
      //  (3.0, *std_devs.get("goals"), true),
      //  (0.45, *std_devs.get("cont_in_zone_after_shot"), true),
      //  (0.25, *std_devs.get("exit_zone_after_shot"), true),
      //  (3.5, *std_devs.get("exp_goals"), true),
      //  (4.0, *std_devs.get("dangerous_shots"), true),
   // ];

    //let defense_benchmarks = Vec<(f64, f64, bool)> = vec![
      //  (10.0, *std_devs.get("takeaways"), true),
       // (26.0, *std_devs.get("opp_shots_on_goal"), false),
      //  (3.0, *std_devs.get("d_zone_giveaways"), false),
      //  (10.0, *std_devs.get("opp_missed_shots"), true),
      //  (2.0, *std_devs.get("opp_goals"), false),
      //  (2.33, *std_devs.get("opp_exp_goals"), false),
      //  (2.5, *std_devs.get("opp_dangerous_shots"), false),
   // ];

   // let grit_benchmarks = Vec<(f64, f64, bool)> = vec![
       // (25.0, *std_devs.get("hits"), true),
       // (10.0, *std_devs.get("blocks"), true),
      //  (0.4, *std_devs.get("penalties_for_perc"), false),
      //  (10.0, *std_devs.get("opp_giveaways"), true),
      //  (5.0, *std_devs.get("opp_d_zone_giveaways"), true),
   // ];

   // let transition_benchmarks = Vec<(f64, f64, bool)> = vec![
      //  (0.55, *std_devs.get("faceoff_win_perc"), true),
      //  (0.35, *std_devs.get("opp_cont_in_zone_after_shot"), false),
      //  (0.35, *std_devs.get("opp_exit_zone_after_shot"), false),
      //  (8.0, *std_devs.get("giveaways"), false),
      //  (13.0, *std_devs.get("opp_takeaways"), false),
    //];

    // Map data to specific playstyle structs and calculate scores
   // let offense = Offense {
       // shots_on_goal: team_data["shots_on_goal"],
       // goals: team_data["goals"],
      //  cont_in_zone_after_shot: team_data["cont_in_zone_after_shot"],
      //  exit_zone_after_shot: team_data["exit_zone_after_shot"],
      //  exp_goals: team_data["exp_goals"],
      //  dangerous_shots: team_data["dangerous_shots"],
    //};

    //let defense = Defense {
       // takeaways: team_data["takeaways"],
       // opp_shots_on_goal: team_data["opp_shots_on_goal"],
      //  d_zone_giveaways: team_data["d_zone_giveaways"],
      //  opp_missed_shots: team_data["opp_missed_shots"],
      //  opp_goals: team_data["opp_goals"],
      //  opp_exp_goals: team_data["opp_exp_goals"],
      //  opp_dangerous_shots: team_data["opp_dangerous_shots"],
   // };

    //let grit = Grit {
       // hits: team_data["hits"],
      //  blocks: team_data["blocks"],
      //  penalties_for_perc: team_data["penalties_for_perc"],
      //  opp_giveaways: team_data["opp_giveaways"],
      //  opp_d_zone_giveaways: team_data["opp_d_zone_giveaways"],
    //};

   // let transition = Transition {
      //  faceoff_win_perc: team_data["faceoff_win_perc"],
      //  opp_cont_in_zone_after_shot: team_data["opp_cont_in_zone_after_shot"],
      //  opp_exit_zone_after_shot: team_data["opp_exit_zone_after_shot"],
      //  giveaways: team_data["giveaways"],
      //  opp_takeaways: team_data["opp_takeaways"],
   // };

    // Compute playstyle scores
   // let offense_score = offense.compute_offense_score(&offense_benchmarks);
   // let defense_score = defense.compute_defense_score(&defense_benchmarks);
   // let grit_score = grit.compute_grit_score(&grit_benchmarks);
   // let transition_score = transition.compute_transition_score(&transition_benchmarks);

    // Print scores
    //println!(
    //    "Scores for {} in {}:\nOffense: {:.2}\nDefense: {:.2}\nGrit: {:.2}\nTransition: {:.2}",
    //    team, season, offense_score, defense_score, grit_score, transition_score
   // );
//}



/*
fn read_csv(file_path: &str) -> Result<Vec<HashMap<String, f64>>, Box<dyn std::error::Error>> {
    let mut rdr = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
    let headers = rdr.headers()?.clone();
    let mut rows = Vec::new();

    for result in rdr.records() {
        let record = result?;
        let mut row = HashMap::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                let value: f64 = field.parse().unwrap_or(0.0);
                row.insert(header.to_string(), value);
            }
        }
        rows.push(row);
    }
    Ok(rows)
}
*/

use std::collections::HashMap;
use std::error::Error;
use csv::ReaderBuilder;
use std::fs::File;
use std::io::{self, Write};

mod playstyles;
use crate::playstyles::Offense;
use crate::playstyles::Defense;
use crate::playstyles::Grit;
use crate::playstyles::Transition;

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

fn main() {
    let data = read_csv("/opt/app-root/src/Project/all_game_stats.csv").unwrap();

    let mut team_name = String::new();
    let mut season = String::new();

    // Prompt user for team name
    print!("Enter team name: ");
    io::stdout().flush().unwrap(); // Ensure prompt is displayed immediately
    let mut team_name = String::new();
    io::stdin().read_line(&mut team_name).unwrap();
    let team_name = team_name.trim(); // Trim to remove any trailing newline or spaces

    // Prompt user for season
    print!("Enter season: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut season).unwrap();
    let season = season.trim(); // Trim to remove any trailing newline or spaces

    let offense_benchmarks: Vec<(f64, f64, bool)> = vec![
        (30.0, 6.880765, true), //shots_on_goal
        (3.0, 1.680705, true), //goals
        (0.45, 0.137061, true), //cont_in_zone_after_shot
        (0.25, 0.137306, true), //exit_zone_after_shot
        (3.5, 0.962112, true), //exp_goals
        (4.0, 1.933012, true), //dangerous_shots
    ];

    let defense_benchmarks: Vec<(f64, f64, bool)> = vec![
        (10.0, 3.818616, true), //takeaways
        (26.0, 6.880765, false), //opp_shots_on_goal
        (3.0, 3.647803, false), //d_zone_giveaways
        (10.0, 4.266148, true), //opp_missed_shots
        (2.0, 1.680705, false), //opp_goals
        (2.33, 0.962112, false), //opp_exp_goals
        (2.5, 1.933012, false), //opp_dangerous_shots
    ];

    let grit_benchmarks: Vec<(f64, f64, bool)> = vec![
        (25.0, 8.754365, true), //hits
        (10.0, 5.175412, true), //blocks
        (0.4, 0.143277, false), //penalties_for_perc
        (10.0, 4.952046, true), //opp_giveaways
        (5.0, 3.647803, true), //opp_d_zone_giveaways
    ];

    let transition_benchmarks: Vec<(f64, f64, bool)> = vec![
        (0.55, 0.073265, true), //faceoff_win_perc
        (0.35, 0.137061, false), //opp_cont_in_zone_after_shot
        (0.35, 0.137306, false), //opp_exit_zone_after_shot
        (8.0, 4.952046, false), //giveaways
        (13.0, 3.818616, false), //opp_takeaways
    ];

    let offense_score = Offense::compute_offense_score(&data, team_name, season, &offense_benchmarks);
    let defense_score = Defense::compute_defense_score(&data, team_name, season, &defense_benchmarks);
    let grit_score = Grit::compute_grit_score(&data, team_name, season, &grit_benchmarks);
    let transition_score = Transition::compute_transition_score(&data, team_name, season, &transition_benchmarks);

    println!("\nFor {} in {}:", team_name, season);
    println!("\nOffense Score: {:.2}", offense_score);
    println!("Defense Score: {:.2}", defense_score);
    println!("Grit Score: {:.2}", grit_score);
    println!("Transition Score: {:.2}", transition_score);
}


   // One you have team name and season, need extract stats for that, and then put them into structs
    // team_offense = Offense {
       // shots_on_goal:
        //goals:
    //}

   // shots_on_goal, goals
   // fn build_offense(shots_on_goal, goals, etc) -> Offense {
      //  let offense = Offense {
       //     shots_on_goal: shots_on_goal,
       //     goals: goals
       // }
    //}

  //  for row in data {
   // }
    /*
        let offense = Offense {
            shots_on_goal: *row.get(&"shots_on_goal").unwrap(),
            goals: 0.0,
            cont_in_zone_after_shot: 0.0,
            exit_zone_after_shot: 0.0,
            exp_goals: 0.0,
            dangerous_shots: 0.0,
        };

        */
        /*
        // Parse the stats from the row (make sure to handle any missing or malformed data)
        let offense = Offense {
            shots_on_goal: row.get("shots_on_goal").unwrap_or("0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            goals: row.get("goals").unwrap_or("0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            cont_in_zone_after_shot: row.get("cont_in_zone_after_shot").unwrap_or("0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            exit_zone_after_shot: row.get("exit_zone_after_shot").unwrap_or("0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            exp_goals: row.get("exp_goals").unwrap_or("0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            dangerous_shots: row.get("dangerous_shots").unwrap_or("0.0".to_string()).parse::<f64>().unwrap_or(0.0),
        };

        let defense = Defense {
            takeaways: row.get("takeaways").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            opp_shots_on_goal: row.get("opp_shots_on_goal").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            d_zone_giveaways: row.get("d_zone_giveaways").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            opp_missed_shots: row.get("opp_missed_shots").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            opp_goals: row.get("opp_goals").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            opp_exp_goals: row.get("opp_exp_goals").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            opp_dangerous_shots: row.get("opp_dangerous_shots").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
        };

        let grit = Grit {
            hits: row.get("hits").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            blocks: row.get("blocks").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            penalties_for_perc: row.get("penalties_for_perc").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            opp_giveaways: row.get("opp_giveaways").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            opp_d_zone_giveaways: row.get("opp_d_zone_giveaways").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
        };

        let transition = Transition {
            faceoff_win_perc: row.get("faceoff_win_perc").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            opp_cont_in_zone_after_shot: row.get("opp_cont_in_zone_after_shot").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            opp_exit_zone_after_shot: row.get("opp_exit_zone_after_shot").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
            giveaways: row.get("giveaways").unwrap_or(&"0.0".to_string()).parse::<f64>()..unwrap_or(0.0),
            opp_takeaways: row.get("opp_takeaways").unwrap_or(&"0.0".to_string()).parse::<f64>().unwrap_or(0.0),
        };

        let offense_score = offense.compute_offense_score(&offense_benchmarks);
        let defense_score = defense.compute_defense_score(&defense_benchmarks);
        let grit_score = grit.compute_grit_score(&grit_benchmarks);
        let transition_score = transition.compute_transition_score(&transition_benchmarks);

        println!("Team: {} - Season: {}", row.get("team_name").unwrap(), row.get("season").unwrap());
        println!("Offense Score: {:.2}", offense_score);
        println!("Defense Score: {:.2}", defense_score);
        println!("Grit Score: {:.2}", grit_score);
        println!("Transition Score: {:.2}", transition_score);
        println!("---");
    }

    Ok(())
}
*/