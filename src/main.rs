use database::table::table;
mod database;
 

fn main() 
{
	let header = table::build_empty_table_header();
	print!("{:?}\n", header);
}


