fn get_random_u32_number(random_state: u32) -> u32 {
    let mut number = random_state;

    // xorshift32
    number ^= number << 13;
    number ^= number >> 15;
    number ^= number << 5;

    number
}

fn get_random_u64_number(random_state: u32) -> (u64, u32) {
    let r1 = get_random_u32_number(random_state);
    let r2 = get_random_u32_number(r1);
    let r3 = get_random_u32_number(r2);
    let r4 = get_random_u32_number(r3);

    let n1 = r1 as u64 & 0xFFFFu64;
    let n2 = r2 as u64 & 0xFFFFu64;
    let n3 = r3 as u64 & 0xFFFFu64;
    let n4 = r4 as u64 & 0xFFFFu64;

    let number = n1 | (n2 << 16) | (n3 << 32) | (n4 << 48);

    (number, r4)
}

pub fn next_magic_candidate(random_state: u32) -> (u64, u32) {
    let (n1, r1) = get_random_u64_number(random_state);
    let (n2, r2) = get_random_u64_number(r1);
    let (n3, r3) = get_random_u64_number(r2);

    (n1 & n2 & n3, r3)
}
