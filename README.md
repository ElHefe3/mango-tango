# Mango Tango

## Running with Docker

### Prerequisites

- Docker installed on your machine.
- A Discord server where you can add the bot.
- A Discord bot token. You can create a bot and get its token from the [Discord Developer Portal](https://discord.com/developers/applications).
- A `.env.docker` file with your Mango Tango configuration (see below).

---

### Discord Bot Setup

1. Go to the [Discord Developer Portal](https://discord.com/developers/applications) and create a new application.
2. Navigate to the **Bot** tab and click **Add Bot**.
3. Copy the bot token and keep it safe. You will need it to configure the bot.
4. Under the **OAuth2** → **URL Generator**:
   - In **Scopes**, select:
     - `bot`
     - `applications.commands`
   - In **Bot Permissions**, at minimum select:
     - **View Channels**
     - **Send Messages**
     - **Use Slash Commands**
     - **Connect**
     - **Speak**
5. Copy the generated OAuth2 URL and use it to invite the bot to your Discord server.

---

### Creating the `.env.docker` File

Create a file named `.env.docker` in the root directory of the project with the following content:

```env
DISCORD_TOKEN=your_discord_bot_token_here
```

---

### Running the Container

Build the Docker image:

```shell
docker build -t mango-tango .
```

Run the Docker container:

```shell
docker run --rm -p 8080:8080 --env-file .env.docker mango-tango
```

---

### How to Use

1. Start Mango Tango using Docker as shown above.
2. Make sure the bot is online in your Discord server.
3. Join a voice channel in your server.
4. Use the slash commands in any text channel where the bot has access.

#### Command Table 

| Command      | Description                               |
| ------------ | ----------------------------------------- |
| `/search`    | Search YouTube and select a track to play |
| `/play_link` | Play a song from a YouTube video URL      |
| `/skip`      | Skip the currently playing song           |
