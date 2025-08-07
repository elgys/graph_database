use database::table::table;
mod database;

fn main() {
    let test = [1, 2, 3, 4];
    let test1 = &test[..2];
    print!("{:?}\n", (test1));
    let header = table::build_empty_table_header();
    print!("{:?}\n", header);
}
