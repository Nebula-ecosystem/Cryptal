pub(crate) struct Scalar(pub(crate) [u8; 32]);
pub(crate) struct Slide(pub(crate) [i8; 256]);

impl Scalar {
    pub fn slide(&self) -> Slide {
        let a = &self.0;
        let mut r = [0i8; 256];

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

        Slide(r)
    }
}
