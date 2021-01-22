// read i32 line by line
fn readinput(filename: String) -> Vec<i32> {
    let input = std::fs::read_to_string(filename).unwrap();
    let entries = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    return entries;
}

fn match_all_pairs(
    entries: &Vec<i32>,
    target: i32,
    avoid_index: i32,
    should_print: bool,
) -> (bool, i32, i32) {
    for (i, element) in entries.iter().enumerate() {
        for (j, other_element) in entries.iter().enumerate() {
            if i == j || i as i32 == avoid_index || j as i32 == avoid_index {
                continue;
            }

            if element + other_element == target {
                if should_print {
                    println!(
                        "{}=({} + {})\t|\t P: {}",
                        element + other_element,
                        element,
                        other_element,
                        element * other_element
                    );
                }
                return (true, *element, *other_element);
            }
        }
    }
    return (false, -1, -1);
}

fn match_all_triples(entries: &Vec<i32>, target: i32) -> () {
    for (i, element) in entries.iter().enumerate() {
        let (matched, one, two) = match_all_pairs(entries, target - element, i as i32, false);
        if matched {
            println!(
                "{}=({} + {} + {})  |\t P: {}",
                element + one + two,
                element,
                one,
                two,
                element * one * two 
            );
            return;
        }
    }
}

fn main() {
    let entries = readinput("0.in".to_string());
    match_all_pairs(&entries, 2020, -1, true);
    match_all_triples(&entries, 2020);
}
