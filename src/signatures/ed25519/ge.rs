use super::field::FieldElement;
use super::precomp_data::{BASE, BI};

pub struct GeP2 {
    pub(crate) x: FieldElement,
    pub(crate) y: FieldElement,
    pub(crate) z: FieldElement,
}

pub struct GeP3 {
    pub(crate) x: FieldElement,
    pub(crate) y: FieldElement,
    pub(crate) z: FieldElement,
    pub(crate) t: FieldElement,
}

pub struct GeP1P1 {
    pub(crate) x: FieldElement,
    pub(crate) y: FieldElement,
    pub(crate) z: FieldElement,
    pub(crate) t: FieldElement,
}

#[derive(Clone, Copy)]
pub struct GeCached {
    pub(crate) yplusx: FieldElement,
    pub(crate) yminusx: FieldElement,
    pub(crate) z: FieldElement,
    pub(crate) t2d: FieldElement,
}

pub struct GePrecomp {
    pub(crate) yplusx: FieldElement,
    pub(crate) yminusx: FieldElement,
    pub(crate) xy2d: FieldElement,
}

impl Default for GeP2 {
    fn default() -> Self {
        Self {
            x: FieldElement::ZERO,
            y: FieldElement::ZERO,
            z: FieldElement::ZERO,
        }
    }
}

impl Default for GeP3 {
    fn default() -> Self {
        Self {
            x: FieldElement::ZERO,
            y: FieldElement::ZERO,
            z: FieldElement::ZERO,
            t: FieldElement::ZERO,
        }
    }
}

impl Default for GeP1P1 {
    fn default() -> Self {
        Self {
            x: FieldElement::ZERO,
            y: FieldElement::ZERO,
            z: FieldElement::ZERO,
            t: FieldElement::ZERO,
        }
    }
}

impl Default for GeCached {
    fn default() -> Self {
        Self {
            yplusx: FieldElement::ZERO,
            yminusx: FieldElement::ZERO,
            z: FieldElement::ZERO,
            t2d: FieldElement::ZERO,
        }
    }
}

impl Default for GePrecomp {
    fn default() -> Self {
        Self {
            yplusx: FieldElement::ZERO,
            yminusx: FieldElement::ZERO,
            xy2d: FieldElement::ZERO,
        }
    }
}

pub fn ge_add(r: &mut GeP1P1, p: &GeP3, q: &GeCached) {
    r.x = p.y + p.x;
    r.y = p.y - p.x;

    r.z = r.x * q.yplusx;
    r.y = r.y * q.yminusx;

    r.t = q.t2d * p.t;
    r.x = p.z * q.z;

    let t0 = r.x + r.x;

    r.x = r.z - r.y;
    r.y = r.z + r.y;

    r.z = t0 + r.t;
    r.t = t0 - r.t;
}

pub fn slide(r: &mut [i8], a: &[u8; 32]) {
    assert_eq!(r.len(), 256);

    for i in 0..256 {
        r[i] = ((a[i >> 3] >> (i & 7)) & 1) as i8;
    }

    for i in 0..256 {
        if r[i] != 0 {
            let mut b = 1usize;
            while b <= 6 && i + b < 256 {
                if r[i + b] != 0 {
                    let rb = (r[i + b] as i32) << b;
                    let ri = r[i] as i32;

                    if ri + rb <= 15 {
                        r[i] = (ri + rb) as i8;
                        r[i + b] = 0;
                    } else if ri - rb >= -15 {
                        r[i] = (ri - rb) as i8;

                        let mut k = i + b;
                        while k < 256 {
                            if r[k] == 0 {
                                r[k] = 1;
                                break;
                            }
                            r[k] = 0;
                            k += 1;
                        }
                    } else {
                        break;
                    }
                }
                b += 1;
            }
        }
    }
}

