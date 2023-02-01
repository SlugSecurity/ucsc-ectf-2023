use core::ffi::c_uchar;

const fn get_random_bytes_size() -> usize {
    const FILE: &str = include_str!("../../rand_uninit_memory/rand_uninit_memory.h");

    let mut file_idx = 0;
    let mut line_len;
    let mut line_buff = [0; FILE.len()];
    let eq_check = b"#define RANDOM_BYTES_SIZE ";

    'outer: while file_idx < FILE.len() {
        line_len = 0;

        // Get each line and put it in line_buff.
        while file_idx < FILE.len() && FILE.as_bytes()[file_idx] != b'\n' {
            line_buff[line_len] = FILE.as_bytes()[file_idx];

            line_len += 1;
            file_idx += 1;
        }

        file_idx += 1;

        // This means we don't have enough bytes for this line to be the one.
        // Or it had too many to be valid (more than 20 characters).
        if line_len <= eq_check.len() || line_len > eq_check.len() + 20 {
            continue;
        }

        let mut line_idx = 0;

        // Check if our line begins with eq_check.
        while line_idx < eq_check.len() {
            if line_buff[line_idx] != eq_check[line_idx] {
                continue 'outer;
            }

            line_idx += 1;
        }

        let mut num = 0;

        // Now, we just need to get the number if there is one.
        while line_idx < line_len {
            if !line_buff[line_idx].is_ascii_digit() {
                continue 'outer;
            }

            num = num * 10 + (line_buff[line_idx] - b'0') as usize;

            line_idx += 1;
        }

        assert!(
            num == 1024,
            "Our old size was 1024. Now it is not. Did you mean to do this?"
        );

        return num;
    }

    assert!(false, "Bad header file. No size present.");
    unreachable!()
}

const RANDOM_BYTES_SIZE: usize = get_random_bytes_size();

#[link(name = "rand_uninit_memory", kind = "static")]
extern "aapcs" {
    static mut random_bytes: [c_uchar; RANDOM_BYTES_SIZE];

    fn init_random_bytes();
}
