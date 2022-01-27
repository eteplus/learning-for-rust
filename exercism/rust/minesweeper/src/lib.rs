fn get_low_range_limit(num: usize) -> usize {
    match num {
        0 => 0,
        _ => num -1
    }
}
fn compute_char(minefield: &[&str], row: usize, col: usize) -> char {
    const STAR_VAL: u8 = '*' as u8;
    let input_row = minefield[row].as_bytes();
    if input_row[col] == STAR_VAL {
        return STAR_VAL as char;
    }
    // We need to add +2 to get higher range limit as it is exclusive
    let row_range = get_low_range_limit(row)..std::cmp::min(minefield.len(), row+2);
    let col_range = get_low_range_limit(col)..std::cmp::min(input_row.len(), col+2);
    let mut count: u8 = 0;
    for r in row_range {
        let input_row = minefield[r].as_bytes();
        for c in col_range.clone() {
            if input_row[c] == STAR_VAL {
                count += 1;
            }
        }
    }
    match count {
        0 => ' ',
        _ => ('0' as u8 + count) as char
    }
}
pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result = Vec::new();
    for row in 0..minefield.len() {
        let mut result_row = String::new();
        for col in 0..minefield[row].as_bytes().len() {
            result_row.push(compute_char(minefield, row, col));
        }
        result.push(result_row)
    }
    result
}