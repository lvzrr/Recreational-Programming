use std::usize;

pub fn roundkeys<'a>(matrix: &'a mut [[u128; 5]; 5], key: &[u128; 2500]) -> &'a mut [[u128; 5]; 5] {
    let mut keyc: usize = 0;
    for i in 0..5 {
        for j in 0..5 {
            matrix[i][j] ^= key[keyc];
            keyc += 1;
        }
    }
    matrix
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

pub fn gen_matrix_variation(mut matrix: [[u128; 5]; 5], key: &[u128; 2500]) -> [[u128; 5]; 5] {
    roundkeys(&mut matrix, key);
    matrix = transpose(matrix);
    matrix
}

pub fn undochanges(mut matrix: [[u128; 5]; 5], key: &[u128; 2500]) -> [[u128; 5]; 5] {
    matrix = transpose(matrix);
    roundkeys(&mut matrix, key);
    matrix
}
