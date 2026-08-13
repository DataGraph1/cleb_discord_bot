# CLEB Discord Bot
(the best discord bot ever)
(For now at least) the main scope is to make currency to be used in a server, the currency can be used to do things within the server like deleting messages, adding channels & timing out other users.

### Development
##### Setup
1. Clone the repo: `git clone https://github.com/DataGraph1/cleb_discord_bot.git`
2. Download and setup [docker](https://www.docker.com/products/docker-desktop/)
3. Make a copy of `.env.example` called `.env`, replacing `<discord_bot_token>` and `<postgres_password>` (both in the password and URL variables) with their correct values
4. Run `docker compose up --build` while in the project directory to build the rust file and startup the project
5. Run `docker compose down -v` to stop the project (and clear the database)

##### Structure
- `main.rs` contains the main code
  - `async fn message() {}` is where code for commands go
- `migrations` holds the default setups for database tables
  - Each table must start with an integer
- `Dockerfile` defines the discord bot's docker container setup
- `docker-compose.yml` defines the stack of all containers and how they connect

### Thats all folks
