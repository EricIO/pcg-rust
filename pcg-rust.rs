// Copyright 2015 Eric Skoglund
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at

//     http://www.apache.org/licenses/LICENSE-2.0

// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[allow(dead_code)]
mod pcg {

    struct pcg32_random {
        state : u64,
        inc   : u64,
    }

    static mut PCG32_GLOBAL : pcg32_random = pcg32_random { state : 0x853c49e6748fea9b, inc : 0xda3e39cb94b95bdb };

    pub fn pcg32_srandom_r(rng : &mut pcg32_random, initstate : u64, initseq : u64) {
        rng.state = 0u64;
        rng.inc   = (initseq << 1u64) | 1u64;
        pcg32_random_r(rng);
        rng.state += initstate;
        pcg32_random_r(rng);
    }

    pub unsafe fn pcg32_srandom(seed: u64, seq : u64) -> () {
        pcg32_srandom_r(&mut PCG32_GLOBAL, seed, seq);
    }

    pub fn pcg32_random_r(rng : &mut pcg32_random) -> u32 {
        let oldstate   = rng.state;
        rng.state      = oldstate * 6364136223846793005u64 + rng.inc;
        let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
        let rot        = oldstate >> 59;
        return (xorshifted >> rot) | (xorshifted << ((-rot) & 31));
    }

    pub unsafe fn pcg32_random() -> u32 {
        return pcg32_random_r(&mut PCG32_GLOBAL);
    }

    pub fn pcg32_boundedrand_r(rng : &mut pcg32_random, bound : u32) -> u32 {
        let threshold = -bound % bound;
        loop {
            let r = pcg32_random_r(rng);
            if r >= threshold {
                return r % bound;
            }
        }
    }

    pub unsafe fn pcg_boundedrand(bound : u32) -> u32 {
        return pcg32_boundedrand_r(&mut PCG32_GLOBAL, bound);
    }
}
