use std::io::BufRead;

fn main() {
    let lines : Vec<Vec<char>> = std::io::stdin().lock().lines().map(|l| l.unwrap().chars().collect()).collect();

    let mut total = 0;
    for line in lines {
//        let pick_score = match line[2] {
//            'X' => 1,
//            'Y' => 2,
//            'Z' => 3,
//            _ => panic!(),
//        };
//
//        let outcome = match (line[0], line[2]) {
//            ('A', 'X') => 3,
//            ('A', 'Y') => 6,
//            ('A', 'Z') => 0,
//
//            ('B', 'X') => 0,
//            ('B', 'Y') => 3,
//            ('B', 'Z') => 6,
//
//            ('C', 'X') => 6,
//            ('C', 'Y') => 0,
//            ('C', 'Z') => 3,
//            _          => panic!(),
//        };

        let pick = match (line[0], line[2]) {
            ('A', 'X') => 'C',
            ('A', 'Y') => 'A',
            ('A', 'Z') => 'B',

            ('B', 'X') => 'A',
            ('B', 'Y') => 'B',
            ('B', 'Z') => 'C',

            ('C', 'X') => 'B',
            ('C', 'Y') => 'C',
            ('C', 'Z') => 'A',
            _          => panic!(),
        };

        let pick_score = match pick {
            'A' => 1,
            'B' => 2,
            'C' => 3,
            _ => panic!(),
        };

        let outcome = match (line[0], pick) {
            ('A', 'A') => 3,
            ('A', 'B') => 6,
            ('A', 'C') => 0,

            ('B', 'A') => 0,
            ('B', 'B') => 3,
            ('B', 'C') => 6,

            ('C', 'A') => 6,
            ('C', 'B') => 0,
            ('C', 'C') => 3,
            _          => panic!(),
        };

        total += pick_score + outcome;
    }

    println!("total: {}", total);
}
