#![feature(proc_macro_hygiene, decl_macro)]
pub mod crawler;

fn main() {
    match crawler::crawl::crawl_entity("870715447136366662".into(), crawler::crawl::EntityTypes::Bot) {
        Ok(e) => {
            for review in e.reviews {
               println!("{} ({}): {}/5 {}", review.display_name, review.profile_picture.unwrap_or("<none>".to_string()), review.rating, review.text) 
            }
        },
        Err(e) => println!("err: {}", e)
    }
}
