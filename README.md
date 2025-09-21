# math-rs 

say hi

# setup guide

## prereq for running everwhere

1. **rost installation**: Rust installed dumbass
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **discord bot token**: https://discord.com/developers/applications

## instructions

1. **clone + navi**:
   ```bash
   cd math-rs
   ```

2. **environment file**:
   ```bash
   cp .env.example .env
   ```

3. **add botToken to environment var** to `.env`:
   ```
   DISCORD_TOKEN=your_actual_bot_token_here
   ```

4. **build**:
   ```bash
   cargo build
   ```

5. **run the bot**:
   ```bash
   cargo run
   ```

## pernms

mathbot needs the following permissions:
- send messages
- use slash commands
- read message history
- embed links
- attach files (This is so LatEX works)

## Usage

1. use `/mathbot start session` to begin a math session
2. send math problems directly to the bot in that channel
3. use `/mathbot info` to check session status
4. use `/mathbot end session` to end your session

## features

-  session-based math conversations
-  basic arithmetic solving

## to bee added fully
-  algebraic equation solving (placeholder)
-  calculus solving (placeholder)
-  LaTeX rendering (placeholder)

## Development

- bot logic: `src/main.rs`
- commands: `src/commands/`
- engine: `src/math/`
- session management: `src/session/`
- utils: `src/utils/`