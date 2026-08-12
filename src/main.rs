use dotenvy::dotenv;
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

use sqlx::Pool;
use sqlx::PgPool;
use sqlx::Postgres;
use sqlx::FromRow;
use sqlx::Transaction;

use std::env;


#[derive(Debug, FromRow)]
struct CoinTransaction {
    transaction_id: i64,
    user_id: String,
    amount: i64,
    date: i64,
}

struct Handler {
    pool: sqlx::PgPool,
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        let msg_components: Vec<&str> = msg.content.as_str().split(' ').collect::<Vec<&str>>();
        println!("Message parts: {:?}", msg_components);

        match msg_components[0] {
            "!ping" => {
                if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                    println!("Error sending message: {why:?}");
                }
            }
            "!transaction_lookup" => {
                match get_transaction(&self.pool, msg_components[1].parse::<i64>().unwrap()).await {
                    Ok(Some(transaction)) => {
                       let _ = msg.channel_id
                            .say(&ctx.http, transaction.user_id)
                            .await;
                    }
                    Ok(None) => {
                        let _ = msg.channel_id
                            .say(&ctx.http, "Transaction not found.")
                            .await;
                    }
                    Err(e) => {
                        println!("Database error: {e:?}");
                    }
                };

            }
            _ => {}
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn bot_setup(pool: Pool<Postgres>) {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let handler = Handler {
        pool: pool.clone(),
    };

    let mut client =
        Client::builder(&token, intents)
        .event_handler(handler)
        .await.expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}

async fn db_setup() -> Result<Pool<Postgres>, sqlx::Error> {
    let database_url: String = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    let pool: Pool<Postgres> = PgPool::connect(&database_url).await?;

    println!("Connected to PostgreSQL");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await?;

    println!("Database migrations complete");

    Ok(pool)
}

pub async fn add_transaction(
    pool: &PgPool,
    transaction_id: i64,
    user_id: &str,
    amount: i64,
    date: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO coin_transactions
        (transaction_id, user_id, amount, date)
        VALUES ($1, $2, $3, $4)"
    )
    .bind(transaction_id)
    .bind(user_id)
    .bind(amount)
    .bind(date)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_transaction(
    pool: &PgPool,
    transaction_id: i64,
) -> Result<Option<CoinTransaction>, sqlx::Error> {
    let transaction = sqlx::query_as::<_, CoinTransaction>(
        r#"
        SELECT transaction_id, user_id, amount, date
        FROM coin_transactions
        WHERE transaction_id = $1
        "#
    )
    .bind(transaction_id)
    .fetch_optional(pool)
    .await?;

    Ok(transaction)
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    dotenvy::dotenv().ok();

    println!("Database setting up");
    let pool = db_setup().await?;
    println!("Database ready\n");

    add_transaction(
        &pool,
        12345,
        "123456789",
        100,
        1786574235,
    ).await?;
        
    if let Some(transaction) = get_transaction(&pool, 12345).await? {
        println!("Transaction: {:?}", transaction);
        println!("User: {}", transaction.user_id);
        println!("Amount: {}", transaction.amount);
    } else {
        println!("Transaction not found!");
    }

    bot_setup(pool).await;

    Ok(())
}