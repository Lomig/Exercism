/// Fin the saddle points of a given matrix
pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    if input.iter().flatten().collect::<Vec<&u64>>() == vec![] as Vec<&u64> {
        return vec![];
    }

    input.iter().enumerate().fold(vec![], |result, (x, row)| {
        let row_results = row
            .into_iter()
            .enumerate()
            .fold(vec![], |row_result, (y, cell)| {
                if row.into_iter().all(|value| { value <= cell }) &&
                    input.into_iter().all(|line| { line[y] >= *cell }) {
                    return [row_result, vec![(x, y)]].concat();
                }
                row_result
            });

        [result, row_results].concat()
    })
}
