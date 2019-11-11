use std::os::raw;


extern "C" {
    pub fn sodium_init() -> i32;
    pub fn crypto_box_keypair(pk: *mut raw::c_uchar, sk: *mut raw::c_uchar) -> i32;
}

fn main() {
    unsafe {
        assert_eq!(sodium_init(), 0)
    };

    let mut pk = [0u8; 32];
    let mut sk = [0u8; 32];

    unsafe {
        assert_eq!(crypto_box_keypair(pk.as_mut_ptr(), sk.as_mut_ptr()), 0)
    };

    println!("pk: {:?}", pk);
    println!("sk: {:?}", sk);
}
