use std::mem;

mod table; 

fn main() 
{
	let header = table::table::build_empty_table_header();
	header.file_format();
}


