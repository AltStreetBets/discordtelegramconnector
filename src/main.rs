#[macro_use]
// a total mess of crates lol
//.env variables
extern crate dotenv_codegen;

//time crate to check ping
use chrono::Utc;

//discord crate
use serenity::async_trait;
use serenity::model::channel::Message as Berichtje; //Brichtje is Message in Dutch; Telegram crate uses Message and Discord crate uses Message, so if you have both as "Message" it causes trouble.
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::id::ChannelId;

//telegram crate
use teloxide::prelude::*;

//error handling
use std::error::Error;

//The Telegram chat that the bot uses, you get this by going to https://api.telegram.org/bot<YourBOTToken>/getUpdates 
//*and then* adding the bot to your telegram group. DM = positive number, Group Chat = negative number. On the site it will show you the chat id.
const CHATID1 :ChatId = ChatId(-id);
const CHATID2 :ChatId = ChatId(-id);
//Telegram bot token, you can get this by messaging https://t.me/BotFather on Telegram
const TELEGRAM_TOKEN :&str= dotenv!("TELEGRAM_TOKEN");
//The User ID of your Discord bot. Turn on developer mode in Discord, and then right click the bot > Copy ID
const BOT_ID :u64 = id;
//The Discord channel that the bot uses, right click a channel > Copy ID
const RECEIVING_CHANNEL_1 :ChannelId = ChannelId(id);
const RECEIVING_CHANNEL_2 :ChannelId = ChannelId(id);
//Discord bot token, set this up at the Discord developer portal
const DISCORD_TOKEN :&str = dotenv!("DISCORD_TOKEN");
//Discord is used for Error messages
const DISCORD_CHANNEL_ERROR :ChannelId = ChannelId(id);
//Discord bot code starts here
struct Handler1;

//Discord message handler
#[async_trait]
impl EventHandler for Handler1 {
    
    async fn message(&self, ctx: Context, msg: Berichtje) {
        //very shitty ping but it works to check if the bot is still working and if there is lag, ping check only works in error channel
        if msg.content == "!ping" && msg.channel_id == DISCORD_CHANNEL_ERROR {
                let utc = Utc::now();
                let timestamping = msg.timestamp;
                let diff = utc - timestamping;
                //let difference = format!("Ping lag in Seconds: {}", diff);
                if let Err(why) = DISCORD_CHANNEL_ERROR.say(&ctx.http, format!("Ping lag in Seconds: {}", diff)).await {
                    println!("Error {:?}", why); //Printing to terminal only works when terminal is not detached.
                }
        //Receiving messages in specified Discord channel and excluding the bot's own messages to avoid recurrence
        } else if msg.channel_id == RECEIVING_CHANNEL_1 && msg.author.id != BOT_ID {
            if let Some(discordname) = msg.author_nick(&ctx.http).await {
                
            //Forwarding messages to Telegram by calling function
                if let Err(_why) = telegramforward(msg.content, discordname.clone()).await {
                    if let Err(why2) = telegramforward("(unsupported message)".to_string(), discordname).await {
                        if let Err(_why3) = DISCORD_CHANNEL_ERROR.say(&ctx.http, format!("Error line 74 \n {:?}", why2)).await{
                            //not error handling this deep but if you want to you can add a println!("{:?}, why3"); here, note that if you detach the terminal this might become funky.
                        }
                    }
                }
            } else { 
                if let Err(_why) = telegramforward2(msg.content, msg.author.name.clone()).await {
                    if let Err(why2) = telegramforward2("(unsupported message)".to_string(), msg.author.name).await {
                        if let Err(_why3) = DISCORD_CHANNEL_ERROR.say(&ctx.http, format!("Error line 74 \n {:?}", why2)).await{
                            //not error handling this deep but if you want to you can add a println!("{:?}, why3"); here, note that if you detach the terminal this might become funky.
                        }
                    }
                }
            }                            
        } else if msg.channel_id == RECEIVING_CHANNEL_2 && msg.author.id != BOT_ID {
            if let Some(discordname) = msg.author_nick(&ctx.http).await {
                
            //Forwarding messages to Telegram by calling function
                if let Err(_why) = telegramforwardofficial(msg.content, discordname.clone()).await {
                    if let Err(why2) = telegramforwardofficial("(unsupported message)".to_string(), discordname).await {
                        if let Err(_why3) = DISCORD_CHANNEL_ERROR.say(&ctx.http, format!("Error line 74 \n {:?}", why2)).await{
                            //not error handling this deep but if you want to you can add a println!("{:?}, why3"); here, note that if you detach the terminal this might become funky.
                        }
                    }
                }
            } else { 
                if let Err(_why) = telegramforwardofficial2(msg.content, msg.author.name.clone()).await {
                    if let Err(why2) = telegramforwardofficial2("(unsupported message)".to_string(), msg.author.name).await {
                        if let Err(_why3) = DISCORD_CHANNEL_ERROR.say(&ctx.http, format!("Error line 74 \n {:?}", why2)).await{
                            //not error handling this deep but if you want to you can add a println!("{:?}, why3"); here, note that if you detach the terminal this might become funky.
                        }
                    }
                }
            }                            
        }
        
        

    }
    //Discord bot connection checker
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        if let Err(why3) = DISCORD_CHANNEL_ERROR.say(&ctx.http, format!("initializing telegrambot",)).await{
            println!("{:?}", why3);
        }
        
