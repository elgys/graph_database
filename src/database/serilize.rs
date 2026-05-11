/**
This is the libray that Serilize data into u8 format.
Made by: Levi van der Griendt
**/

macro_rules! number_serilize {
    ($t:ty, $n:ident, $size:literal) => {
        pub fn $n(x: $t, buffer: &mut [u8]) {
            buffer.clone_from_slice(&x.to_be_bytes());
        }
    };
}

pub trait Serilize {
    fn serilize(&self) -> Vec<u8>;
}

pub mod serilizer {
    number_serilize!(i8, i8_serilize, 1);
    number_serilize!(u8, u8_serilize, 1);
    number_serilize!(i16, i16_serilize, 2);
    number_serilize!(u16, u16_serilize, 2);
    number_serilize!(i32, i32_serilize, 4);
    number_serilize!(u32, u32_serilize, 4);
    number_serilize!(i64, i64_serilize, 8);
    number_serilize!(u64, u64_serilize, 8);
    number_serilize!(i128, i128_serilize, 16);
    number_serilize!(u128, u128_serilize, 16);

    pub fn uft8_serilize(input: &str, buffer: &mut [u8]) {
        let bytes = input.as_bytes();
        let size = bytes.len().min(buffer.len() - 1);

        buffer[..size].copy_from_slice(&bytes[..size]);
        buffer[size] = 0;
    }
}

#[cfg(test)]
mod test_serilizer {

    use super::serilizer;

    #[test]
    fn i8_serilize_test() {
        let test: [u8; 1] = [100];
        let tested: i8 = 100;
        let mut buffer: [u8; 1] = [0];

        serilizer::i8_serilize(tested, &mut buffer);
        assert_eq!(buffer, test);
    }

    #[test]
    fn u8_serilize_test() {
        let test: [u8; 1] = [100];
        let tested: u8 = 100;

        let mut buffer: [u8; 1] = [0];

        serilizer::u8_serilize(tested, &mut buffer);

        assert_eq!(buffer, test);
    }

    #[test]
    fn i16_serilize_test() {
        let ans: [u8; 2] = [0x8A, 0xD0];
        let test: i16 = -30000;
        let mut buffer: [u8; 2] = [0, 0];

        serilizer::i16_serilize(test, &mut buffer);

        assert_eq!(buffer, ans);
    }

    #[test]
    fn u16_serilize_test() {
        let ans: [u8; 2] = [0x80, 0x15];
        let test: u16 = 32789;
        let mut buffer: [u8; 2] = [0, 0];

        serilizer::u16_serilize(test, &mut buffer);

        assert_eq!(buffer, ans);
    }

    #[test]
    fn i32_serilize_test() {
        let ans: [u8; 4] = [0x05, 0xE8, 0x5B, 0x3C];
        let test: i32 = 99113788;
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        serilizer::i32_serilize(test, &mut buffer);
        assert_eq!(buffer, ans);
    }

    #[test]
    fn u32_serilize_test() {
        let ans: [u8; 4] = [0x05, 0xE8, 0x5B, 0x3C];
        let test: u32 = 99113788;
        let mut buffer: [u8; 4] = [0, 0, 0, 0];

        serilizer::u32_serilize(test, &mut buffer);
        assert_eq!(buffer, ans);
    }

    #[test]
    fn i64_serilize_test() {
        let ans: [u8; 8] = [0x53, 0x44, 0x48, 0x35, 0xEC, 0x58, 0x00, 0x00];
        let test: i64 = 6_000_000_000_000_000_000;
        let mut buffer: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

        serilizer::i64_serilize(test, &mut buffer);
        assert_eq!(buffer, ans);
    }

    #[test]
    fn u64_serilize_test() {
        let ans: [u8; 8] = [0x53, 0x44, 0x48, 0x35, 0xEC, 0x58, 0x00, 0x00];
        let test: u64 = 6_000_000_000_000_000_000;
        let mut buffer: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

        serilizer::u64_serilize(test, &mut buffer);
        assert_eq!(buffer, ans);
    }

    #[test]
    fn i128_serilize_test() {
        let ans: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F, 0xB7, 0x97, 0x0E, 0xCF, 0x8C, 0x97, 0xD0,
            0x00, 0x00,
        ];
        let test: i128 = 149_780_000_000_000_000_000_000;
        let mut buffer: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        serilizer::i128_serilize(test, &mut buffer);
        assert_eq!(buffer, ans);
    }

    #[test]
    fn u128_serilize_test() {
        let ans: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x1F, 0xB7, 0x97, 0x0E, 0xCF, 0x8C, 0x97, 0xD0,
            0x00, 0x01,
        ];
        let test: u128 = 149_780_000_000_000_000_000_001;
        let mut buffer: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

        serilizer::u128_serilize(test, &mut buffer);
        assert_eq!(buffer, ans);
    }

    #[test]
    fn uft8_serilize_test() {
        let ans: [u8; 6] = [0x48, 0x61, 0x6c, 0x6c, 0x6f, 0x00];
        let test: &str = "Hallo";
        let mut buffer: [u8; 6] = [0; 6];

        serilizer::uft8_serilize(test, &mut buffer);

        assert_eq!(buffer, ans);
    }
    #[test]
    fn uft8_serilize_to_big() {
        let ans: [u8; 3] = [0x48, 0x61, 0x00];
        let test: &str = "Hallo";
        let mut buffer: [u8; 3] = [0; 3];

        serilizer::uft8_serilize(test, &mut buffer);

        assert_eq!(buffer, ans);
    }
}
