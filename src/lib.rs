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

#![allow(dead_code)]
#![allow(unsigned_negation)]

/*
 * This structure holds the state for the random number generator.
 */
pub struct Pcg32Random {
    state : u64,
    inc   : u64,
}

// Global pcg32_random struct for global RNG.
static mut PCG32_GLOBAL : Pcg32Random = Pcg32Random { state : 0x853c49e6748fea9b, inc : 0xda3e39cb94b95bdb };

/*
 * Seeds the random number generator.
 */
pub fn pcg32_srandom_r(rng : &mut Pcg32Random, initstate : u64, initseq : u64) {
    rng.state = 0u64;
    rng.inc   = (initseq << 1u64) | 1u64;
    pcg32_random_r(rng);
    rng.state += initstate;
    pcg32_random_r(rng);
}

/*
 * Same as pcg32_srandom_r but uses the global RNG state.
 */
pub unsafe fn pcg32_srandom(seed: u64, seq : u64) -> () {
    pcg32_srandom_r(&mut PCG32_GLOBAL, seed, seq);
}

/*
 * Generate a uniformly distributed 32-bit random number.
 */
pub fn pcg32_random_r(rng : &mut Pcg32Random) -> u32 {
    let oldstate   = rng.state;
    rng.state      = oldstate * 6364136223846793005u64 + rng.inc;
    let xorshifted = (((oldstate >> 18) ^ oldstate) >> 27) as u32;
    let rot        = oldstate >> 59;
    return (xorshifted >> rot) | (xorshifted << ((-rot) & 31));
}

/*
 * Same as pcg32_random_r but uses the global RNG state.
 */
pub unsafe fn pcg32_random() -> u32 {
    return pcg32_random_r(&mut PCG32_GLOBAL);
}

/*
 * Generate a uniformly distributed 32-bit random number n where 0 <= n < bound.
 */
pub fn pcg32_boundedrand_r(rng : &mut Pcg32Random, bound : u32) -> u32 {
    let threshold = -bound % bound;
    loop {
        let r = pcg32_random_r(rng);
        if r >= threshold {
            return r % bound;
        }
    }
}

/*
 * Same as pcg32_boundedrand_r but uses the global RNG state.
 */
pub unsafe fn pcg_boundedrand(bound : u32) -> u32 {
    return pcg32_boundedrand_r(&mut PCG32_GLOBAL, bound);
}
