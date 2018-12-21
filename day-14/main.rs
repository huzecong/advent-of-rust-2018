use std::fs;

fn concat(arr: &[usize]) -> String {
    arr.iter().map(|&x| x.to_string()).collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").ok().unwrap();
    let steps = input.trim().parse::<usize>().ok().unwrap();
    let match_recipe = steps.to_string().chars()
            .map(|x| x.to_digit(10).unwrap() as usize)
            .collect::<Vec<_>>();
    const RECIPES: usize = 10;

    // Parts 1 & 2
    let mut scores: Vec<usize> = vec![];
    scores.extend(&[3, 7]);
    let mut p1 = 0;
    let mut p2 = 1;
    let mut found: Option<usize> = None;
    while scores.len() < steps + RECIPES || found == None {
        let sum = scores[p1] + scores[p2];
        if sum >= 10 { scores.push(sum / 10); }
        scores.push(sum % 10);
        p1 = (p1 + scores[p1] + 1) % scores.len();
        p2 = (p2 + scores[p2] + 1) % scores.len();
        if found == None && scores.len() > match_recipe.len() {
            let start = scores.len() - match_recipe.len();
            if &scores[(start - 1)..(scores.len() - 1)] == &match_recipe[..] {
                found = Some(start - 1);
            } else if &scores[start..] == &match_recipe[..] {
                found = Some(start);
            }
        }
    }
    let recipes = concat(&scores[steps..(steps + RECIPES)]);
    println!("{}", recipes);
    println!("{}", found.unwrap());
}
