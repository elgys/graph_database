/**
This is the libray that Deserilize [u8] into a usefull format.
Made by: Levi van der Griendt
**/

macro_rules! number_deserilize {
    ($t:ty, $name:ident,$n:ident, $size:literal) => {
        pub fn $name(input: &[u8]) -> Result<$t, DeserilizeError> {
            let int_arrays = input[..$size].try_into();
            let int: [u8; $size] = match int_arrays {
                Ok(num) => num,
                Err(e) => {
                    return Err(DeserilizeError {
                        type_input: "$n".to_owned(),
                        text: "Something went wrong reading a number.".to_owned(),
                    })
                }
            };
            let number: $t = $n::from_be_bytes(int);
            return Ok(number)
        }
    };
}

pub mod Deserilizer {
    use std::{
        array::TryFromSliceError,
        error::Error,
        fmt::{self, Display},
        io::Read,
    };

    number_deserilize!(i8, i8_deserilize, i8, 1);
    number_deserilize!(u8, u8_deserilize, u8, 1);
    number_deserilize!(i16, i16_deserilize, i16, 2);
    number_deserilize!(u16, u16_deserilize, u16, 2);
    number_deserilize!(i32, i32_deserilize, i32, 4);
    number_deserilize!(u32, u32_deserilize, u32, 4);
    number_deserilize!(i64, i64_deserilize, i64, 8);
    number_deserilize!(u64, u64_deserilize, u64, 8);
    number_deserilize!(i128, i128_deserilize, i128, 16);
    number_deserilize!(u128, u128_deserilize, u128, 16);

    pub fn uft8_deserilize(input: &[u8], size: usize) -> Result<String, DeserilizeError> {
        let text: &[u8] = &input[..size];
        let s: String = match str::from_utf8(text) {
            Ok(v) => v.to_string(),
            Err(e) => {
                return Err(DeserilizeError {
                    type_input: "string".to_owned(),
                    text: "the string is not a correct uft-8".to_owned(),
                });
            }
        };

        return Ok(s);
    }

    #[derive(Debug, Clone)]
    pub struct DeserilizeError {
        type_input: String,
        text: String,
    }

    impl Display for DeserilizeError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}: {}", self.type_input, self.text)
        }
    }
    impl Error for DeserilizeError {}
}

#[cfg(test)]
mod test_desierilizer {
    use super::Deserilizer;
    use std::array::TryFromSliceError;
    #[test]
    fn i8_deserilize_test() {
        let test = [100, 255, 78];
        let ans: i8 = 100;

        assert_eq!(Deserilizer::i8_deserilize(&test).unwrap(), ans)
    }

    #[test]
    fn u8_deserilize_test() {
        let test = [233, 22, 89];
        let ans: u8 = 233;

        assert_eq!(Deserilizer::u8_deserilize(&test).unwrap(), ans)
    }

    #[test]
    fn i16_deserilize_test() {
        let test = [0x67, 0x78, 0xCC];
        let ans: i16 = 26488;

        assert_eq!(Deserilizer::i16_deserilize(&test).unwrap(), ans)
    }

    #[test]
    #[should_panic]
    fn i16_deserilize_panic_test() {
        let test = [0x67];
        let ans = 2;

        assert_eq!(Deserilizer::i16_deserilize(&test).unwrap(), ans)
    }
    #[test]
    fn u16_deserilize_test() {
        let test = [0x97, 0x78, 0xCC];
        let ans: u16 = 38776;

        assert_eq!(Deserilizer::u16_deserilize(&test).unwrap(), ans)
    }

    #[test]
    #[should_panic]
    fn u16_deserilize_panic_test() {
        let test = [0x67];
        let ans = 2;

        assert_eq!(Deserilizer::u16_deserilize(&test).unwrap(), ans)
    }

    #[test]
    fn i32_deserilize_test() {
        let test = [0x67, 0x78, 0xCC, 0x1A];
        let ans: i32 = 1735969818;

        assert_eq!(Deserilizer::i32_deserilize(&test).unwrap(), ans)
    }