pub fn ge_double_scalarmult_vartime(r: &mut GeP2, a: &[u8; 32], a_point: &GeP3, b: &[u8; 32]) {
    let mut aslide: [i8; 256] = [0; 256];
    let mut bslide: [i8; 256] = [0; 256];

    let mut ai: [GeCached; 8] = [GeCached {
        yplusx: FieldElement::ZERO,
        yminusx: FieldElement::ZERO,
        z: FieldElement::ZERO,
        t2d: FieldElement::ZERO,
    }; 8];

    let mut t = GeP1P1 {
        x: FieldElement::ZERO,
        y: FieldElement::ZERO,
        z: FieldElement::ZERO,
        t: FieldElement::ZERO,
    };

    let mut u = GeP3 {
        x: FieldElement::ZERO,
        y: FieldElement::ZERO,
        z: FieldElement::ZERO,
        t: FieldElement::ZERO,
    };

    let mut a2 = GeP3 {
        x: FieldElement::ZERO,
        y: FieldElement::ZERO,
        z: FieldElement::ZERO,
        t: FieldElement::ZERO,
    };

    slide(&mut aslide, a);
    slide(&mut bslide, b);

    ge_p3_to_cached(&mut ai[0], a_point);

    ge_p3_dbl(&mut t, a_point);
    ge_p1p1_to_p3(&mut a2, &t);

    // Ai[1..7]
    for j in 1..8 {
        ge_add(&mut t, &a2, &ai[j - 1]);
        ge_p1p1_to_p3(&mut u, &t);
        ge_p3_to_cached(&mut ai[j], &u);
    }

    ge_p2_0(r);

    let mut i: i32 = 255;
    while i >= 0 {
        if aslide[i as usize] != 0 || bslide[i as usize] != 0 {
            break;
        }
        i -= 1;
    }

    while i >= 0 {
        let asi = aslide[i as usize];
        let bsi = bslide[i as usize];

        ge_p2_dbl(&mut t, r);

        if asi > 0 {
            ge_p1p1_to_p3(&mut u, &t);
            let idx = (asi / 2) as usize;
            ge_add(&mut t, &u, &ai[idx]);
        } else if asi < 0 {
            ge_p1p1_to_p3(&mut u, &t);
            let idx = ((-asi) / 2) as usize;
            ge_sub(&mut t, &u, &ai[idx]);
        }

        if bsi > 0 {
            ge_p1p1_to_p3(&mut u, &t);
            let idx = (bsi / 2) as usize;
            ge_madd(&mut t, &u, &BI[idx]);
        } else if bsi < 0 {
            ge_p1p1_to_p3(&mut u, &t);
            let idx = ((-bsi) / 2) as usize;
            ge_msub(&mut t, &u, &BI[idx]);
        }

        ge_p1p1_to_p2(r, &t);
        i -= 1;
    }
}

pub(crate) const D: FieldElement = FieldElement([
    -10913610, 13857413, -15372611, 6949391, 114729, -8787816, -6275908, -3247719, -18696448,
    -12055116,
]);

pub(crate) const SQRTM1: FieldElement = FieldElement([
    -32595792, -7943725, 9377950, 3500415, 12389472, -272473, -25146209, -2005654, 326686, 11406482,
]);

pub fn ge_frombytes_negate_vartime(h: &mut GeP3, s: &[u8; 32]) -> i32 {
    h.y = FieldElement::from_bytes(s);
    h.z = FieldElement::ONE;

    let mut u = h.y.sq();
    let mut v = u * D;
    u = u - h.z;
    v = v + h.z;

    let v3 = v.sq();
    let v3 = v3 * v;

    h.x = v3.sq();
    h.x = h.x * v;
    h.x = h.x * u;
    h.x = h.x.pow22523();
    h.x = h.x * v3;
    h.x = h.x * u;

    let mut vxx = h.x.sq();
    vxx = vxx * v;
    let mut check = vxx - u;

    if check.is_non_zero() == 1 {
        check = vxx + u;
        if check.is_non_zero() == 1 {
            return -1;
        }
        h.x = h.x * SQRTM1;
    }

    let sign = (s[31] >> 7) as i32;
    if h.x.is_negative() == sign {
        h.x = -h.x;
    }

    h.t = h.x * h.y;
    0
}

