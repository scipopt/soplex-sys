#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        //create a soplex problem
        let mut soplex = unsafe { SoPlex_create() };
        assert!(!soplex.is_null());
    }
}