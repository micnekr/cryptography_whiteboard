#[cfg(test)]
mod tests {
    use cryptography_whiteboard::bitstream::BitVec;

    #[test]
    fn test_bit_vec_from() {
        let raw_val = "0110101001";
        let vec: BitVec = raw_val.try_into().unwrap();
        assert_eq!(
            vec.len(),
            raw_val.len(),
            "The vector should have the length of the string it was created from"
        );

        let mut vec2 = BitVec::new();
        vec2 += false;
        vec2 += true;
        vec2 += true;
        vec2 += false;

        vec2 += true;
        vec2 += false;
        vec2 += true;
        vec2 += false;
        vec2 += false;
        vec2 += true;

        assert_eq!(vec, vec2, "The vector should have the expected value");
    }
}
