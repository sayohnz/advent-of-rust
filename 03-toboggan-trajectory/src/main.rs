fn tree_at(mountain: Vec<String>, row: usize, col: usize) -> bool {
    let point = mountain[row].chars().nth(col).unwrap();
    point == '#'
}

fn ski(mountain: Vec<String>, slope: (usize, usize)) -> usize {
    let mut hit_trees = 0;
    let mut pos = (0, 0);

    while pos.0 < mountain.len() {
        if tree_at(mountain.clone(), pos.0, pos.1) {
            hit_trees += 1;
        }

        pos.0 += slope.0;
        pos.1 = (pos.1 + slope.1) % mountain[0].chars().count();
    }

    hit_trees
}

fn main() {
    let mountain = readinput("0.in".to_string());
    println!("{}", ski(mountain.clone(), (1, 3)));

    let mut prod_hit_trees = 1;
    for slope in vec![(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        prod_hit_trees *= ski(mountain.clone(), slope);
    }
    println!("{}", prod_hit_trees);
}

fn readinput(filename: String) -> Vec<String> {
    let input = std::fs::read_to_string(filename).unwrap();
    let entries = input
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<String>().unwrap())
        .collect::<Vec<_>>();
    return entries;
}
