fn main() {
    use  std::collections::HashMap;
    let teams = vec![String::from("Blue"), String::from("Yello")];
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    for (key, value) in &scores{
        println!("{}: {}", key, value );
    }
}


