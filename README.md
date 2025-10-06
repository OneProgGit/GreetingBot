# MeowBot (aka Greeting Bot)

MeowBot is a Telegram bot, which sends a message to all users in database with AI-generated text.
It has microservice architecture (since 0.7.0), which allows you to choose database, weather, AI or platform provider or create your own.

## MeowBot in action

![MeowBot in action](https://github.com/user-attachments/assets/eebb6303-783f-4ce5-9762-26bbcbf05b1c)

## Getting started

Make sure git installed ([how to install git](https://git-scm.com/book/en/v2/Getting-Started-Installing-Git)):

```bash
git --version
```

Clone git repo:

```bash
git clone https://github.com/OneProgGit/GreetingBot/
```

Cd into the project folder:

```bash
cd GreetingBot
```

Create your config file in meowbot-core folder and fill it like that ([about cron](https://en.wikipedia.org/wiki/Cron)):

```toml
weather_fmt = "" # Weather format in greeting message

ai_model = "" # Ai model name
ai_prompt = "" # Ai model prompt
ai_msg_off = "" # Message which appears when cannot connect to AI provider

greeting_date_cron = "" # Datetime of greeting message in cron format
greeting_fmt = "" # Greeting message format

start_fmt = "" # Message which appears when /start command called

draw_date_cron = "" # Datetime of draw in cron format
draw_win_fmt = "" # Format of message which sends to user when he won the draw

admin = "" # Admin id
draw_results_fmt = "" # Format of message which sends to channel and admin when draw's winner has chosen

channel = "" # Channel or chat id
skip_channel = "true/false" # Skip processing channel?

changed_city_fmt = "" # Format of message which sends to user when changed city name successfully
changed_city_failed_fmt = "" # Format of message which sends to user when changed city name failed

changed_areas_of_interest_fmt = "" # Format of message which sends to user when updated areas of interest successfully
```

Create .env in the same folder and fill these fields:

```env
COMMANDS_ADDR= # Address to host commands handler server
AI_ADDR= # Ai microservice address
DB_ADDR= # Db microservice address
PLATFORM_ADDR= # Platform microservice address
WEATHER_ADDR= # Weather microservice address
CONFIG_PATH= # Path to the config from previous step
```

Create .env files in meowbot-ai, meowbot-db, meowbot-platform, meowbot-weather folders and fill AI_ADDR, DB_ADDR, PLATFORM_ADDR, WEATHER_ADDR fields respectively. Also, fill COMMANDS_ADDR and TELOXIDE_TOKEN (token of your telegram bot, if you use so) field in .env file in meowbot-platform folder and DB_URL (url of your database) in .env file in meowbot-db folder.

Make sure Rust and GNU Make installed (it should display versions, [how to install Rust](https://rustup.rs/), [how to install GNU Make](https://www.gnu.org/software/make/#download)):

```bash
cargo --version
rustc --version
make --version
```

Make sure you're in the root directory of the project:
```bash
pwd
```

Build all crates:

```bash
make build
```

Run 
```bash
make run-(ai/db/platform/weather/core)
```

for each microservice respectively to run it.
