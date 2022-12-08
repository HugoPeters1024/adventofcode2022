use std::io::BufRead;

fn main() {
    let grid : Vec<Vec<u32>> = 
        std::io::stdin().lock()
            .lines()
            .map(|x| x.unwrap().chars().map(|y| y.to_digit(10).unwrap()).collect())
            .collect();

    let grid_width = grid[0].len();
    let grid_height = grid.len();

    let mut total = 0;
    let mut best_scenic = 0;
    for i in 1..grid_height-1 {
        for j in 1..grid_width-1 {
            let mut scenic = 1;

            let tree = grid[i][j];
            let mut vis = false;

            let mut flag = true;
            let mut lscenic = 0;
            for ii in (0..i).rev() {
                lscenic += 1;
                if grid[ii][j] >= tree {
                    flag = false;
                    break;
                }
            }
            vis |= flag;
            scenic *= lscenic;



            flag = true;
            lscenic = 0;
            for ii in i+1..grid_height {
                lscenic += 1;
                if grid[ii][j] >= tree {
                    flag = false;
                    break;
                }
            }
            vis |= flag;
            scenic *= lscenic;

            flag = true;
            lscenic = 0;
            for jj in (0..j).rev() {
                lscenic += 1;
                if grid[i][jj] >= tree {
                    flag = false;
                    break;
                }
            }
            vis |= flag;
            scenic *= lscenic;

            flag = true;
            lscenic = 0;
            for jj in j+1..grid_width {
                lscenic += 1;
                if grid[i][jj] >= tree {
                    flag = false;
                    break;
                }
            }
            vis |= flag;
            scenic *= lscenic;

            if vis {
                total += 1;
            }

            if scenic > best_scenic {
                best_scenic = scenic;
            }
        }
    }
    total += 2 * (grid_width + grid_height - 2);

    dbg!(total);
    dbg!(best_scenic);
}