pub fn ge_madd(r: &mut GeP1P1, p: &GeP3, q: &GePrecomp) {
    r.x = p.y + p.x;
    r.y = p.y - p.x;

    r.z = r.x * q.yplusx;
    r.y = r.y * q.yminusx;

    r.t = q.xy2d * p.t;

    let t0 = p.z + p.z;

    r.x = r.z - r.y;
    r.y = r.z + r.y;

    r.z = t0 + r.t;
    r.t = t0 - r.t;
}

pub fn ge_msub(r: &mut GeP1P1, p: &GeP3, q: &GePrecomp) {
    r.x = p.y + p.x;
    r.y = p.y - p.x;

    r.z = r.x * q.yminusx;
    r.y = r.y * q.yplusx;

    r.t = q.xy2d * p.t;

    let t0 = p.z + p.z;

    r.x = r.z - r.y;
    r.y = r.z + r.y;

    r.z = t0 - r.t;
    r.t = t0 + r.t;
}

pub fn ge_p1p1_to_p2(r: &mut GeP2, p: &GeP1P1) {
    r.x = p.x * p.t;
    r.y = p.y * p.z;
    r.z = p.z * p.t;
}

pub fn ge_p1p1_to_p3(r: &mut GeP3, p: &GeP1P1) {
    r.x = p.x * p.t;
    r.y = p.y * p.z;
    r.z = p.z * p.t;
    r.t = p.x * p.y;
}

pub fn ge_p2_0(h: &mut GeP2) {
    h.x = FieldElement::ZERO;
    h.y = FieldElement::ONE;
    h.z = FieldElement::ONE;
}

pub fn ge_p2_dbl(r: &mut GeP1P1, p: &GeP2) {
    r.x = p.x.sq();
    r.z = p.y.sq();
    r.t = p.z.sq2();

    r.y = p.x + p.y;
    let t0 = r.y.sq();

    r.y = r.z + r.x;
    r.z = r.z - r.x;

    r.x = t0 - r.y;
    r.t = r.t - r.z;
}

pub fn ge_p3_0(h: &mut GeP3) {
    h.x = FieldElement::ZERO;
    h.y = FieldElement::ONE;
    h.z = FieldElement::ONE;
    h.t = FieldElement::ZERO;
}

pub fn ge_p3_dbl(r: &mut GeP1P1, p: &GeP3) {
    let mut q = GeP2 {
        x: FieldElement::ZERO,
        y: FieldElement::ZERO,
        z: FieldElement::ZERO,
    };

    ge_p3_to_p2(&mut q, p);
    ge_p2_dbl(r, &q);
}

pub(crate) const D2: FieldElement = FieldElement([
    -21827239, -5839606, -30745221, 13898782, 229458, 15978800, -12551817, -6495438, 29715968,
    9444199,
]);

pub fn ge_p3_to_cached(r: &mut GeCached, p: &GeP3) {
    r.yplusx = p.y + p.x;
    r.yminusx = p.y - p.x;
    r.z = p.z;
    r.t2d = p.t * D2;
}

pub fn ge_p3_to_p2(r: &mut GeP2, p: &GeP3) {
    r.x = p.x;
    r.y = p.y;
    r.z = p.z;
}

pub fn ge_p3_tobytes(output: &mut [u8; 32], h: &GeP3) {
    let recip = h.z.invert();
    let x = h.x * recip;
    let y = h.y * recip;

    *output = y.to_bytes();

    let sign_bit = x.is_negative() as u8;
    output[31] ^= sign_bit << 7;
}

pub fn equal(b: i8, c: i8) -> u8 {
    let ub = b as u8;
    let uc = c as u8;
    let x = ub ^ uc;
    let mut y = x as u64;
    y = y.wrapping_sub(1);
    y >>= 63;
    y as u8
}

pub fn negative(b: i8) -> u8 {
    let mut x = b as i64 as u64;
    x >>= 63;
    x as u8
}

pub fn cmov(t: &mut GePrecomp, u: &GePrecomp, b: u8) {
    t.yplusx.mov(&u.yplusx, b as u32);
    t.yminusx.mov(&u.yminusx, b as u32);
    t.xy2d.mov(&u.xy2d, b as u32);
}

