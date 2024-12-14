use crate::playstyles::calculate_z_score;
use std::collections::HashMap;
use crate::compute_offense_score;
use crate::compute_grit_score;

#[test]
fn test_calculate_z_score() {
    let value = 10.0;
    let benchmark = 5.0;
    let weight = 2.0;
    let std_dev = 2.5;
    let result = calculate_z_score(value, benchmark, weight, std_dev);
    assert_eq!(result, 4.0); // (10 - 5) / 2.5 * 2 = 4.0
}

#[test]
fn test_compute_offense_score() {
    let data = vec![
        HashMap::from([
            ("team".to_string(), "TeamA".to_string()),
            ("season".to_string(), "2023".to_string()),
            ("shots_on_goal".to_string(), "36".to_string()),
            ("goals".to_string(), "4".to_string()),
            ("cont_in_zone_after_shot".to_string(), "0.4".to_string()),
            ("exit_zone_after_shot".to_string(), "0.55".to_string()),
            ("exp_goals".to_string(), "4.5".to_string()),
            ("dangerous_shots".to_string(), "4".to_string()),
        ]),
    ];

    let benchmarks = vec![
        (32.8, 0.2, 6.880765, true), //shots_on_goal
        (3.0, 0.45, 1.680705, true), //goals
        (0.48, 0.1, 0.137061, true), //cont_in_zone_after_shot
        (0.49, 0.05, 0.137306, false), //exit_zone_after_shot
        (3.0, 0.1, 0.962112, true), //exp_goals
        (2.9, 0.1, 1.933012, true), //dangerous_shots
    ];

    let score = compute_offense_score(&data, "TeamA", "2023", &benchmarks);
    let rounded_score = (score * 1000.0).round() / 1000.0;

    assert_eq!(rounded_score, 0.493);
}

#[test]
fn test_compute_grit_score() {

    // Sample data for the team "TeamA" in the 2023 season
    let data = vec![
        HashMap::from([
            ("team".to_string(), "TeamA".to_string()),
            ("season".to_string(), "2023".to_string()),
            ("hits".to_string(), "19".to_string()),
            ("blocks".to_string(), "14".to_string()),
            ("penalties_for_perc".to_string(), "0.44".to_string()),
            ("opp_giveaways".to_string(), "8".to_string()),
            ("opp_d_zone_giveaways".to_string(), "3".to_string()),
        ]),
    ];

    // Benchmarks for the statistics (hits, blocks, etc.)
    let benchmarks = vec![
        (24.2, 0.45, 8.754365, true), //hits
        (15.3, 0.3, 5.175412, true), //blocks
        (0.46, 0.05, 0.143277, false), //penalties_for_perc
        (9.3, 0.15, 4.952046, true), //opp_giveaways
        (5.3, 0.05, 3.647803, true), //opp_d_zone_giveaways
    ];

    // Call the function under test
    let score = compute_grit_score(&data, "TeamA", "2023", &benchmarks);

    // Round the result for more predictable testing
    let rounded_score = (score * 1000.0).round() / 1000.0; // Round to 3 decimal places

    // Compare with a small tolerance
    assert_eq!(rounded_score, -0.407);
}