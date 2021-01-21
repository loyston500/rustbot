use std::collections::{HashMap, HashSet};

use serenity::client::Context;
use serenity::model::channel::Message;
//use serenity::model::channel::Embed;
//use serenity::model::channel::EmbedAuthor;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};


// my libs.
use crate::utils::argparser::argparse;

#[command]
pub async fn tell(ctx: &Context, msg: &Message, args: Args) -> CommandResult{
    let cont = if args.is_empty() {
        "pass something to tell".to_string()
    } else {
        format!("{}", args.rest())
    };
    msg.channel_id.say(&ctx.http, cont).await?;
    Ok(())
}

#[command]
pub async fn say(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult{
    let times = args.single::<u16>()?;
    let mut word: String = args.single::<String>()?;

    if word.starts_with("\"") && word.ends_with("\""){
        word = word[1..(word.len() - 1)].to_string();
    }

    let repeate = word.repeat(times.into());
    msg.channel_id.say(&ctx.http, repeate).await?;
    Ok(())

}

#[command]
// Lets us also call `~math *` instead of just `~math multiply`.
#[aliases("*")]
pub async fn multiply(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let first = args.single::<f64>()?;
    let second = args.single::<f64>()?;

    let res = first * second;

    msg.channel_id.say(&ctx.http, &res.to_string()).await?;

    Ok(())

}

#[command]
pub async fn wait(ctx: &Context, msg: &Message) -> CommandResult {
    //let wait_time: u8 = args.single::<u8>()?;
    //sleep(Duration::from_millis(wait_time.into())).await;
    let mut message = msg.channel_id.say(&ctx.http, "pog").await?;
    message.edit(&ctx, |m| m.content("new content")).await?;
    Ok(())
}

#[command]
pub async fn emb(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let content: &str = args.rest();
    let (params, mut inputs, _flags): (HashMap<String, String>,  Vec<String>, HashSet<String>) = argparse(content.to_string()).unwrap(); // parses the string
    
    let inputs_len = inputs.len();
    if inputs_len % 2 != 0 {
        msg.channel_id.say(&ctx.http, "the inputs should be even.").await?;
        return Ok(())
    }
    
    // title, description and some other shit.
    let title = match params.get("t"){
        Some(this) => this.as_str(),
        None => ""
    };
    
    let desc = match params.get("d"){
        Some(this) => this.as_str(),
        None => ""
    };
    
    let author = match params.get("a"){
        Some(this) => this.as_str(),
        None => ""
    };
    
    let footer = match params.get("f"){
        Some(this) => this.as_str(),
        None => ""
    };
    
    
    
    msg.channel_id.send_message(&ctx.http, |m|{
        m.embed(|e| {
            e.title(title);
            e.description(desc);
            match params.get("i"){
                Some(this) => {
                    e.image(this.as_str());
                }
                None => {}
            };
            
            // set author.
            e.author(|a|{
                a.name(author);
                a
            });
            
            // loop to set all the fields.
            for _ in 0..(inputs_len/2) {
                e.field(inputs.remove(0).as_str(), inputs.remove(0).as_str(), false);
            };
            
            // set the footer.
            e.footer(|f|{
                f.text(footer);
                f
            });
            
            e
        });
        m
    }).await?;
    Ok(())
}


#[command]
pub async fn parse(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    let content: &str = args.rest();

    let parse_result = argparse(content.to_string()).unwrap();

    let params: HashMap<String, String>  = parse_result.0;
    let inputs: Vec<String> = parse_result.1;
    let flags: HashSet<String> = parse_result.2;

    let result_string: String = format!("{:?}, {:?}, {:?}", params, inputs, flags);
    msg.channel_id.say(&ctx.http, result_string).await?;

    Ok(())
}
