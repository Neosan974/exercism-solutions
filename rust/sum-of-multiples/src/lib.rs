fn get_factors_of_less_than(of: u32, limit: u32) -> Vec<u32> {
    let mut result: Vec<u32> = vec![];
    if of == 0 {
        return result;
    }
    let mut i: u32 = 0;
    loop {
        i += 1;
        let factor = of * i;
        if factor < limit {
            result.push(factor);
        } else {
            return result;
        }
    }
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut list_of_factors: Vec<u32> = factors
        .iter()
        .flat_map(|&e| get_factors_of_less_than(e, limit))
        .collect();
    println!("factors before dedup : {:?}", list_of_factors);
    list_of_factors.dedup();
    println!("factors after dedup : {:?}", list_of_factors);
    list_of_factors.iter().sum()
}
