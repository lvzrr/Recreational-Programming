mod keygen;
mod matrixopts;
mod modmatrix;

fn main() {
    let key = keygen::gen_key();
    let key_exp = keygen::expand_key(key);
    let mut matrix = matrixopts::generate_matrix(1234567);
    matrixopts::display_matrix(matrix);
    matrix = modmatrix::gen_martix_variation(matrix, key_exp);
    matrixopts::display_matrix(matrix);
    matrix = modmatrix::undochanges(matrix, key_exp);
    matrixopts::display_matrix(matrix);
    println!("Solve: {}", matrixopts::solvematrix(matrix));
}
