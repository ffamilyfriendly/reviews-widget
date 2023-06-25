use std::{error::Error, sync::Arc, path::PathBuf};

use headless_chrome::{Browser, browser::{default_executable, transport::{Transport, SessionId}, tab::RequestPausedDecision}, Element, Tab, protocol::cdp::{Fetch::{events::RequestPausedEvent, FailRequest}, Network::ErrorReason}};
use urlencoding::decode;

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


// I straight up stole this from stackoverflow. I'm as lost as you are
fn get_attr(elt: &headless_chrome::Element, attr: &str) -> String {
    match elt.call_js_fn(&format!("function() {{ return this.getAttribute(\"{}\"); }}", attr), vec![], true).unwrap().value {
        Some(s) => s.to_string(),
        _ => panic!("Expected string"),
    }
}

fn sanetize_image_url( url: String ) -> String {
    let split: Vec<&str> = url.split("?url=").collect();
    
    decode(split.get(1).unwrap()).expect("UFT-8").to_string()
}

pub fn crawl_reviews( reviews: Vec<Element<'_>>) -> Result<Vec<Review>, Box<dyn Error>> {
    let mut reviews_list: Vec<Review> = Vec::new();
    for review in reviews {
        /*
            Scroll to the review.
            This is required as otherwise the profile picture does not load and instead returns garbage base64 image data with height & width of 0
            I swear to god it took me ages to debug this as when I debugged I had to scroll to the images in the first place which obviously loaded them
            my existence is a mix of booze filled adventures and pure pain
        */
        review.scroll_into_view()?;
        // Ehem... anyhow :D here we get some elements and what elements we get should be pretty clear by my epic and pattented naming convention
        let ratings_element = review.find_element(".chakra-stack.css-trv47m")?;
        let username_element = review.find_element("a.css-1hongtb")?;
        let review_text_element = review.find_element("p.css-542wex")?;
        let review_likes_element = review.find_element("p.css-2qyx8u")?;
        // there are hooligans out there who do not have a discord profile picture. For some dumb reason these "people" must be handled
        let image_element: Option<Element<'_>> = match review.find_element(".css-1emigkr>span>img") {
            Ok(e) => Some(e),
            Err(_) => None
        };

        // get the "data-stars" attribute and filter out any non-digit characters 
        let stars: String = get_attr(&ratings_element, "data-stars").chars().filter(|c| c.is_digit(10)).collect();

        println!("...done");

        // push the silly review to our silly little reviews list
        reviews_list.push( Review {
            display_name: username_element.get_inner_text()?,
            votes: review_likes_element.get_inner_text()?.parse::<u8>()?,
            text: review_text_element.get_inner_text()?,
            rating: stars.parse::<u16>()?,
            profile_picture: match image_element {
                Some(e) => {
                    let src_attr = get_attr(&e, "src");
                    if src_attr.contains("base64") {
                        None
                    } else {
                        Some(sanetize_image_url(src_attr))
                    }
                },
                None => None
            },
        } )
    }
    Ok(reviews_list)
}

pub fn crawl_entity( id: String, entity_type: EntityTypes, use_google_cache: Option<bool> ) -> Result<ReviewsListing, Box<dyn Error>> {
    let mut path = PathBuf::new();
    path.push("/home/john/.config/chromium/Default");
    let browser = Browser::new(
        headless_chrome::LaunchOptionsBuilder::default()
        .path(Some(default_executable().unwrap()))
        .headless(false)
        .sandbox(false)
        .user_data_dir(Some(path))
        .build()
        .expect("Something went real wrong")
    )?;

    let tab = browser.new_tab()?;
    tab.set_user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/114.0.0.0 Safari/537.36", Some("sv"), None)?;
    //tab.enable_stealth_mode()?;

    //tab.enable_stealth_mode()?;
    // Enabling fetch so we can intercept requests
    tab.enable_fetch(None, None)?;

    /*
        Top.gg loads some "we value some privacy" element from an advertiser who obviously does not value or privacy.
        In fact, it values the very opposite as that is how it makes money.
        This element is inside an iframe and when shown also blocks scrolling on the main top.gg page.

        We deal with this by simply blocking the iframe from loading in the first place :)
    */
    tab.enable_request_interception(Arc::new(
         | _transport: Arc<Transport>, _session_id: SessionId, intercepted: RequestPausedEvent | {
            let blocked_origins = vec![ "privacy-mgmt.com", "kumo.network-n.com", "moatads.com", "scorecardresearch.com", "videoplayerhub.com", "permutive.app", "pbstck.com", "cpx.to", "primis.tech", "cdn-cgi/images/trace/managed/js" ];
            for blocked_origin in blocked_origins {
                if intercepted.params.request.url.contains(blocked_origin) {
                    println!("blocked request {} due to containing blocked origin {}", intercepted.params.request.url, blocked_origin);
                    return RequestPausedDecision::Fail(FailRequest { request_id: intercepted.params.request_id, error_reason: ErrorReason::BlockedByClient });
                }
            }
            println!("did not block: {}", intercepted.params.request.url);
            return RequestPausedDecision::Continue(None);
        }
    ))?;
    
    let mut google_cache_prefix = "";
    if use_google_cache.unwrap_or(false) {
        google_cache_prefix = "https://webcache.googleusercontent.com/search?q=cache:";
    }
    let url: String = match entity_type {
        EntityTypes::Bot => format!("{}https://top.gg/bot/{}", google_cache_prefix, id),
        EntityTypes::Server => format!("{}https://top.gg/server/{}", google_cache_prefix, id)
    };
    tab.navigate_to(&url)?;

    // Gets all the buttons from the bottom row that you can click to go to that review page
    //let mut reviews_pages_elements = tab.wait_for_elements(".chakra-stack.css-ztemmk>button")?;
    // Remove the first button as it is page 1 and we are already on page 1
    //reviews_pages_elements.remove(0);
    let avg_reviews_element = tab.wait_for_element("p[aria-label='average score']")?;
    let reviews_count_element = tab.find_element("div.css-13igg3x>p")?;

    let mut reviews_list: Vec<Review> = Vec::new(); 

    // This aptly named function does indeed do "thing".
    // the thing is:
    fn do_thing(reviews_list: &mut Vec<Review>, tab: &Arc<Tab>) -> Result<(), Box<dyn Error>> {
        println!("getting elements:");
        // Get a Vec (array in un-based langs) that contains all the review elements
        let reviews_list_element = tab.wait_for_elements("div.chakra-stack.css-587fn9>.css-ay5wn")?;
        // Pass this Vec to the actuall review parser
        reviews_list.append(&mut crawl_reviews(reviews_list_element)?);
        Ok(())
    }

    do_thing(&mut reviews_list, &tab)?;

    // below is commented code for loading more pages. It does not work :(

    /*for page_button in reviews_pages_elements {
        println!("clicking");
        page_button.click()?;
        do_thing(&mut reviews_list, &tab)?;
    }*/

    /*while has_more {
        let reviews_list_element = tab.wait_for_elements("div.chakra-stack.css-587fn9>.css-ay5wn")?;
        println!("dealing with page:");

        // pages list: chakra-stack css-ztemmk

        reviews_list.append(&mut crawl_reviews(reviews_list_element)?);
    }*/

    Ok(ReviewsListing {
        id: id,
        ratings_avg: avg_reviews_element.get_inner_text()?.parse::<f32>()?,
        reviews: reviews_list,
        reviews_count: reviews_count_element.get_inner_text()?.split(' ').collect::<Vec<&str>>()[0].parse::<u16>()?
    })
}