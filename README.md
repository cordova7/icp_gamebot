# icp_gamebot

A Telegram bot for betting ICP tokens on emoji dice games. Players type a bet amount, pick a game, and win or lose based on the roll result.

## How It Works

1. Player sends a bet amount (in $ICP) to the bot in the game thread
2. The bot checks the player's balance on the ICP ledger via ICRC-1
3. Player selects a game from the inline keyboard
4. The bot sends a Telegram dice animation in the selected game
5. Win/loss is determined by the roll value, and ICP is transferred accordingly

## Games

| Game           | Emoji | Win Condition      | Odds     | Multiplier |
|----------------|-------|--------------------|----------|------------|
| Dice           | 6     | Roll 6             | 1 in 6   | 5x         |
| Darts          | 6     | Roll 6             | 1 in 6   | 5x         |
| Bowling        | 6     | Roll 6             | 1 in 6   | 5x         |
| Soccer         | 5     | Roll 5             | 1 in 5   | 4x         |
| Basketball     | 5     | Roll 5             | 1 in 5   | 4x         |
| Slot Machine   | 64    | Roll 64            | 1 in 64  | 10x        |

## Tech Stack

- **Language:** Rust
- **Telegram Bot:** teloxide 0.12
- **Async Runtime:** tokio
- **Ledger:** ICP via ICRC-1 (calls `dfx canister call --ic`)
- **Number Formatting:** numfmt

## Prerequisites

- Rust toolchain (latest stable)
- `dfx` (ICP Canister SDK) installed and configured
- A Telegram bot token (from @BotFather)
- An ICP ledger canister ID (`ryjl3-tyaaa-aaaaa-aaaba-cai` on mainnet)
- A `TREASURY` dfx identity with funds for paying out winnings

## Configuration

Open `src/main.rs` and set your values:

```rust
let bot = Bot::new("YOUR_TELEGRAM_BOT_TOKEN");
const TREASURY_PRINCIPAL: &str = "YOUR_TREASURY_WALLET_PRINCIPAL";
const TREASURY_IDENTITY: &str = "TREASURY"; // dfx identity name
const GAME_THREAD_ID: i64 = YOUR_FORUM_THREAD_ID;
const GAME_CHAT_ID: i64 = YOUR_CHAT_ID;
```

## Build and Run

```bash
cargo build --release
cargo run --release
```

The bot listens for messages in the configured chat and thread. Players interact entirely through the Telegram UI.

## Project Structure

```
icp_gamebot/
├── Cargo.toml
├── README.md
└── src/
    └── main.rs      # All bot logic: message handling, game dispatch, ICP transfers
```

## ICP Transfers

- **Bets** are transferred from the player to the treasury before the game
- **Winnings** are transferred from the treasury to the player on a win
- **Losses** stay in the treasury
- All transfers use `dfx canister call --ic ryjl3-tyaaa-aaaaa-aaaba-cai icrc1_transfer`
- Balances are checked via `icrc1_balance_of`

## License

MIT
