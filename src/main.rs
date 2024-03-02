use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
// use tracing::{error, info};
use tracing::info;
use regex::Regex;

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // if msg.content == "!hello" {
        //     if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
        //         error!("Error sending message: {:?}", e);
        //     }
        // }
        // 発言者がBotの場合はreturn
        if msg.author.bot {
            return;
        }
        // メッセージにtwitterのリンクが含まれていた場合にvxtwitterにしてリプライする
        // let Some((username, hash)) = match_url(&msg.content) else {
        //     return;
        // };
        // let reply = format!("https://vxtwitter.com/{}/status/{}\n", username, hash);
        // msg.reply(&ctx.http, reply).await.unwrap();
        if let Some((username, hash)) = match_url(&msg.content) {
            let reply = format!("https://vxtwitter.com/{}/status/{}\n", username, hash);
            msg.reply(&ctx.http, reply).await.unwrap();
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

fn match_url(content: &str) -> Option<(String, String)> {
    let regex = Regex::new(
        r"https://(x|twitter).com/(?<username>[a-zA-Z0-9_]{1,16})/status/(?<hash>[0-9]+)",
    )
    .unwrap();

    regex
        .captures(content)
        .map(|caps| (caps["username"].to_string(), caps["hash"].to_string()))
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in `Secrets.toml`
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client.into())
}
