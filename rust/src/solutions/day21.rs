use std::collections::{HashMap, HashSet};

pub fn solve() {
    println!("Day 21");

    let input_lines = adventlib::read_input_lines("day21input.txt");

    let mut full_ingredient_listing: Vec<&str> = Vec::new();
    let mut possible_ingredients: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in &input_lines {
        let parts: Vec<_> = line.split(" (contains ").collect();
        let ingredients: HashSet<_> = parts[0].split(" ").collect();
        let alergens: Vec<_> = parts[1].trim_end_matches(")").split(", ").collect();

        full_ingredient_listing.extend(ingredients.iter());

        for alergen in &alergens {
            if possible_ingredients.contains_key(alergen) {
                let previous_set = possible_ingredients.remove(alergen).unwrap();
                let new_set: HashSet<_> =
                    previous_set.intersection(&ingredients).cloned().collect();
                possible_ingredients.insert(alergen, new_set);
            } else {
                possible_ingredients.insert(
                    alergen,
                    ingredients.iter().cloned().collect::<HashSet<&str>>(),
                );
            }
        }
    }

    let mut ingredients_with_alergen = HashSet::new();
    let mut ingredient_alergen_pairs = HashSet::new();

    while possible_ingredients.len() > 0 {
        let alergen: &str = possible_ingredients
            .iter()
            .filter(|(_alergen, set)| set.len() == 1)
            .map(|(&alergen, _set)| alergen)
            .next()
            .expect("Might need some better logic.");

        let set_of_one = possible_ingredients.remove(alergen).unwrap();
        let ingredient = set_of_one.iter().cloned().next().unwrap();
        ingredients_with_alergen.insert(ingredient);
        ingredient_alergen_pairs.insert((ingredient, alergen));

        for ingredient_list in possible_ingredients.values_mut() {
            ingredient_list.remove(ingredient);
        }
    }

    let safe_count = full_ingredient_listing
        .iter()
        .filter(|&ingredient| !ingredients_with_alergen.contains(ingredient))
        .count();

    println!("Output (part 1): {}", safe_count);

    let mut unsafe_ingredients: Vec<_> = ingredient_alergen_pairs.iter().cloned().collect();
    unsafe_ingredients.sort_by_key(|(_i, a)| *a);
    let canonical_list: Vec<_> = unsafe_ingredients
        .iter()
        .cloned()
        .map(|(i, _a)| i)
        .collect();
    println!("Output (part 2): {}", canonical_list.join(","));
}
