pub(crate) fn equal(b: i8, c: i8) -> u8 {
    let ub = b as u8;
    let uc = c as u8;
    let x = ub ^ uc;
    let mut y = x as u64;

    y = y.wrapping_sub(1);
    y >>= 63;
    y as u8
}

pub(crate) fn negative(b: i8) -> u8 {
    let mut x = b as i64 as u64;
    x >>= 63;

    x as u8
}
