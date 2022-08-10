use std::{env, error::Error};

fn usage(prog_name: &str) {
    println!("Usage: ./{prog_name} <width> <height> <scale> <rule> [output]");
}

fn main() -> Result<(), Box<dyn Error>> {
    let argv = env::args()
        .take(6)
        .collect::<Vec<_>>();

    if argv.len() < 5 {
        usage(&argv[0]);
        return Err("Missing arguments!".into());
    }

    let width = argv[1]
        .parse::<usize>()?;
    
    let height = argv[2]
        .parse::<usize>()?;

    let scale = argv[3]
        .parse::<usize>()?;

    let rule = argv[4]
        .parse::<u8>()?;

    // optional
    let filename = argv.get(5)
        .unwrap_or(&format!("{rule}.png"))
        .to_string();

    let gen_size = height / scale;
    let grid_size = width / scale;

    let mut grid = vec![0u8; grid_size];
    let mut old_grid = vec![0u8; grid_size];

    // may change it later
    grid[grid_size / 2] = 1;
    old_grid[grid_size / 2] = 1;

    let mut output = image::RgbImage::new(width as _, height as _);

    for gen in 0..gen_size {
        for cell in 0..grid_size {
            // this awfully unreadable code makes the cells wrap around on boundaries
            let prev = old_grid[(cell as isize - 1).rem_euclid(grid_size as isize) as usize];
            let curr = old_grid[cell];
            let next = old_grid[(cell+1).rem_euclid(grid_size)];

            let code = 4 * prev + 2 * curr + next;
            let idx = 1 << code;

            grid[cell] = (rule & idx) >> code;

            let color = if grid[cell] == 1 { image::Rgb([255, 255, 255]) }
                else { image::Rgb([0, 0, 0]) };

            for x in (cell*scale)..(cell+1)*scale {
                for y in (gen*scale)..(gen+1)*scale {
                    output[(x as _, y as _)] = color;
                }
            }
        }

        old_grid.clone_from(&grid);
    }

    output.save(&filename)?;

    println!("Output successfully written at {filename}");

    Ok(())
}
