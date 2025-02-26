use crate::consts::{MAXIMUM_NUMBER_OF_SOURCE_SYMBOLS, PRIME_TABLE, SYSTEMATIC_INDICES_TABLE};

// Variable J(K)
pub fn get_systematic_index(num_of_symbols: u32) -> u32 {
    assert!(
        num_of_symbols <= MAXIMUM_NUMBER_OF_SOURCE_SYMBOLS,
        "GetSystematicIndex: num_of_symbols exceeds maximum value"
    );

    SYSTEMATIC_INDICES_TABLE[num_of_symbols as usize]
}

// Variable X: smallest positive integer such that X*(X-1) >= 2*K.
// OR X^2 - X - 2*K >= 0
pub fn get_var_x(num_of_symbols: u32) -> u32 {
    assert!(
        num_of_symbols <= MAXIMUM_NUMBER_OF_SOURCE_SYMBOLS,
        "GetVarX: num_of_symbols exceeds maximum value"
    );

    let rhs = 2 * num_of_symbols;
    for x in 1..num_of_symbols {
        if x * (x - 1) >= rhs {
            return x;
        }
    } 
    unreachable!();
}

// Variable S: smallest prime integer such that S >= ceil(0.01*K) + X
pub fn get_number_of_ldpc_symbols(num_of_symbols: u32) -> u32 {
    let x = get_var_x(num_of_symbols);
    let rhs = (0.01 * num_of_symbols as f64).ceil() as u32 + x;

    for prime in PRIME_TABLE {
        if prime >= rhs {
            return prime;
        }
    }
    unreachable!("GetNumberOfLdpcSymbols: precalculated prime table is too small");
}

// Choose function
pub fn choose(n: u32, k: u32) -> u32 {
    if k > n {
        return 0;
    }
    let mut result = 1;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

// Variable H: smallest integer such that choose(H,ceil(H/2)) >= K + S
pub fn get_number_of_half_symbols(num_of_symbols: u32) -> u32 {
    let s = get_number_of_ldpc_symbols(num_of_symbols);
    let rhs = num_of_symbols + s;

    for h in 1.. {
        if choose(h, h.div_ceil(2)) >= rhs {
            return h;
        }
    }
    unreachable!();
}

// Variable L: Number of Intermediate Symbols
pub fn get_number_of_intermediate_symbols(num_of_symbols: u32) -> u32 {
    get_systematic_index(num_of_symbols)
        + get_number_of_ldpc_symbols(num_of_symbols)
        + get_number_of_half_symbols(num_of_symbols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_choose() {
        assert_eq!(choose(10, 5), 252);
        assert_eq!(choose(5, 2), 10);
        assert_eq!(choose(4, 2), 6);
        assert_eq!(choose(4, 2), 6);
        assert_eq!(choose(6, 5), 6);
        assert_eq!(choose(5, 5), 1);
    }
}
