pub fn get_chunk_key((x, y, z): (i64, i64, i64)) -> i64 {
    let mask = (1 << 21) - 1;

    ((x & mask) << 42) |
    ((y & mask) << 21) |
    (z & mask)
}
