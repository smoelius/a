#![no_main]
use cargo_libafl_helper::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(n): Result<usize, _> = bincode::deserialize(data) else {
        return;
    };
    a::target(n);
});
