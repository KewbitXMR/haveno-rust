use std::time::{Duration, Instant};
use std::ptr;

extern "C" {
    fn randomx_alloc_cache(flags: u32) -> *mut std::ffi::c_void;
    fn randomx_init_cache(cache: *mut std::ffi::c_void, key: *const std::ffi::c_void, key_size: usize);
    fn randomx_release_cache(cache: *mut std::ffi::c_void);
    fn randomx_create_vm(flags: u32, cache: *mut std::ffi::c_void, dataset: *mut std::ffi::c_void) -> *mut std::ffi::c_void;
    fn randomx_destroy_vm(vm: *mut std::ffi::c_void);
    fn randomx_calculate_hash(vm: *mut std::ffi::c_void, input: *const std::ffi::c_void, input_size: usize, output: *mut std::ffi::c_void);
}

const RANDOMX_FLAG_DEFAULT: u32 = 0x01 | 0x04; // JIT | FULL_MEM
const DIFFICULTY_ZEROS: usize = 5; // Number of leading zeros required (hex)

static INIT: Once = Once::new();
static mut CACHE: *mut std::ffi::c_void = ptr::null_mut();

pub fn initialize(key: &[u8]) {
    unsafe {
        INIT.call_once(|| {
            let cache = randomx_alloc_cache(RANDOMX_FLAG_DEFAULT);
            if cache.is_null() {
                panic!("Failed to allocate RandomX cache");
            }
            randomx_init_cache(cache, key.as_ptr() as *const _, key.len());
            CACHE = cache;
        });
    }
}

pub fn solve_pow(seed: &[u8]) -> (Vec<u8>, u64, Duration) {
    let mut rng = rand::thread_rng();
    let mut nonce = 0u64;
    let start = Instant::now();

    unsafe {
        let vm = randomx_create_vm(RANDOMX_FLAG_DEFAULT, CACHE, ptr::null_mut());
        if vm.is_null() {
            panic!("Failed to create RandomX VM");
        }

        let mut output = [0u8; 32];

        loop {
            let mut input = Vec::new();
            input.extend_from_slice(seed);
            input.extend_from_slice(&nonce.to_le_bytes());

            randomx_calculate_hash(
                vm,
                input.as_ptr() as *const _,
                input.len(),
                output.as_mut_ptr() as *mut _
            );

            if output.iter().take(DIFFICULTY_ZEROS).all(|&b| b == 0) {
                randomx_destroy_vm(vm);
                return (output.to_vec(), nonce, start.elapsed());
            }

            nonce += 1;
        }
    }
}