pub fn select(t: &mut GePrecomp, pos: usize, b: i8) {
    let mut minust = GePrecomp {
        yplusx: FieldElement::ZERO,
        yminusx: FieldElement::ZERO,
        xy2d: FieldElement::ZERO,
    };

    let bnegative = negative(b);
    let babs = (b as i16 - (((-(bnegative as i16)) & (b as i16)) << 1)) as i8;

    t.yplusx = FieldElement::ONE;
    t.yminusx = FieldElement::ONE;
    t.xy2d = FieldElement::ZERO;

    cmov(t, &BASE[pos][0], equal(babs, 1));
    cmov(t, &BASE[pos][1], equal(babs, 2));
    cmov(t, &BASE[pos][2], equal(babs, 3));
    cmov(t, &BASE[pos][3], equal(babs, 4));
    cmov(t, &BASE[pos][4], equal(babs, 5));
    cmov(t, &BASE[pos][5], equal(babs, 6));
    cmov(t, &BASE[pos][6], equal(babs, 7));
    cmov(t, &BASE[pos][7], equal(babs, 8));

    minust.yplusx = t.yminusx;
    minust.yminusx = t.yplusx;
    minust.xy2d = -t.xy2d;

    cmov(t, &minust, bnegative);
}

pub fn ge_scalarmult_base(h: &mut GeP3, a: &[u8; 32]) {
    let mut e = [0i8; 64];

    let mut r = GeP1P1 {
        x: FieldElement::ZERO,
        y: FieldElement::ZERO,
        z: FieldElement::ZERO,
        t: FieldElement::ZERO,
    };

    let mut s = GeP2 {
        x: FieldElement::ZERO,
        y: FieldElement::ZERO,
        z: FieldElement::ZERO,
    };

    let mut t = GePrecomp {
        yplusx: FieldElement::ZERO,
        yminusx: FieldElement::ZERO,
        xy2d: FieldElement::ZERO,
    };

    for i in 0..32 {
        e[2 * i] = (a[i] & 15) as i8;
        e[2 * i + 1] = ((a[i] >> 4) & 15) as i8;
    }

    let mut carry: i8 = 0;
    for v in e.iter_mut().take(63) {
        *v += carry;
        carry = (*v + 8) >> 4;
        *v -= carry << 4;
    }
    e[63] += carry;

    ge_p3_0(h);

    for i in (1..64).step_by(2) {
        select(&mut t, i / 2, e[i]);
        ge_madd(&mut r, h, &t);
        ge_p1p1_to_p3(h, &r);
    }

    ge_p3_dbl(&mut r, h);
    ge_p1p1_to_p2(&mut s, &r);
    ge_p2_dbl(&mut r, &s);
    ge_p1p1_to_p2(&mut s, &r);
    ge_p2_dbl(&mut r, &s);
    ge_p1p1_to_p2(&mut s, &r);
    ge_p2_dbl(&mut r, &s);
    ge_p1p1_to_p3(h, &r);

    for i in (0..64).step_by(2) {
        select(&mut t, i / 2, e[i]);
        ge_madd(&mut r, h, &t);
        ge_p1p1_to_p3(h, &r);
    }
}

pub fn ge_sub(r: &mut GeP1P1, p: &GeP3, q: &GeCached) {
    r.x = p.y + p.x;
    r.y = p.y - p.x;

    r.z = r.x * q.yminusx;
    r.y = r.y * q.yplusx;

    r.t = q.t2d * p.t;
    r.x = p.z * q.z;

    let t0 = r.x + r.x;

    r.x = r.z - r.y;
    r.y = r.z + r.y;

    r.z = t0 - r.t;
    r.t = t0 + r.t;
}

pub fn ge_tobytes(output: &mut [u8; 32], h: &GeP2) {
    let recip = h.z.invert();
    let x = h.x * recip;
    let y = h.y * recip;

    *output = y.to_bytes();

    let sign_bit = x.is_negative() as u8;
    output[31] ^= sign_bit << 7;
}
