use std::io::BufRead;

fn main() {
    let grid: Vec<Vec<char>> = std::io::stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().chars().collect::<Vec<_>>())
        .collect();

    let width = grid[0].len();
    let height = grid.len();

    let mut xmas_count = 0;

    // horz
    for y in 0..height {
        for x in 0..width {
            if grid[y]
                .iter()
                .skip(x)
                .collect::<String>()
                .starts_with("XMAS")
            {
                xmas_count += 1
            }

            if grid[y]
                .iter()
                .rev()
                .skip(x)
                .collect::<String>()
                .starts_with("XMAS")
            {
                xmas_count += 1
            }
        }
    }

    // vert
    for x in 0..width {
        let mut str: Vec<char> = Vec::new();
        for y in 0..height {
            str.push(grid[y][x]);
        }

        for i in 0..str.len() {
            if str.iter().skip(i).collect::<String>().starts_with("XMAS") {
                xmas_count += 1
            }

            if str
                .iter()
                .rev()
                .skip(i)
                .collect::<String>()
                .starts_with("XMAS")
            {
                xmas_count += 1
            }
        }
    }

    // SW <-> NE diagnally
    for c in 0..height + width - 1 {
        let (mut x, mut y) = if c < height {
            (0, c)
        } else {
            (c - height + 1, height - 1)
        };

        let mut str: Vec<char> = Vec::new();

        str.push(grid[y][x]);
        while y > 0 && x < width - 1 {
            x += 1;
            y -= 1;
            str.push(grid[y][x]);
        }

        for i in 0..str.len() {
            if str.iter().skip(i).collect::<String>().starts_with("XMAS") {
                xmas_count += 1;
            }

            if str
                .iter()
                .rev()
                .skip(i)
                .collect::<String>()
                .starts_with("XMAS")
            {
                xmas_count += 1;
            }
        }
    }

    // SE <-> NW diagnally
    for c in 0..height + width - 1 {
        let (mut x, mut y) = if c < height {
            (width - 1, c)
        } else {
            (width - 1 - (c - height) - 1, height - 1)
        };

        let mut str: Vec<char> = Vec::new();

        str.push(grid[y][x]);
        while y > 0 && x > 0 {
            x -= 1;
            y -= 1;
            str.push(grid[y][x]);
        }

        for i in 0..str.len() {
            if str.iter().skip(i).collect::<String>().starts_with("XMAS") {
                xmas_count += 1;
            }

            if str
                .iter()
                .rev()
                .skip(i)
                .collect::<String>()
                .starts_with("XMAS")
            {
                xmas_count += 1;
            }
        }
    }

    println!("XMAS count: {xmas_count}");

    let mut x_mas_count = 0;

    for y in 1..height-1 {
        for x in 1..width-1 {
            let mut mas1: Vec<char> = Vec::new();
            let mut mas2: Vec<char> = Vec::new();

            mas1.push(grid[y-1][x-1]);
            mas1.push(grid[y][x]);
            mas1.push(grid[y+1][x+1]);

            mas2.push(grid[y-1][x+1]);
            mas2.push(grid[y][x]);
            mas2.push(grid[y+1][x-1]);


            let mas1 = mas1.iter().collect::<String>();
            let mas2 = mas2.iter().collect::<String>();

            if (mas1 == "MAS" || mas1 == "SAM") && (mas2 == "MAS" || mas2 == "SAM") {
                x_mas_count += 1;
            }
        }
    }

    println!("X-MAS count: {x_mas_count}")
}
