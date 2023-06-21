use std::error::Error;

use headless_chrome::{Browser, browser::default_executable, Element};
//use headless_chrome::protocol::cdp::Page;

#[derive(Debug)]
pub struct Review {
    pub display_name: String,
    pub profile_picture: Option<String>,
    pub text: String,
    pub rating: u16,
    pub votes: u8
}

#[derive(Debug)]
pub struct ReviewsListing {
    pub id: String,
    pub ratings_avg: f32,
    pub reviews: Vec<Review>,
    pub reviews_count: u16
}

pub enum EntityTypes {
    Bot,
    Server
}

fn get_attr(elt: &headless_chrome::Element, attr: &str) -> String {
    match elt.call_js_fn(&format!("function() {{ return this.getAttribute(\"{}\"); }}", attr), vec![], true).unwrap().value {
        Some(s) => s.to_string(),
        _ => panic!("Expected string"),
    }
}

pub fn crawl_entity( id: String, entity_type: EntityTypes ) -> Result<ReviewsListing, Box<dyn Error>> {
    let browser = Browser::new(
        headless_chrome::LaunchOptionsBuilder::default()
        .path(Some(default_executable().unwrap()))
        .headless(false)
        .sandbox(false)
        .build()
        .expect("Something went real wrong")
    )?;
    let tab = browser.new_tab()?;
    
    
    let url: String = match entity_type {
        EntityTypes::Bot => format!("https://top.gg/bot/{}#reviews", id),
        EntityTypes::Server => format!("https://top.gg/server/{}#reviews", id)
    };
    tab.navigate_to(&url)?;
    tab.wait_until_navigated()?;

    let avg_reviews_element = tab.find_element("p[aria-label='average score']")?;
    let reviews_count_element = tab.find_element("div.css-13igg3x>p")?;
    let reviews_list_element = tab.find_elements("div.chakra-stack.css-587fn9>.css-ay5wn")?;

    let mut reviews_list: Vec<Review> = Vec::new(); 
    
    for review in reviews_list_element {
        let ratings_element = review.find_element(".chakra-stack.css-trv47m")?;
        let username_element = review.find_element("a.css-1hongtb")?;
        let review_text_element = review.find_element("p.css-542wex")?;
        let review_likes_element = review.find_element("p.css-2qyx8u")?;
        let image_element: Option<Element<'_>> = match review.find_element(".css-1emigkr>span>img") {
            Ok(e) => Some(e),
            Err(_) => None
        };

        let stars: String = get_attr(&ratings_element, "data-stars").chars().filter(|c| c.is_digit(10)).collect();

        reviews_list.push( Review {
            display_name: username_element.get_inner_text()?,
            votes: review_likes_element.get_inner_text()?.parse::<u8>()?,
            text: review_text_element.get_inner_text()?,
            rating: stars.parse::<u16>()?,
            profile_picture: match image_element {
                Some(e) => {
                    Some(get_attr(&e, "src"))
                },
                None => None
            },
        } )
    }

    Ok(ReviewsListing {
        id: id,
        ratings_avg: avg_reviews_element.get_inner_text()?.parse::<f32>()?,
        reviews: reviews_list,
        reviews_count: reviews_count_element.get_inner_text()?.split(' ').collect::<Vec<&str>>()[0].parse::<u16>()?
    })
}