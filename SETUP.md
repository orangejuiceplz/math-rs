# Math-RS Setup Guide

## Prerequisites

1. **Rust Installation**: Make sure you have Rust installed
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Discord Bot Token**: Create a Discord application and bot at https://discord.com/developers/applications

## Setup Instructions

1. **Clone and navigate to the project**:
   ```bash
   cd math-rs
   ```

2. **Create environment file**:
   ```bash
   cp .env.example .env
   ```

3. **Add your Discord bot token** to `.env`:
   ```
   DISCORD_TOKEN=your_actual_bot_token_here
   ```

4. **Build the project**:
   ```bash
   cargo build
   ```

5. **Run the bot**:
   ```bash
   cargo run
   ```

## Bot Permissions

Your Discord bot needs the following permissions:
- Send Messages
- Use Slash Commands
- Read Message History
- Embed Links
- Attach Files (for future LaTeX rendering)

## Usage

1. Use `/mathbot start session` to begin a math session
2. Send math problems directly to the bot in that channel
3. Use `/mathbot info` to check session status
4. Use `/mathbot end session` to end your session

## Current Features

- ✅ Session-based math conversations
- ✅ Basic arithmetic solving
- ⚠️ Algebraic equation solving (placeholder)
- ⚠️ Calculus solving (placeholder)
- ⚠️ LaTeX rendering (placeholder)

## Development

- Main bot logic: `src/main.rs`
- Commands: `src/commands/`
- Math engine: `src/math/`
- Session management: `src/session/`
- Utilities: `src/utils/`