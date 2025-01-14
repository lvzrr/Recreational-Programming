use std::{u128, usize};

fn sieve(n: u128) -> Vec<u128> {
    let mut primes: Vec<u128> = vec![1; (n + 1) as usize];
    let mut out: Vec<u128> = Vec::new();
    primes[0] = 0;
    primes[1] = 0;
    for i in 2..=n {
        if primes[i as usize] == 1 {
            out.push(i);
            for j in (i * i..=n).step_by(i as usize) {
                primes[j as usize] = 0;
            }
        }
    }

    return out;
}

fn checkprime(num: u128) -> bool {
    let sieve2: Vec<u128> = sieve(num);
    if sieve2.contains(&num) {
        return true;
    }
    return false;
}

pub fn solvematrix(matrix: [[u128; 5]; 5]) -> u128 {
    let mut sum: u128 = 0;
    for i in 0..5 {
        for j in 0..4 {
            if !checkprime(matrix[i][j]) {
                return 0;
            }
        }
    }
    for i in 0..5 {
        for j in 0..5 {
            sum += matrix[i][j];
        }
    }
    return sum;
}

pub fn generate_matrix(num: u128) -> [[u128; 5]; 5] {
    let primes: Vec<u128> = sieve(num);

    let mut prevprime: u128 = primes[primes.len() / 2];

    let mut firstiter: [u128; 5] = [0; 5];
    firstiter[0] = prevprime;

    for i in 1..4 {
        if !checkprime(prevprime) {
            println!("{} NOT PRIME", prevprime);
        }

        let sieve2: Vec<u128> = sieve(prevprime);
        firstiter[i] = sieve2[sieve2.len() / 2];
        prevprime = firstiter[i];
    }

    firstiter[4] = num - (firstiter[0] + firstiter[1] + firstiter[2] + firstiter[3] + firstiter[4]);

    let mut finaliter: [[u128; 5]; 5] = [[0; 5]; 5];

    for i in 0..5 {
        let sieve2: Vec<u128> = sieve(firstiter[i]);
        prevprime = sieve2[sieve2.len() / 2];
        finaliter[i][0] = prevprime;
        for j in 1..4 {
            let sieve2: Vec<u128> = sieve(prevprime);
            finaliter[i][j] = sieve2[sieve2.len() / 2];
            prevprime = finaliter[i][j];
        }
        finaliter[i][4] =
            firstiter[i] - (finaliter[i][0] + finaliter[i][1] + finaliter[i][2] + finaliter[i][3]);
    }

    return finaliter;
}

pub fn display_matrix(matrix: [[u128; 5]; 5]) {
    for i in 0..5 {
        for j in 0..5 {
            print!("{} ", matrix[i][j]);
        }
        println!("\n");
    }
}
