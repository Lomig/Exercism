/// Fin the saddle points of a given matrix
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.iter().flatten().collect::<Vec<&u64>>() == vec![] as Vec<&u64> {
        return vec![];
    }

    let mins_in_columns = input[0]
        .clone()
        .into_iter()
        .enumerate()
        .fold(vec![], |result, (index, _)| {
            [result, vec![min_in_column(input, index)]].concat()
        });

    input.iter().enumerate().fold(vec![], |result, (x, row)| {
        let max_in_row = row.iter().max().unwrap();

        let row_results = row
            .into_iter()
            .enumerate()
            .fold(vec![], |row_result, (y, cell)| {
                if cell >= max_in_row && cell <= &mins_in_columns[y] {
                    return [row_result, vec![(x, y)]].concat();
                }
                row_result
            });

        [result, row_results].concat()
    })
}

/// Utility function: find the smallest element in a given column
fn min_in_column(matrix: &[Vec<u64>], column: usize) -> u64 {
    matrix.into_iter().fold(matrix[0][column], |minimum, row| {
        *[minimum, row[column]].iter().min().unwrap()
    })
}
