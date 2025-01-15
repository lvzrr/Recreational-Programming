mod crypt;
mod keygen;
mod lang;
mod matrixopts;
mod modmatrix;
mod server;

use server::runserver;

fn main() {
    runserver();
}
