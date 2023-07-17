pub fn rle_decode(encoded_string: &str) -> Vec<f32> {
    let mut decoded_vector = Vec::new();

    for segment in encoded_string.split(';').filter(|s| !s.is_empty()) {
        let mut iter = segment.split(':');
        let value: f32 = iter.next().unwrap().parse().unwrap();
        let count: usize = iter.next().unwrap().parse().unwrap();
        decoded_vector.extend(std::iter::repeat(value).take(count));
    }

    decoded_vector
}

pub fn rle_encode(vector: &[f32]) -> String {
    let mut encoded_string = String::new();
    let mut count = 0;
    let mut prev_value: Option<f32> = None;

    for &value in vector {
        match prev_value {
            Some(prev) if prev == value => count += 1,
            Some(prev) => {
                encoded_string.push_str(&format!("{}:{};", prev, count));
                count = 1;
            }
            None => count = 1,
        }

        prev_value = Some(value);
    }

    if let Some(value) = prev_value {
        encoded_string.push_str(&format!("{}:{};", value, count));
    }

    encoded_string
}

pub fn zigzag_traversal(matrix: Vec<Vec<f32>>) -> Vec<f32> {
    let rows = matrix.len();
    if rows == 0 {
        return Vec::new();
    }

    let cols = matrix[0].len();
    let mut result = Vec::new();
    let mut row = 0;
    let mut col = 0;
    let mut going_up = true;

    while row < rows && col < cols {
        result.push(matrix[row][col]);

        if going_up {
            if row > 0 && col < cols - 1 {
                row -= 1;
                col += 1;
            } else if col == cols - 1 {
                row += 1;
                going_up = false;
            } else {
                col += 1;
                going_up = false;
            }
        } else {
            if col > 0 && row < rows - 1 {
                col -= 1;
                row += 1;
            } else if row == rows - 1 {
                col += 1;
                going_up = true;
            } else {
                row += 1;
                going_up = true;
            }
        }
    }

    result
}

pub fn inverse_zigzag_traversal(path: Vec<f32>, rows: usize, cols: usize) -> Vec<Vec<f32>> {
    let mut matrix = vec![vec![0.0; cols]; rows];
    let mut row = 0;
    let mut col = 0;
    let mut going_up = true;
    let mut index = 0;

    while index < path.len() {
        matrix[row][col] = path[index];

        if going_up {
            if row > 0 && col < cols - 1 {
                row -= 1;
                col += 1;
            } else if col == cols - 1 {
                row += 1;
                going_up = false;
            } else {
                col += 1;
                going_up = false;
            }
        } else {
            if col > 0 && row < rows - 1 {
                col -= 1;
                row += 1;
            } else if row == rows - 1 {
                col += 1;
                going_up = true;
            } else {
                row += 1;
                going_up = true;
            }
        }

        index += 1;
    }

    matrix
}

pub fn save_data(value: Vec<Vec<f32>>) -> String {
    let encoded = rle_encode(&zigzag_traversal(value.clone()));

    encoded
}

pub fn load_data(value: String) -> Vec<Vec<f32>> {
    let decoded = rle_decode(value.as_str());
    let res_f32 = inverse_zigzag_traversal(decoded, 8, 8);

    let mut res: Vec<Vec<f32>> = vec![vec![0.0; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            res[i][j] = res_f32[i][j] as f32;
        }
    }

    res
}