    #[test]
    #[should_panic]
    fn i32_deserilize_panic_test() {
        let test = [0x67];
        let ans = 2;

        assert_eq!(Deserilizer::i32_deserilize(&test).unwrap(), ans)
    }

    #[test]
    fn u32_deserilize_test() {
        let test = [0xA7, 0x78, 0xCC, 0x1A];
        let ans: u32 = 2809711642;

        assert_eq!(Deserilizer::u32_deserilize(&test).unwrap(), ans)
    }

    #[test]
    #[should_panic]
    fn u32_deserilize_panic_test() {
        let test = [0x67];
        let ans = 2;

        assert_eq!(Deserilizer::u32_deserilize(&test).unwrap(), ans)
    }

    #[test]
    fn i64_deserilize_test() {
        let test = [0x67, 0x78, 0xCC, 0x1A, 0x68, 0x00, 0xc7, 0x1F];
        let ans: i64 = 7455933596897953567;

        assert_eq!(Deserilizer::i64_deserilize(&test).unwrap(), ans)
    }

    #[test]
    #[should_panic]
    fn i64_deserilize_panic_test() {
        let test = [0x67];
        let ans = 2;

        assert_eq!(Deserilizer::i64_deserilize(&test).unwrap(), ans)
    }

    #[test]
    fn u64_deserilize_test() {
        let test = [0xA7, 0x78, 0xCC, 0x1A, 0x67, 0xB3, 0x92, 0x8F];
        let ans: u64 = 12067619615320281743;

        assert_eq!(Deserilizer::u64_deserilize(&test).unwrap(), ans)
    }

    #[test]
    #[should_panic]
    fn u64_deserilize_panic_test() {
        let test = [0x67];
        let ans = 2;

        assert_eq!(Deserilizer::u64_deserilize(&test).unwrap(), ans)
    }

    #[test]
    fn i128_deserilize_test() {
        let test = [
            0xe1, 0x23, 0x8e, 0xf9, 0x3c, 0x6f, 0x1c, 0x3a, 0x07, 0x50, 0x10, 0x94, 0xb6, 0x40,
            0x49, 0x1b,
        ];
        let ans: i128 = -41021437630604063852041646220641285861;

        assert_eq!(Deserilizer::i128_deserilize(&test).unwrap(), ans)
    }
    #[test]
    #[should_panic]
    fn i128_deserilize_panic_test() {
        let test = [0x67];
        let ans = 2;

        assert_eq!(Deserilizer::i128_deserilize(&test).unwrap(), ans)
    }

    #[test]
    fn u128_deserilize_test() {
        let test = [
            0xe1, 0x23, 0x8e, 0xf9, 0x3c, 0x6f, 0x1c, 0x3a, 0x07, 0x50, 0x10, 0x94, 0xb6, 0x40,
            0x49, 0x1b,
        ];
        let ans: u128 = 299260929290334399611332961211126925595;

        assert_eq!(Deserilizer::u128_deserilize(&test).unwrap(), ans)
    }

    #[test]
    #[should_panic]
    fn u128_deserilize_panic_test() {
        let test = [0x67];
        let ans = 2;

        assert_eq!(Deserilizer::u128_deserilize(&test).unwrap(), ans)
    }

    #[test]
    fn ascii_deserilize_test() {
        let test: [u8; 5] = [b'H', b'E', b'L', b'L', b'O'];
        let ans: String = "HELLO".to_string();

        assert_eq!(Deserilizer::uft8_deserilize(&test, 5), ans)
    }

    #[test]
    fn uft8_deserilize_test() {}

    #[test]
    #[should_panic]
    fn uft_deserilizer_panic_test() {
        let test: [u8; 1] = [b'H'];
        let ans = "HELLO";

        assert_eq!(Deserilizer::uft8_deserilize(&test, 5), ans);
    }
}
