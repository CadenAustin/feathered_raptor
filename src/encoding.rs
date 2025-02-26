use crate::{
    common::{get_number_of_intermediate_symbols, get_systematic_index}, consts::{DEG_TABLE, PRIME_TABLE, V0_TABLE, V1_TABLE}, symbol::Symbol
};

type Triple = (u32, u32, u32);

pub fn rand(x: u32, i: u32, m: u32) -> u32 {
    assert!(m > 0, "RAND: m must be greater than 0");
    (V0_TABLE[((x + i) % 256) as usize] ^ V1_TABLE[(((x / 256) + i) % 256) as usize]) % m
}

pub fn deg(v: u32) -> u32 {
    assert!(v < 1_048_576, "DEG: v must be less than 1048576");
    for &(f, d) in DEG_TABLE.iter().skip(1) {
        if f > v {
            return d;
        }
    }
    unreachable!();
}

pub fn lt_encode(num_source_symbols: u32, intermediate_symbols: &[Symbol], source_triple: Triple) -> Symbol {
    let l = get_number_of_intermediate_symbols(num_source_symbols);
    let mut l_prime = 0;
    for prime in PRIME_TABLE {
        if prime >= l {
            l_prime = prime;
            break;
        }
    }
    if l_prime == 0 {
        unreachable!("LT_ENCODE: precalculated prime table is too small");
    }

    let (d, a, mut b) = source_triple;
    assert!(a >= 1 && a < l_prime, "LT_ENCODE: a must be in the range [1, l_prime - 1]");
    assert!(a < l_prime, "LT_ENCODE: a must be less than l_prime");

    while b >= l {
        b = (b + a) % l_prime;
    }

    let mut result = intermediate_symbols[b as usize].clone();
    for j in 1..(d-1).min(l-1) {
        b = (b + a) % l_prime;
        while b >= l {
            b = (b + a) % l_prime;
        }
        result ^= intermediate_symbols[b as usize].clone();
    }    
    result
}

pub fn trip(num_source_symbols: u32, esi: u32) -> Triple {
    let l = get_number_of_intermediate_symbols(num_source_symbols);
    let mut l_prime = 0;
    for prime in PRIME_TABLE {
        if prime >= l {
            l_prime = prime;
            break;
        }
    }
    if l_prime == 0 {
        unreachable!("LT_ENCODE: precalculated prime table is too small");
    }

    let q: u32 = 65_521;
    let jk = get_systematic_index(num_source_symbols);

    let a_upper = (53591 + jk * 997) % q;
    let b_upper = 10_267 * (jk + 1) % q;
    let y_upper = (b_upper + esi * a_upper) % q;
    let v = rand(y_upper, 0, 1_048_576);
    let d = deg(v);
    let a = 1 + rand(y_upper, 1, l_prime - 1);
    let b = rand(y_upper, 2, l_prime);

    (d, a, b)
}

pub fn partition(i: u32, j: u32) -> (u32, u32, u32, u32) {
    let il = i.div_ceil(j);
    let is = i / j;
    let jl = i - is * j;
    let js = j - jl;

    (il, is, jl, js)
}