use std::iter::repeat;
use cached::proc_macro::cached;

#[cached]
fn pascal(x: u32, y: u32) -> u32 {
    if x == 0 || y == 0 {
        1
    } else {
        pascal(x - 1, y) + pascal(x, y - 1)
    }
}

fn coords_from_row_col(row: u32, col: u32) -> (u32, u32) {
    // row = x + y
    // col = y
    // y = col
    // x = row - y
    (row - col, col)
}

fn main() {
    let size: u16 = 15;
    for row in 0..size {
        let nums = (0..=row).map(|col| {
            let (x, y) = coords_from_row_col(row as u32, col as u32);
            pascal(x, y)
        });

        let padding: usize = (size - row - 1).into();

        println!(
            "{}{}",
            repeat("   ").take(padding).collect::<String>(),
            nums
                .map(|n| format!("{:>5}", n))
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}

