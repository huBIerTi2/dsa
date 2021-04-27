use std::cmp;

const B: usize = 16;
const M: usize = 1 << B;
const ALPHA: f64 = 0.7213/(1.0+1.079/(M as f64));
// const ALPHA: f64 = 0.709;
const R: f64 = ((1 as i64) << 32) as f64;

pub struct HyperLogLog {
    regs: [u8; M],
    zeros: i32
}

let mut hlls: Vec<HyperLogLog>;

ocaml_interop::impl_conv_ocaml_record! {
    HyperLogLog {
        regs: [u8; M],
        zeros: i32
    }
}

#[inline(always)]
// usually rho is the position of the MSB, but this is equivalent as per Flajolet et al.
fn get_rho(x: u64) -> u8 {
    return (x.trailing_zeros()+1) as u8;
}

#[inline(always)]
fn ignore_left_bytes(n_bytes: usize, value: u64) -> u64 {
    let mask: u64 = (1 << (64-n_bytes))-1;
    return value & mask;
}

impl HyperLogLog {
    #[inline(always)]
    pub fn add(&mut self, hash: u64) {
        let addr = hash >> (64-B);
        let rest = ignore_left_bytes(B, hash);
        if self.regs[addr as usize] == 0 {
            self.zeros -= 1;
        }
        self.regs[addr as usize] = cmp::max(self.regs[addr as usize], get_rho(rest));
    }

    fn count_zeros(&self) -> i32 {
        let mut s = 0;
        for i in self.regs.iter() {
            if *i == 0 {
                s += 1;
            }
        }
        s
    }

    #[inline(always)]
    fn estimate(&self) -> f64 {
        let mut s: f64 = 0.0;
        for i in self.regs.iter() {
            s += 2f64.powi(-(*i as i32));
        }

        let e:f64 = ALPHA*(M as f64).powi(2)/s;
        if e <= 2.5*(M as f64) {
            let v = self.zeros;
            if v == 0 {
                e
            } else {
                M as f64 * (M as f64/v as f64).ln()
            }
        } else if e <= R/30.0 {
            e
        } else {
            -R * (1.0 - e/R).ln()
        }
    }

    // estimate cardinality of multiset
    pub fn cardinality(&self) -> i32 {
        self.estimate().round() as i32
    }
}


pub fn init() -> i32 {
    let mut hll = HyperLogLog {
        regs: [0; M],
        zeros: M as i32
    };
    return hll;
}
