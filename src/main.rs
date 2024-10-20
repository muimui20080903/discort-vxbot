use anyhow::anyhow;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
// use shuttle_secrets::SecretStore;
// use tracing::{error, info};
use regex::Regex;
use tracing::info;

struct Bot;

#[async_trait]
impl EventHandler for Bot {

    // messageが送られてきた際の処理
    async fn message(&self, ctx: Context, msg: Message) {

        // 発言者がBotの場合はreturn
        if msg.author.bot {
            return;
        }

        // メッセージにtwitterのリンクが含まれていた場合に"https://vxtwitter.com/"にしてリプライする
        // twitterのリンクか判別し、同時にユーザー名とツイートIDも取得する
        if let Some((username, hash)) = match_url(&msg.content) {

            // メッセージに画像が埋め込まれていた場合はreturn
            // if !msg.embeds.is_empty() && msg.embeds[0].image.is_some() {
            //     return;
            // };

            // リプライメッセージを作成
            let reply = format!("https://vxtwitter.com/{}/status/{}\n", username, hash);

            // リプライメッセージを送信
            msg.reply(&ctx.http, reply)
                .await
                .expect("Error sending message");
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

// twitterのリンクを含むメッセージを受け取り、ユーザー名とハッシュを取り出す
fn match_url(content: &str) -> Option<(String, String)> {

    // 正規表現を使ってユーザー名とハッシュを取り出す
    let regex = Regex::new(
        r"https://(x|twitter).com/(?<username>[a-zA-Z0-9_]{1,16})/status/(?<hash>[0-9]+)",
    )
    .expect("Failed to create regex");

    // 正規表現にマッチした場合はユーザー名とハッシュを返す
    regex
        .captures(content)
        .map(|caps| (caps["username"].to_string(), caps["hash"].to_string()))
}

#[shuttle_runtime::main]
async fn serenity(
    #[shuttle_runtime::Secrets] secret_store: shuttle_runtime::SecretStore,
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
