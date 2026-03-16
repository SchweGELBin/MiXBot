# MiXBot
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/SchweGELBin/MiXBot/total)](https://github.com/SchweGELBin/MiXBot/releases)
[![GitHub License](https://img.shields.io/github/license/SchweGELBin/MiXBot)](../LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/SchweGELBin/MiXBot)](https://github.com/SchweGELBin/MiXBot/releases/latest)

- Minecraft MiXBot
- Available in my [nur expressions](https://github.com/SchweGELBin/nur-expressions) repo
- **This bot is very experimental and should not be used in the current state**

## Environment
The Bot uses following environment variables or flags:
| Variable              | Flag           | Description                     | Default      |
| --------              | ----           | -----------                     | -------      |
| MIXBOT_ACCOUNTS       | -A, --accounts | Your Bot's Name + Auth Mode     | MiXBot,false |
| MIXBOT_HOST           | -H, --hosts    | Your Server IP                  | localhost    |
| MIXBOT_OWNER          | -O, --owner    | Your Bot's Owner Name           | -            |
| MIXBOT_PREFIX         | -P, --prefix   | Your Bot's Command Prefix       | !            |
| MIXBOT_DISCORD_ID     | --dcid         | Your Discord User ID            | -            |
| MIXBOT_DISCORD_PREFIX | --dcprefix     | Your Discord Bot Command Prefix | !            |
| MIXBOT_DISCORD_TOKEN  | --dctoken      | Your Discord Bot Token          | -            |

## Commands
- "[arg]" -> Necessary Argument
- "\<arg\>" -> Optional Argument

### Discord
Disabled if MIXBOT_DISCORD_TOKEN is unset or empty

| Command | Arguments     | Description                        |
| ------- | ---------     | -----------                        |
| help    | \<command\>   | Show command information           |
| join    | -             | Join the Server                    |
| leave   | -             | Leave the Server                   |
| msg     | \<name\>      | Send Message to chat/user          |
| ping    | -             | Reply with "Pong!"                 |
| status  | -             | Give Player List, TPS, Coordinates |
| version | -             | Give the current Bot version       |

### Minecraft
| Command | Arguments     | Description                   |
| ------- | ---------     | -----------                   |
| fight   | [player]      | Fight a player                |
| follow  | \<name\>      | Follow a player               |
| goto    | [x] \<y\> [z] | Goto coordinates              |
| guard   | \<mob\>       | Attack (specific) nearby mobs |
| msg     | [message]     | Send Message to discord chat  |
| stop    | -             | Stop the current action       |
| tp      | -             | Close the nearest trapdoor    |
