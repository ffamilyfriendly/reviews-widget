use std::error::Error;

use headless_chrome::{Browser, browser::default_executable};
//use headless_chrome::protocol::cdp::Page;

#[derive(Debug)]
pub struct Review {
    pub display_name: String,
    pub profile_picture: Option<String>,
    pub rating: u8,
    pub votes: u8
}

#[derive(Debug)]
pub struct ReviewsListing {
    pub id: String,
    pub ratings_avg: f32,
    pub reviews: Vec<Review>
}

pub enum EntityTypes {
    Bot,
    Server
}

pub fn crawl_entity( id: String, entity_type: EntityTypes ) -> Result<ReviewsListing, Box<dyn Error>> {
    let browser = Browser::new(
        headless_chrome::LaunchOptionsBuilder::default()
        .path(Some(default_executable().unwrap()))
        .headless(false)
        .sandbox(true)
        .build()
        .expect("Something went real wrong")
    )?;
    let tab = browser.new_tab()?;
    
    
    let url: String = match entity_type {
        EntityTypes::Bot => format!("https://top.gg/bot/{}", id),
        EntityTypes::Server => format!("https://top.gg/server/{}", id)
    };
    tab.navigate_to(&url)?;
    tab.wait_until_navigated()?;

    let avg_reviews_element = tab.wait_for_element("p[aria-label='average score']")?;
    let _reviews_count_element = tab.wait_for_element("div.css-13igg3x>p")?;
    let _reviews_list_element = tab.wait_for_elements("div.chakra-stack.css-587fn9>.css-ay5wn")?;

    let reviews_list: Vec<Review> = Vec::new(); 
    
    Ok(ReviewsListing {
        id: id,
        ratings_avg: avg_reviews_element.get_inner_text()?.parse::<f32>()?,
        reviews: reviews_list
    })
}