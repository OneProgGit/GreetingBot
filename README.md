# Greeting Bot (aka MeowBot)
[Версия на русском](README-RU.md)

Greeting Bot is a Telegram bot, which sends a message to all users in database with AI-generated text.
It has modular architecture (since 0.6.0), which allows you to choose database, weather, AI or platform provider or create your own.

## Greeting Bot in action
<img width="1239" height="977" alt="greeting_bot_in_action" src="https://github.com/user-attachments/assets/eebb6303-783f-4ce5-9762-26bbcbf05b1c" />

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
Create your config file and fill it like that ([about cron](https://en.wikipedia.org/wiki/Cron)):
```toml
weather_url = "" # Url to weather provider
weather_fmt = "" # Weather format in greeting message

ai_model = "" # Ai model name (example: `qwen3:30b`)
ai_prompt = "" # Ai model prompt
ai_msg_off = "" # Message which appears when cannot connect to AI provider

greeting_date_cron = "" # Datetime of greeting message in cron format
greeting_fmt = "" # Greeting message format

start_fmt = "" # Message which appears when /start command called

db_url = "" # Url to database

draw_date_cron = "" # Datetime of draw in cron format
draw_win_fmt = "" # Format of message which sends to user when he won the draw

admin = "" # Admin id
draw_results_fmt = "" # Format of message which sends to channel and admin when draw's winner has chosen

channel = "" # Channel or chat id
```
Create .env and fill these fields:
```env
TELOXIDE_TOKEN = "" # Token of your telegram bot, if the target platform is so
CONFIG_PATH = "" # Path to the config from previous step
```
Make sure Rust installed ([how to install Rust](https://rustup.rs/)):
```bash
cargo --version
rustc --version
```
Run this command:
```bash
cargo run --release
```
Now, the bot is running!
