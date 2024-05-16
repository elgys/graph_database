
 mod table; 

fn main() 
{
	let header = table::table::build_empty_table_header();
	print!("{:?}", header);
}


