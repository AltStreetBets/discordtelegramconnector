# Discord<>Telegram Connector
A bot to bridge Discord and Telegram.

*It forwards messages from Discord to Telegram, and from Telegram to Discord*



# Set up

**Setting up toolchain**
1. Start with installing the [Rust toolchain](https://www.rust-lang.org/tools/install).
2. Open this repository in your favourite editor

**Setting up Discord bot**

3. Go to the [Discord Developer page](https://discord.com/developers/applications).
4. Open the application page and click OAuth2
5. Click URL Generator and then select "bot"
6. This opens a dropdown menu where you select:
- Send Messages
- Read Messages/View Channels
- Manage Messages
7. Copy paste the URL into your browser and invite the bot to your server.
8. Go back to the Discord Developer page and now select OAuth2 > General
9. Copy the Client Secret and paste it in the .env file next to "DISCORD_TOKEN"
10. Turn on developer mode in Discord
11. Right click the bot account in Discord and click "Copy ID" > Replace "id" in line 28 in main.rs with this u64 string
12. "Copy ID" of the channel in Discord you want to bridge > Replace "id" in line 30 in main.rs with this ChannelID
13. Repeat 12, but for the channel you want to use for Error handling, replace "id" in line 34 in main.rs

**Setting up Telegram bot**

14. Go to https://t.me/BotFather and create a new bot
15. Copy the private key into the .env behind "TELEGRAM_TOKEN"
16. Go to https://api.telegram.org/bot<YOUR_TELEGRAM_TOKEN>/getUpdates and then let the bot join your telegram group. Copy the chat id visible and replace "id" in line 24 in main.rs
17. Go back to @BotFather on Telegram:

-> /mybots

-> Yourbot

-> Bot Settings

-> Group Privacy

-> Turn off (otherwise you can't read group chats)

**Run the bot**

18. Type in the terminal ```cargo run```

When you see a message "bot_name is connected!" the Discord bot is live. Almost instantly afterwards you should see a message in the Error channel on Discord.


# (Cross)-Compiling

There are two options:

- Run the bot on your host device, you can keep on using ```cargo run``` or you can use ```cargo build --release``` which will create an application file in /target/release which you can open like any application to start the bot.
- Run the bot on a server / external device (a Raspberry Pi Zero W is enough, and will save you a lot on your electricity bill; you can also use a free hosting service)

If you go for the second option, it is the easiest to cross compile your rust code.
This is achieved by using [cross-rs](https://github.com/cross-rs/cross).


Prepare Rust for cross compiling

1. Install [Docker](https://www.docker.com/).

2. ```cargo install cross```

and, done!

Now just do ```cross build --target arm-unknown-linux-gnueabihf```, the last part should be the compilation target of your choice.

If the file won't compile, remove the first # in the cargo.toml at line 14.


Use ```scp C:\Users\yourname\yourfilepathtotheproject\application hostdevice@192.ipaddress:/home/user/``` to transfer the file to your host device if it is on the same network. Change the paths accordingly.


# Enjoy your bot!

If you end up enjoy the bot, pleae let me know:).


# TO DO:

- Flag for "ready", right now you can possibly have a leak due to ready spawning multiple times. Small chance, but possible.
- Discord image link forwarding so that you can forward image from discord to telegram. Other way around does not work.

