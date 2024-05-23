
/**
Table is the base of the sate of a table files in this type are orderd in the following way.
*/

pub mod table 
{
    use std::{fs::File, mem};


    #[derive(Debug,Clone, Copy)]
    pub struct TableHeader
    {
        signature: u16, // 2
        version: u16,  // 2
        pub b_tree_loc: u32, // 4
        pub data_loc: u32, // 4
        pub relationship_loc: u32, // 4
    }

    impl TableHeader
    {
        pub fn check_correct(&self) -> Result<(),String>
        {
            if (self.signature == 0x4442 && self.version == 0) {return Ok(());} 
            else {return Err("File has a incorrect header".into());}
        }
   
         pub fn file_format(&self) -> [u8; 16]
         {
            let mut file_binary: [u8; 16];
            file_binary = [0x44,0x42,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
            let mut array_version = &mut file_binary[2 .. 4];
            array_version = self.version.to_le_bytes();

            return file_binary;
         }
    }

    pub fn build_empty_table_header() -> TableHeader 
    {
        TableHeader 
        {
            signature : 0x4442,
            version : 0,
            b_tree_loc : 0,
            data_loc : 0,
            relationship_loc : 0,
        }
    }

    

    #[cfg(test)]
    mod test_table 
    {
        use super::*;


        fn correct_header () -> TableHeader
        {
            return build_empty_table_header();
        }

        #[test]
        fn check_header()
        {
            let test = correct_header();
            assert_eq!( test.check_correct() , Ok(()) );
        }

        #[test]
        #[should_panic]
        fn faulty_header()
        {
            let mut  test = correct_header();
            test.signature = 0x4044;
            assert_eq!( test.check_correct() , Ok(()) );
        }

        #[test]
        fn empty_file_header_text()
        {
            let test = correct_header();
            let text = test.file_format();
            assert_eq!([0x44u8, 0x42u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8], text )
        }
    }
    
}


