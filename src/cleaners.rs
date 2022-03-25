use url::Url;
use crate::StripResult;

pub fn amazon(url: &Url) -> StripResult {
    if url.host() != Some(url::Host::Domain("www.amazon.com")) {
        return StripResult::NoMatch
    }
    let mut url = url.clone();
    // Unless we are searching something, drop the query string
    if url.path() != "/s" {
        url.set_query(None)
    } else {
        use std::borrow::Cow;
        let pairs: Vec<(String, String)> = url.query_pairs().into_owned().collect();
        let mut pairs_mut = url.query_pairs_mut();
        pairs_mut.clear();
        for (k, v) in pairs.iter() {
            if (k == "k") {
                pairs_mut.append_pair(&k, &v);
            }
        }
    }
    match url.path().rfind("ref=") {
        Some(idx) => {
            let s = &url.path()[..idx].to_owned();
            url.set_path(s)
        }
        None => ()
    }
    StripResult::Striped(url)
}

pub fn utm(url: &Url) -> StripResult {
    let mut url = url.clone();
    {
        let pairs: Vec<(String, String)> = url.query_pairs().into_owned().collect();
        let mut pairs_mut = url.query_pairs_mut();
        pairs_mut.clear();
        for (k, v) in pairs.iter() {
            if (k != "utm_content") && (k != "utm_term") && (k != "utm_campaign") && (k != "utm_medium") && (k != "utm_source") {
                pairs_mut.append_pair(&k, &v);
            }
        }
    }
    StripResult::Striped(url)
}

// UTM is striped seperatly
pub fn reddit(url: &Url) -> StripResult {
    println!("{:?}", url.host());
    if url.host() != Some(url::Host::Domain("www.reddit.com")) {
        return StripResult::NoMatch
    }
    let mut url = url.clone();
    {
        let pairs: Vec<(String, String)> = url.query_pairs().into_owned().collect();
        let mut pairs_mut = url.query_pairs_mut();
        pairs_mut.clear();
        for (k, v) in pairs.iter() {
            if (k != "context") {
                pairs_mut.append_pair(&k, &v);
            }
        }
    }
    StripResult::Striped(url)
}

#[test]
fn test_amazon_product_listing() {
    use crate::strip_tracking;
    assert_eq!(
        strip_tracking("https://www.amazon.com/How-Linux-Works-Brian-Ward-ebook-dp-B07X7S1JMB/dp/B07X7S1JMB/ref=mt_other?_encoding=UTF8&me=&qid=1648149738#customerReviews").unwrap(), 
        "https://www.amazon.com/How-Linux-Works-Brian-Ward-ebook-dp-B07X7S1JMB/dp/B07X7S1JMB/?#customerReviews".to_owned()
    )
}

#[test]
fn test_amazon_search() {
    use crate::strip_tracking;
    assert_eq!(
        strip_tracking("https://www.amazon.com/s?k=xkcd&crid=1U5URLUEA9LU3&sprefix=xkcd%2Caps%2C148&ref=nb_sb_noss_1").unwrap(), 
        "https://www.amazon.com/s?k=xkcd".to_owned()
    )
}

#[test]
fn test_reddit_utm() {
    use crate::strip_tracking;
    assert_eq!(
        strip_tracking("https://www.reddit.com/r/privacy/comments/tn5pjw/is_there_education_on_the_topic_of_combating_the/?utm_source=share&utm_medium=web2x&context=3").unwrap(), 
        "https://www.reddit.com/r/privacy/comments/tn5pjw/is_there_education_on_the_topic_of_combating_the/?".to_owned()
    )
}
