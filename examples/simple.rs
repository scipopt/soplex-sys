use soplex_sys::SoPlex_create;

fn main() {
    let soplex = unsafe { SoPlex_create() };
    assert!(!soplex.is_null());
    unsafe {
        soplex_sys::SoPlex_free(soplex);
    }
}
