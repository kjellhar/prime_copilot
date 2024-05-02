use std::time::Instant;

fn main() {
    const MAX_LIMIT: usize = (u32::MAX as usize)-1;
    //const MAX_LIMIT: usize = 100;
 

    let start = Instant::now();
    let primes = sieve_of_eratosthenes(MAX_LIMIT);
    let duration = start.elapsed();

    println!("Number of found primes: {}", primes.len());
    println!(
        "Time elapsed: {} ms, or {} min",
        duration.as_millis(),
        duration.as_secs() / 60
    );

}


fn sieve_of_eratosthenes(limit: usize) -> Vec<u32> {
    let mut primes: Vec<bool> = vec!(true; (limit+1)/2);

    primes[0] = false;
    let mut p = 3;
    while p * p < limit {
        if primes[p>>1 as usize] {
            let mut i = p * p;
            while i < limit {
                primes[i>>1 as usize] = false;
                i += p+p;
            }
        }
        p += 2;
    }

    //primes.iter().fold(0,|acc, p| if *p {acc +1} else {acc})
    primes.iter().enumerate()
        .filter_map(|(i, &prime)| if prime { Some((i*2 + 1) as u32) } else { None })
        .collect()

}