/**
Table is the base of the sate of a table files in this type are orderd in the following way.
*/

use std::{fs::File, mem, io};

pub mod table 
{
    use std::{fs, io::Write};
    
    #[derive(Debug,Clone, Copy)]
    pub struct TableHeader
    {
        signature: u16, // 2
        version: u16,  // 2
        pub page_size: u32, // 4
        pub b_tree_loc: u32, // 4
        pub data_loc: u32, // 4
        pub relationship_loc: u32, // 4
    }

    impl TableHeader
    {
        pub fn check_correct(&self) -> Result<(),String>
        {
            if self.signature == 0x4442 && self.version == 0 {return Ok(());} 
            else {return Err("File has a incorrect header".into());}
        }

         pub fn file_format(&self) -> [u8; 20]
         {
            let file_binary: [u8; 20];
            let sig = self.signature.to_be_bytes();
            let ver = self.version.to_be_bytes();
            let p_z = self.page_size.to_be_bytes();
            let b_t_l = self.b_tree_loc.to_be_bytes();
            let d_l = self.data_loc.to_be_bytes();
            let r_l = self.relationship_loc.to_be_bytes();
            file_binary = [sig[0], sig[1], ver[0], ver[1],p_z[0], p_z[1], p_z[2], p_z[3], b_t_l[0], b_t_l[1], b_t_l[2], b_t_l[3], d_l[0], d_l[1], d_l[2],d_l[3], r_l[0], r_l[1], r_l[2], r_l[3]];
            return file_binary;
         }

        pub fn write_to_file(&self, file:& mut fs::File) -> std::io::Result<()>
        {
            file.write_all(&self.file_format())?;
            Ok(())
        }

    }

    pub fn build_empty_table_header() -> TableHeader 
    {
        TableHeader 
        {
            signature : 0x4442,
            version : 0,
            page_size : 0,
            b_tree_loc : 0,
            data_loc : 0,
            relationship_loc : 0,
        }
    }

    #[cfg(test)]
    mod test_table 
    {
        
        use std::{fs, io::Read};

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
            assert_eq!([0x44u8, 0x42u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8], text )
        }

        #[test]
        fn  write_to_file_test()
        {
            let test = correct_header();
            let mut file = fs::File::create("foo.txt").expect("the file exist already.");
            test.write_to_file(&mut file).expect("can't write to file");
            let mut content = [0;20];
            file = fs::File::open("foo.txt").expect("can't open the file to read.");
            file.read_exact(& mut content).expect("can't read from the file");
            assert_eq!(content, [0x44u8, 0x42u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 
                0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8, 0x00u8]);
            fs::remove_file("foo.txt").expect("this file doesn't exist");
        }

    }
}


