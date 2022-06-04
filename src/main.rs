mod client;

use serenity::client::{Client, Context, EventHandler};
use serenity::framework::standard::{
    macros::{command, group},
    CommandResult, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::model::permissions::Permissions;
use serenity::{async_trait, model::id::UserId};

use std::env;

#[group]
#[commands(ban, help, kick, mute, poll, unban, unmute)]

struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    dotenv::dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the enviroment");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(e) = client.start().await {
        println!("Error: {:?}", e);
    }
}

#[command]
#[only_in(guilds)]
async fn ban(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.is_empty() {
        let message = msg
            .channel_id
            .send_message(&ctx, |m| m.content("No user to ban provided."))
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }

        return Ok(());
    }

    // member to ban
    let member = &msg.mentions[0];

    // guild the message is sent in
    let guild = msg.guild_id.unwrap();

    if client::check_perm(
        ctx,
        &msg.member(&ctx.http).await.unwrap(),
        Permissions::BAN_MEMBERS,
    )
        .await
        && client::is_role_higher(
        ctx,
        &msg.member(&ctx.http).await.unwrap(),
        &guild.member(&ctx.http, member).await.unwrap(),
    )
        .await
    {
        // ban the member from the guild
        if let Ok(()) = guild.ban(&ctx, member, 0).await {
            println!("Successfully banned member");
        }
        let message = msg.channel_id.say(&ctx, "User banned").await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    } else {
        let message = msg
            .channel_id
            .say(&ctx, "You are missing permissions to ban members.")
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn kick(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.mentions.is_empty() {
        let message = msg
            .channel_id
            .send_message(&ctx, |m| m.content("No uset to kick provided."))
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }

        return Ok(());
    }

    // member to ban
    let member = &msg.mentions[0];

    // guild the message is sent in
    let guild = msg.guild_id.unwrap();

    if client::check_perm(
        ctx,
        &msg.member(&ctx.http).await.unwrap(),
        Permissions::KICK_MEMBERS,
    )
        .await
    {
        if client::is_role_higher(
            ctx,
            &msg.member(&ctx.http).await.unwrap(),
            &guild.member(&ctx.http, member).await.unwrap(),
        )
            .await
        {
            if let Ok(()) = guild.kick(ctx, member).await {
                println!("Successfully kicked member");
            }
            let message = msg.channel_id.say(&ctx, "User kicked").await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }
        } else {
            let message = msg
                .channel_id
                .say(
                    &ctx,
                    "The User you are trying to kick has a higher or equal role",
                )
                .await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }
        }
    } else {
        let message = msg
            .channel_id
            .say(&ctx, "You are missing permissions to kick member.")
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
    if msg
        .member(&ctx.http)
        .await
        .unwrap()
        .roles(&ctx.cache)
        .await
        .unwrap()
        .iter()
        .any(|r| r.permissions.manage_roles())
    {
        if msg.mentions.is_empty() {
            let message = msg
                .channel_id
                .send_message(&ctx, |m| m.content("No user to mute provided."))
                .await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }

            return Ok(());
        }

        let user_id = &msg.mentions[0];

        // Get the guild the member is in
        if let Some(guild) = msg.guild_id.unwrap().to_guild_cached(&ctx).await {
            // Get the muted role by its name
            if let Some(role) = guild.role_by_name("Muted") {
                // Assign the mute role to the member
                let mute = guild
                    .member(&ctx, user_id)
                    .await
                    .unwrap()
                    .add_role(&ctx, role.id)
                    .await;
                if let Err(e) = mute {
                    println!("Error muting member: {}", e);
                }

                let message = msg.channel_id.say(&ctx, "User muted").await;
                if let Err(e) = message {
                    println!("Error sending message {}", e);
                }
            } else {
                let message = msg
                    .channel_id
                    .send_message(&ctx, |m| {
                        m.content("Mute role not found, please create a \"Muted\" role")
                    })
                    .await;
                if let Err(e) = message {
                    println!("Error sending message: {}", e);
                }
            }
        }
    } else {
        let message = msg
            .channel_id
            .say(ctx, "You are missing permissions to mute memeber.")
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn unban(ctx: &Context, msg: &Message) -> CommandResult {
    if msg
        .member(&ctx.http)
        .await
        .unwrap()
        .roles(&ctx.cache)
        .await
        .unwrap()
        .iter()
        .any(|r| r.permissions.ban_members())
    {
        if msg.mentions.is_empty() {
            let message = msg
                .channel_id
                .send_message(&ctx, |m| m.content("No user to unban provided."))
                .await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }

            return Ok(());
        }

        let user_id = msg.content[2..].split_once(" ").unwrap();
        let user_id = UserId(user_id.1.parse::<u64>().unwrap());
        let guild = msg.guild_id.unwrap();

        if let Ok(()) = guild.unban(&ctx, user_id).await {
            println!("Successfully unbanned member");
        }
        let message = msg.channel_id.say(&ctx, "User unbanned").await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    } else {
        let message = msg
            .channel_id
            .say(&ctx, "You dont have permissions to unban members")
            .await;
        if let Err(e) = message {
            println!("Error sending message: {}", e);
        }
    }
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
    if msg
        .member(&ctx.http)
        .await
        .unwrap()
        .roles(&ctx.cache)
        .await
        .unwrap()
        .iter()
        .any(|r| r.permissions.manage_roles())
    {
        if msg.mentions.is_empty() {
            let message = msg
                .channel_id
                .send_message(&ctx, |m| m.content("No user to unmute provided."))
                .await;
            if let Err(e) = message {
                println!("Error sending message: {}", e);
            }

            return Ok(());
        }

        let user_id = &msg.mentions[0];
        let guild = msg.guild_id.unwrap();

        // See if the member has the muted role
        if guild
            .member(&ctx, user_id)
            .await
            .unwrap()
            .roles(&ctx)
            .await
            .unwrap()
            .iter()
            .any(|r| matches!(r.name.clone().as_str(), "Muted"))
        {
            // Get guild the member is in
            if let Some(guild) = msg.guild_id.unwrap().to_guild_cached(&ctx).await {
                // Get the muted role by its name
                if let Some(role) = guild.role_by_name("Muted") {
                    // Remove the muted role from the member
                    let unmute = guild
                        .member(&ctx, user_id)
                        .await
                        .unwrap()
                        .remove_role(&ctx, role.id)
                        .await;
                    if let Err(e) = unmute {
                        println!("Error muting member: {}", e);
                    }

                    let message = msg.channel_id.say(&ctx, "User unmuted").await;
                    if let Err(e) = message {
                        println!("Error sending message {}", e);
                    }
                }
            }
        } else {
            let message = msg
                .channel_id
                .say(ctx, "The user you are trying to unmute is not muted")
                .await;
            if let Err(e) = message {
                println!("Error sending message {}", e);
            }
        }
    } else {
        let message = msg
            .channel_id
            .say(ctx, "You are missing permissions to unmute member.")
            .await;
        if let Err(e) = message {
            println!("Error sending message {}", e);
        }
    }
    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let message = msg.channel_id.send_message(&ctx, |m|{
        m.embed(|e|{
            e.title("Commands")
                .description("All available commands:")
                .field("!ban", "Bans the mentioned user.", false)
                .field("!unban", "Unbans a user by their user ID.", false)
                .field("!kick", "Kicks the mentioned user.", false)
                .field("!mute", "Mutes a member.", false)
                .field("!unmute", "Unmutes a member.", false)
                .field("!poll", "Starts a poll.", false)
                .field("!help", "Shows this help.", false)
        })
    }).await;

    if let Err(e) = message {
        println!("Error sending message: {}", e);
    }

    Ok(())
}

#[command]
async fn poll(ctx: &Context, msg: &Message) -> CommandResult {
    if msg.content == "!poll" {
        let empty_message = msg.channel_id.say(ctx, "No topic for the poll.").await;

        if let Err(e) = empty_message {
            println!("Error sending message: {}", e);
        }
    } else {
        let args = msg.content[2..].split_once(" ").unwrap();
        let topic = format!("Poll by {}:", msg.author.name);
        let embed = msg
            .channel_id
            .send_message(&ctx, |m| {
                m.embed(|e| {
                    e.title(topic).description(args.1).footer(|f| {
                        f.icon_url(
                            "https://cdn.discordapp.com/avatars/941197837885112330/927d4e9fbc4a4b1ee13ee2962a4b116c.png?size=256",
                        )
                            .text("Made by zinedin")
                    })
                })
            })
            .await;

        let poll = embed.unwrap();
        poll.react(&ctx, '✅').await.unwrap();
        poll.react(&ctx, '❌').await.unwrap();
    }

    Ok(())
}