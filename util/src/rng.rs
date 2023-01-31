extern "C" {
    #[no_mangle]
    static random_bytes: *const c_void;
}

extern "aapcs" {
    #[link_name = "init_random_bytes"]
    fn init_random_bytes();
}

// TODO: MAKE A SAFE WRAPPER AROUND THIS FOR RNG GENERATION (use a once cell probs)
pub unsafe fn init_rng() {

} 
