use chrono::naive::NaiveTime;
use chrono::Local;
use reqwest;
use std::error::Error;
use std::str;
use texting_robots::Robot;

//based on this article https://www.zenrows.com/blog/robots-txt-web-scraping#robots-txt-web-scraping

fn parse_visit_time(input: &str) -> Option<String> {
    // Find the position of "Visit-time:"
    if let Some(visit_time_index) = input.find("Visit-time:") {
        // Skip the "Visit-time:" part
        let remaining_string = &input[visit_time_index + "Visit-time:".len()..];

        // Find the end of the visit time value (up to the next newline)
        if let Some(newline_index) = remaining_string.find('\n') {
            let visit_time = &remaining_string[..newline_index].trim();
            return Some(visit_time.to_string());
        }
    }

    None
}

pub async fn process_robots_txt(input: &str) -> Result<Vec<String>, Box<dyn Error>> {
    //robots.txt has to be on the root of the site
    //when a url is split by a / you get -> ["https:", "", "site.com", "about", ""] --> https://site.com/about/
    let paths = input.split('/').collect::<Vec<&str>>();

    //base path will always be at position 2 https:// included
    let base_path = paths[..=2].join("/");

    //ping the robots.txt file and see if it exists on the following site

    let res = reqwest::get(format!("{}{}{}", base_path, "/", "robots.txt")).await?;

    if !res.status().is_success() {
        info!("The following site does not have a robots.txt file, if this is your site - it is advised that you create one. \nhttps://developers.google.com/search/docs/crawling-indexing/robots/intro#:~:text=A%20robots.txt%20file%20tells,or%20password%2Dprotect%20the%20page. ");
    }

    let txt = res.bytes().await?;

    let robot = Robot::new("Crawler", &txt).unwrap();

    let mut notified_uncrawlable = false;

    //see if we are allowed to crawl the site
    if !str::from_utf8(&txt).unwrap().contains("User-agent: *") || !robot.allowed("/") {
        warn!("we are not allowed to crawl the following site :(");
        notified_uncrawlable = true;
    }

    //see if we are in visiting time hours
    if let Some(visit_time) = parse_visit_time(str::from_utf8(&txt).unwrap()) {
        let visit_time_start_end: Vec<&str> = visit_time.split('-').collect();

        //only check the visit time if it is in time format - the other format is for requests
        if visit_time_start_end.len() == 2 {
            if let Ok(start_time) = NaiveTime::parse_from_str(visit_time_start_end[0], "%H%M") {
                if let Ok(end_time) = NaiveTime::parse_from_str(visit_time_start_end[1], "%H%M") {
                    let current_datetime = Local::now();
                    if !(current_datetime.time() >= start_time
                        && current_datetime.time() <= end_time)
                    {
                        warn!("{}", format!("Based on the robots.txt, we are not within visiting hours to crawl this website. Please try again at {}", start_time.format("%I:%M %p")));
                    }
                } else {
                    error!("Failed to parse end time");
                    info!("We were not able to verify that we are following compliances via the robots.txt file, if this is your site double check the Visit-Time");
                }
            } else {
                error!("Failed to parse start time");
                info!("We were not able to verify that we are following compliances via the robots.txt file, if this is your site double check the Visit-Time");
            }
        }
    }

    //see if the path entered is allowed to crawl
    let path = paths[3..paths.len()].join("/");

    if !robot.allowed(&path) && !notified_uncrawlable {
        info!("We are not allowed to crawl the path, please try another page for more information about this site.");
    }

    //possibly find sitemap (sitemaps aren't always listed in the robots.txt file)
    if robot.sitemaps.len() == 0 {
        info!("there is no mention of a sitemap in the robots.txt, if this is your site, it is recommended that you add one. \nhttps://developers.google.com/search/docs/crawling-indexing/sitemaps/build-sitemap?hl=en&sjid=15467058254278674160-NC&visit_id=638408798372148862-3622568734&rd=1#addsitemap")
    }

    return Ok(robot.sitemaps);
}
