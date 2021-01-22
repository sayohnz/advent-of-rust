#[derive(Debug, Clone)]
struct Password {
    lo: usize,
    hi: usize,
    ch: char,
    pass: String,
}

impl Password {
    fn new(raw_line: &str) -> Self {
        let tokens = raw_line.split(" ").collect::<Vec<&str>>();
        let limits = tokens[0]
            .split("-")
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        let target = tokens[1].chars().next().unwrap();
        let passw = tokens[2];

        return Password {
            lo: limits[0],
            hi: limits[1],
            ch: target,
            pass: passw.to_string(),
        };
    }

    fn valid(&self) -> bool {
        let matches = self.pass.matches(self.ch).count();
        return self.lo <= matches && matches <= self.hi;
    }

    fn new_policy(&self) -> bool {
        (self.pass.chars().nth(self.lo - 1).unwrap() == self.ch)
            ^ (self.pass.chars().nth(self.hi - 1).unwrap() == self.ch)
    }
}

fn readinput(filename: String) -> Vec<Password> {
    let input = std::fs::read_to_string(filename).unwrap();
    let entries = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(Password::new)
        .collect::<Vec<_>>();
    return entries;
}

fn main() {
    let passwords = readinput("0.in".to_string());

    // part I
    println!(
        "{}",
        passwords
            .clone()
            .into_iter()
            .filter(|pass| pass.valid())
            .count()
    );

    // part II
    println!(
        "{}",
        passwords
            .clone()
            .into_iter()
            .filter(|pass| pass.new_policy())
            .count()
    );
}
