#![feature(proc_macro_hygiene, decl_macro)]
pub mod crawler;

fn main() {
    // 870715447136366662
    let mut client = crawler::crawl::CrawlerClient::new();

    println!("nr 1:");
    match client.get_bot("870715447136366662".to_string()) {
        Ok(e) => {
            for review in e.reviews {
               println!("{} ({}): {}/5 {}", review.display_name, review.profile_picture.unwrap_or("<none>".to_string()), review.rating, review.text) 
            }
        },
        Err(e) => println!("err: {}", e)
    }
    println!("nr 2:");
    match client.get_bot("870715447136366662".to_string()) {
        Ok(e) => {
            for review in e.reviews {
               println!("{} ({}): {}/5 {}", review.display_name, review.profile_picture.unwrap_or("<none>".to_string()), review.rating, review.text) 
            }
        },
        Err(e) => println!("err: {}", e)
    }
}
