use ch8_misc::{stats, pig_latin, company};

fn main() {
    println!("Stats for 1, 2, 2, 3:\n {:?}\n", stats::get_stats(&mut [1, 2, 2, 3]));
    println!("hellow world apple first to pig_latin:\n {}\n", 
             pig_latin::text_to_pl("hello world apple first"));

    company::add_from_io();
}
