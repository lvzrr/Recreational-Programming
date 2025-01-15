mod crypt;
mod data;
mod keygen;
mod lang;
mod matrixopts;
mod modmatrix;
mod server;

use server::runserver;

fn main() {
    runserver();
}
