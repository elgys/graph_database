/**
Table is the base of the sate of a table files in this type are orderd in the following way.
*/

#[feature(test)]
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
            let file_binary: [u8; 16] = [self.signature.to_be_bytes()[0],self.signature.to_be_bytes()[1],
                self.version.to_be_bytes()[0], self.version.to_be_bytes()[1],
                self.b_tree_loc.to_be_bytes()[0], self.b_tree_loc.to_be_bytes()[1], self.b_tree_loc.to_be_bytes()[2], self.b_tree_loc.to_be_bytes()[3],
                self.data_loc.to_be_bytes()[0], self.data_loc.to_be_bytes()[1], self.data_loc.to_be_bytes()[2], self.data_loc.to_be_bytes()[3],
                self.relationship_loc.to_be_bytes()[0], self.relationship_loc.to_be_bytes()[1], self.relationship_loc.to_be_bytes()[2], self.relationship_loc.to_be_bytes()[3]];
        
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
        extern crate test;
        use super::*;
        use test::Bencher;


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