        //Turn on telegram bot
        let bot = Bot::new(TELEGRAM_TOKEN).auto_send();
        let handler = dptree::entry()
            .branch(Update::filter_message().endpoint(move |m, b| message_handler(m, b, ctx.clone())));
                
        Dispatcher::builder(bot, handler)
        .default_handler(|_| async {})
        .build()
        .dispatch().await;
        println!("line 91");

        //Collecting messages from telegram
        async fn message_handler(
            m: Message,
            bot: AutoSend<Bot>,
            ctx5: Context             
        ) -> Result<(), Box<dyn Error + Send + Sync>> {
            let chat = &m.chat;
            if let Some(contenttelegram) = m.text() {
                if contenttelegram == "/start" {
                    bot.send_message(chat.id, "Bot is working").await?;
                }
                else if chat.id == CHATID1 {
                    
                    if let Some(username) = m.from() {
                        if let Err(why3) = RECEIVING_CHANNEL_1.say(&ctx5.http, format!("**{:?}:** {}",username.first_name, contenttelegram)).await{
                            if let Err(_why4) = DISCORD_CHANNEL_ERROR.say(&ctx5.http, format!("Error line 74 \n {:?}", why3)).await{
                                //not error handling this deep but if you want to you can add a println!("{:?}, why3"); here, note that if you detach the terminal this might become funky.
                            }
                        }
                    }                                    
                } else if chat.id == CHATID2 {
                    
                    if let Some(username) = m.from() {
                        if let Err(why3) = RECEIVING_CHANNEL_2.say(&ctx5.http, format!("**{:?}:** {}",username.first_name, contenttelegram)).await{
                            if let Err(_why4) = DISCORD_CHANNEL_ERROR.say(&ctx5.http, format!("Error line 74 \n {:?}", why3)).await{
                                //not error handling this deep but if you want to you can add a println!("{:?}, why3"); here, note that if you detach the terminal this might become funky.
                            }
                        }
                    }                                    
                }
            }
            Ok(())        
        }

    }
    
}

//Discord Bot Deployer
#[tokio::main]
async fn main() {
    let mut client = Client::builder(&DISCORD_TOKEN).event_handler(Handler1).await.expect("Err creating client");
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

//Forwards Discord messages to Telegram
async fn telegramforward(content :String, discordname :String)-> Result<(), Box<dyn Error + Send + Sync>> {    
    let bot = Bot::new(TELEGRAM_TOKEN).auto_send();    
    bot.send_message(CHATID1, format!("{}: {}", discordname, content)).await?;
    Ok(())
}

async fn telegramforward2(content :String, discordname :String)-> Result<(), Box<dyn Error + Send + Sync>> {    
    let bot = Bot::new(TELEGRAM_TOKEN).auto_send();    
    bot.send_message(CHATID1, format!("{}: {}", discordname, content)).await?;
    Ok(())
}

//Forwards Discord messages to Telegram official
async fn telegramforwardofficial(content :String, discordname :String)-> Result<(), Box<dyn Error + Send + Sync>> {    
    let bot = Bot::new(TELEGRAM_TOKEN).auto_send();    
    bot.send_message(CHATID2, format!("{}: {}", discordname, content)).await?;
    Ok(())
}

async fn telegramforwardofficial2(content :String, discordname :String)-> Result<(), Box<dyn Error + Send + Sync>> {    
    let bot = Bot::new(TELEGRAM_TOKEN).auto_send();    
    bot.send_message(CHATID2, format!("{}: {}", discordname, content)).await?;
    Ok(())
}
