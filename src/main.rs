#![feature(proc_macro_hygiene, decl_macro)]
pub mod crawler;

fn main() {
    match crawler::crawl::crawl_entity("870715447136366662".into(), crawler::crawl::EntityTypes::Bot) {
        Ok(e) => println!("review avg ratings: {}", e.ratings_avg),
        Err(e) => println!("err: {}", e)
    }
    println!("{}", "cum")
}
