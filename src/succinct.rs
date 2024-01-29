extern "C" {
    fn syscall_keccak_permute(w: *mut u64);
}

#[inline]
pub fn keccakf(state: &mut [u64; WORDS]) {
    unsafe {
        syscall_keccak_permute(state.as_mut_ptr());
    }
}