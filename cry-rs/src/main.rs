mod keygen;
mod matrixopts;
mod modmatrix;

fn main() {
    let s = keygen::gen_solution();
    let key = keygen::expand_key(keygen::gen_key());
    let mut matrix = matrixopts::generate_matrix(&s);
    matrixopts::display_matrix(&matrix);
    matrix = modmatrix::gen_matrix_variation(matrix, &key);
    matrixopts::display_matrix(&matrix);
    matrix = modmatrix::undochanges(matrix, &key);
    matrixopts::display_matrix(&matrix);
    println!("SOL: {} | {}", s, matrixopts::solvematrix(&matrix));
}
