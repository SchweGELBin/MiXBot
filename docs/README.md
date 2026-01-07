# MiXBot
[![GitHub Downloads (all assets, all releases)](https://img.shields.io/github/downloads/SchweGELBin/MiXBot/total)](https://github.com/SchweGELBin/MiXBot/releases)
[![GitHub License](https://img.shields.io/github/license/SchweGELBin/MiXBot)](../LICENSE)
[![GitHub Release](https://img.shields.io/github/v/release/SchweGELBin/MiXBot)](https://github.com/SchweGELBin/MiXBot/releases/latest)

- Minecraft MiXBot
- Available in my [nur expressions](https://github.com/SchweGELBin/nur-expressions) repo
- **This bot is very experimental and should not be used in the current state**

## Environment
The Bot needs following environment variables to function:
| Variable | Description | Default |
| -------- | ----------- | ------- |
| MIXBOT_DISCORD_ID |  	Your Discord User ID | - |
| MIXBOT_DISCORD_PREFIX | Your Discord Bot Command Prefix | ! |
| MIXBOT_DISCORD_TOKEN | Your Discord Bot Token | - |
| MIXBOT_HOST | Your Server IP | localhost |
| MIXBOT_NAME | Your Bot's Name | MiXBot |
| MIXBOT_ONLINE | Authenticate with Microsoft | false |

## Commands
- "[arg]" -> Necessary Argument
- "\<arg\>" -> Optional Argument

### Discord
| Command | Arguments | Description |
| ------- | --------- | ----------- |
| fight | [player] | Fight a player |
| goto | [x] \<y\> [z] | Goto coordinates |
| guard | \<mob\> | Attack (specific) nearby mobs |
| help | \<command\> | Show command information |
| join | - | Join the Server |
| leave | - | Leave the Server |
| msg | \<name\> | Send Message to chat/user |
| ping | - | Reply with "Pong!" |
| status | - | Give Player List, TPS, Coordinates |
| version | - | Give the current Bot version |

### Minecraft
| Command | Arguments | Description |
| ------- | --------- | ----------- |
| follow | \<name\> | Follow a player |
| stop | - | Stop the current action |
| tp | - | Close the nearest trapdoor |

### Extra
Minecraft: "/msg [bot] [msg]" -> Bridge to Discord
