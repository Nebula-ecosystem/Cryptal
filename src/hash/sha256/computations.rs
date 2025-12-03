pub use super::K256;

#[inline(always)]
pub fn small_sigma0(x: u32) -> u32 {
    x.rotate_right(7) ^ x.rotate_right(18) ^ (x >> 3)
}

#[inline(always)]
pub fn small_sigma1(x: u32) -> u32 {
    x.rotate_right(17) ^ x.rotate_right(19) ^ (x >> 10)
}

#[inline(always)]
pub fn big_sigma0(x: u32) -> u32 {
    x.rotate_right(2) ^ x.rotate_right(13) ^ x.rotate_right(22)
}

#[inline(always)]
pub fn big_sigma1(x: u32) -> u32 {
    x.rotate_right(6) ^ x.rotate_right(11) ^ x.rotate_right(25)
}

#[inline(always)]
pub fn ch(e: u32, f: u32, g: u32) -> u32 {
    (e & f) ^ ((!e) & g)
}

#[inline(always)]
pub fn maj(a: u32, b: u32, c: u32) -> u32 {
    (a & b) ^ (a & c) ^ (b & c)
}

pub fn expand_w(w: &mut [u32; 64]) {
    let wp = w.as_mut_ptr();

    unsafe {
        for i in 16..64 {
            let val = (*wp.add(i - 16))
                .wrapping_add(small_sigma0(*wp.add(i - 15)))
                .wrapping_add(*wp.add(i - 7))
                .wrapping_add(small_sigma1(*wp.add(i - 2)));
            *wp.add(i) = val;
        }
    }
}

pub fn all_rounds(state: [u32; 8], w: &[u32; 64]) -> [u32; 8] {
    let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;

    let wp = w.as_ptr();
    let kp = K256.as_ptr();
    for i in 0..64 {
        let wi = unsafe { wp.add(i).read() };
        let ki = unsafe { kp.add(i).read() };

        let t1 = h
            .wrapping_add(big_sigma1(e))
            .wrapping_add(ch(e, f, g))
            .wrapping_add(ki)
            .wrapping_add(wi);
        let t2 = big_sigma0(a).wrapping_add(maj(a, b, c));

        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(t1);
        d = c;
        c = b;
        b = a;
        a = t1.wrapping_add(t2);
    }

    [a, b, c, d, e, f, g, h]
}
