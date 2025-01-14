use std::usize;

pub fn roundkeys(mut matrix: [[u128; 5]; 5], key: [u128; 2500]) -> [[u128; 5]; 5] {
    let mut keyc: usize = 0;
    for i in 0..5 {
        for j in 0..5 {
            matrix[i][j] ^= key[keyc];
            keyc += 1;
        }
    }
    return matrix;
}

pub fn transpose(matrix: [[u128; 5]; 5]) -> [[u128; 5]; 5] {
    let mut newmatrix: [[u128; 5]; 5] = [[0; 5]; 5];
    for i in 0..5 {
        for j in 0..5 {
            newmatrix[i][j] = matrix[j][i];
        }
    }
    return newmatrix;
}

pub fn gen_martix_variation(matrix: [[u128; 5]; 5], key: [u128; 2500]) -> [[u128; 5]; 5] {
    let mut matrix = roundkeys(matrix, key);
    matrix = transpose(matrix);
    return matrix;
}

pub fn undochanges(matrix: [[u128; 5]; 5], key: [u128; 2500]) -> [[u128; 5]; 5] {
    let mut matrix = transpose(matrix);
    matrix = roundkeys(matrix, key);
    return matrix;
}
