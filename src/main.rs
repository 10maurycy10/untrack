use url::Url;
use std::error::Error;
use url::ParseError;
use linkify::{LinkFinder, LinkKind};
use serenity::cache::{Cache, Settings};
use std::sync::Arc;

mod cleaners;

pub enum StripResult {
    NoMatch,
    Striped(Url),
    Decoded(Url) // indicates to restrip the url, usefull for decoding outgoing link tracking
}

pub fn strip_tracking(url: &str) -> Result<String, ParseError> {
    let mut url = Url::parse(url)?;
    let modules: Vec<Box<dyn Fn(&Url) -> StripResult>> = vec![
        Box::new(cleaners::amazon),
        Box::new(cleaners::utm),
        Box::new(cleaners::reddit),
    ];
    for module in modules.iter() {
        match module(&url) {
            StripResult::NoMatch => (),
            StripResult::Striped(new) => {
                url=new;
            },
            StripResult::Decoded(_) => todo!(),
        }
    }
    
    match url.query() {
        Some(x) => if x.len() == 0 {url.set_query(None)}
        None => ()
    }
    
    match url.fragment() {
        Some(x) => if x.len() == 0 {url.set_fragment(None)}
        None => ()
    }
    
    Ok(url.as_str().to_owned())
}

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

use std::env;

struct Handler {
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.is_own(Cache::new()).await {return;}
        let finder = LinkFinder::new();
        let links: Vec<_> = finder.links(&msg.content).collect();
        for link in links {
            match strip_tracking(&link.as_str()) {
                Ok(striped) => if striped != link.as_str() {
                    if let Err(why) = msg.channel_id.say(&ctx.http, format!("Cleaned link: {}", striped)).await {
                        println!("Error sending message: {:?}", why);
                    }
                },
                Err(x) => println!("Error on '{}' {:?}", link.as_str(), x)
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token).event_handler(Handler {}).await.expect("Err creating client");
    if let Err(why) = client.start_shards(2).await {
        println!("Client error: {:?}", why);
    }
}
