use std::fs;
use std::error::Error;
use ndarray::*;

/// Reads a file and returns a matrix corresponding to the contents of the file.
/// The contents file should be formatted as numbers separated by a space to
/// indicate a column and by a newline to indicate a row.
/// Get an error if the file is badly formatted or does not represent an
/// actual matrix.
pub fn read_matrix(filename: &str) -> Result<Array2<f64>, &'static str> {
    let contents = fs::read_to_string(filename);
    if let Err(e) = contents {
        return Err("could not read the file");
    }
    let contents = contents.unwrap();
    let contents = contents.trim();
    fill_matrix(&contents)
}

/// Attempts to parse a string as a matrix.
fn fill_matrix(contents: &str) -> Result<Array2<f64>, &'static str> {
    let mut rows = contents.split('\n');

    // First row is used to determine column length
    let row = rows.next();
    if let None = row {
        return Err("file was empty");
    }
    let row = parse_row(row.unwrap())?;

    let num_columns = row.len();
    let mut num_rows = 1;
    let mut matrix = Array::zeros((0, num_columns));
    matrix.push_row(ArrayView::from(&row));
    for row in rows {
        num_rows += 1;
        let row = parse_row(row)?;
        if row.len() != num_columns {
            return Err("cannot create array with variable length columns");
        }
        matrix.push_row(ArrayView::from(&row));
    }
    
    Ok(matrix)
}

/// Attempts to convert a line of tokens into a vector of numbers.
fn parse_row(row: &str) -> Result<Vec<f64>, &'static str> {
    let tokens: Vec<&str> = row.split(',').collect();
    let mut result = Vec::with_capacity(tokens.len());
    for token in tokens {
        let token = token.parse::<f64>();
        match token {
            Err(_) => return Err("could not parse token into a number"),
            Ok(x) => result.push(x),
        }
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fills_matrix_correctly() {
        let actual_matrix = "1,0,0\n-2,1,0\n0,0,1";
        let actual_matrix = fill_matrix(&actual_matrix)
            .expect("error reading from file.");
        let expected_matrix = array![
            [1., 0., 0.],
            [-2., 1., 0.],
            [0., 0., 1.]];
        assert_eq!(expected_matrix, actual_matrix);
    }

    #[test]
    fn returns_err_if_file_not_found() {
        // Just make sure that there's no file called 'soklmfrgi.txt'
        assert!(matches!(read_matrix("soklmfrgi.txt"), Err(_)));
    }
}
