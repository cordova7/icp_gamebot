
use std::collections::HashMap;
use std::ops::Not;
use std::sync::Arc;
use teloxide::prelude::*;
use teloxide::types::DiceEmoji;
use teloxide::types::InlineKeyboardButton;
use teloxide::types::InlineKeyboardButtonKind::CallbackData;
use teloxide::types::InlineKeyboardMarkup;
use teloxide::types::MessageKind;
use teloxide::types::ReplyMarkup;
use teloxide::types::User;
use teloxide::RequestError;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let bot = Bot::new("7008560751:AAEtomrzqJrH-GR5b4mAkGftlRtiBLHi0Ac");
    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(analyze_games))
        .branch(Update::filter_callback_query().endpoint(callback_handler));
    let map_id_bet: Arc<Mutex<HashMap<String, f32>>> = Arc::new(Mutex::new(HashMap::new()));
    let map_game_userandbet: Arc<Mutex<HashMap<String, (User, f32)>>> =
        Arc::new(Mutex::new(HashMap::new()));

    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![map_id_bet, map_game_userandbet])
        .build()
        .dispatch()
        .await;
}

async fn callback_handler(
    bot: Bot,
    q: CallbackQuery,
    map_id_bet: Arc<Mutex<HashMap<String, f32>>>,
    map_game_userandbet: Arc<Mutex<HashMap<String, (User, f32)>>>,
) -> Result<(), RequestError> {
    let mut first_map = map_id_bet.lock().await;
    let mut second_map = map_game_userandbet.lock().await;

    let data = q.data.unwrap_or_default();
    let user = q.from;

    match data.as_str() {
        "Dice🎲" => {
            let mut glaze_mut: f32 = 0.0;
            if let Some(glaze_bet) = first_map.get_mut(&user.id.0.to_string()) {
                glaze_mut = *glaze_bet;
                second_map.insert("Dice🎲".to_string(), (user.clone(), *glaze_bet));
                first_map.remove(&user.id.0.to_string());
            }

            let rewards_multiple = 5.0;

            let result = bot
                .send_dice(ChatId(-1002028434193))
                .message_thread_id(12880)
                .emoji(DiceEmoji::Dice)
                .send()
                .await;
            match result {
                Ok(msg) => {
                    println!("\nbot dice msg: {:?}", msg);
                    match msg.kind.clone() {
                        MessageKind::Dice(_dice_message) => {
                            let game_to_delete = determine_win(
                                bot.clone(),
                                (user.clone(), glaze_mut),
                                msg,
                                rewards_multiple,
                            )
                            .await;
                            second_map.remove(&game_to_delete);
                        }
                        _ => (),
                    }
                }
                Err(_) => (),
            }
        }

        "Darts🎯" => {
            let mut glaze_mut: f32 = 0.0;
            if let Some(glaze_bet) = first_map.get_mut(&user.id.0.to_string()) {
                glaze_mut = *glaze_bet;
                second_map.insert("Darts🎯".to_string(), (user.clone(), *glaze_bet));
                first_map.remove(&user.id.0.to_string());
            }

            let rewards_multiple = 5.0;

            let result = bot
                .send_dice(ChatId(-1002028434193))
                .message_thread_id(12880)
                .emoji(DiceEmoji::Darts)
                .send()
                .await;
            match result {
                Ok(msg) => {
                    println!("\nbot dice msg: {:?}", msg);
                    match msg.kind.clone() {
                        MessageKind::Dice(_dice_message) => {
                            let game_to_delete = determine_win(
                                bot.clone(),
                                (user.clone(), glaze_mut),
                                msg,
                                rewards_multiple,
                            )
                            .await;
                            second_map.remove(&game_to_delete);
                        }
                        _ => (),
                    }
                }
                Err(_) => (),
            }
        }

        "Bowling🎳" => {
            let mut glaze_mut: f32 = 0.0;
            if let Some(glaze_bet) = first_map.get_mut(&user.id.0.to_string()) {
                glaze_mut = *glaze_bet;
                second_map.insert("Bowling🎳".to_string(), (user.clone(), *glaze_bet));
                first_map.remove(&user.id.0.to_string());
            }

            let rewards_multiple = 5.0;

            let result = bot
                .send_dice(ChatId(-1002028434193))
                .message_thread_id(12880)
                .emoji(DiceEmoji::Bowling)
                .send()
                .await;
            match result {
                Ok(msg) => {
                    println!("\nbot dice msg: {:?}", msg);
                    match msg.kind.clone() {
                        MessageKind::Dice(_dice_message) => {
                            let game_to_delete = determine_win(
                                bot.clone(),
                                (user.clone(), glaze_mut),
                                msg,
                                rewards_multiple,
                            )
                            .await;
                            second_map.remove(&game_to_delete);
                        }
                        _ => (),
                    }
                }
                Err(_) => (),
            }
        }
        
        "Soccer⚽️" => {
            let mut glaze_mut: f32 = 0.0;
            if let Some(glaze_bet) = first_map.get_mut(&user.id.0.to_string()) {
                glaze_mut = *glaze_bet;
                second_map.insert("Soccer⚽️".to_string(), (user.clone(), *glaze_bet));
                first_map.remove(&user.id.0.to_string());
            }

            let rewards_multiple = 4.0;

            let result = bot
                .send_dice(ChatId(-1002028434193))
                .message_thread_id(12880)
                .emoji(DiceEmoji::Football)
                .send()
                .await;
            match result {
                Ok(msg) => {
                    println!("\nbot dice msg: {:?}", msg);
                    match msg.kind.clone() {
                        MessageKind::Dice(_dice_message) => {
                            let game_to_delete = determine_win(
                                bot.clone(),
                                (user.clone(), glaze_mut),
                                msg,
                                rewards_multiple,
                            )
                            .await;
                            second_map.remove(&game_to_delete);
                        }
                        _ => (),
                    }
                }
                Err(_) => (),
            }
        }

        "Basketball🏀" => {
            let mut glaze_mut: f32 = 0.0;
            if let Some(glaze_bet) = first_map.get_mut(&user.id.0.to_string()) {
                glaze_mut = *glaze_bet;
                second_map.insert("Basketball🏀".to_string(), (user.clone(), *glaze_bet));
                first_map.remove(&user.id.0.to_string());
            }

            let rewards_multiple = 4.0;

            let result = bot
                .send_dice(ChatId(-1002028434193))
                .message_thread_id(12880)
                .emoji(DiceEmoji::Basketball)
                .send()
                .await;
            match result {
                Ok(msg) => {
                    println!("\nbot dice msg: {:?}", msg);
                    match msg.kind.clone() {
                        MessageKind::Dice(_dice_message) => {
                            let game_to_delete = determine_win(
                                bot.clone(),
                                (user.clone(), glaze_mut),
                                msg,
                                rewards_multiple,
                            )
                            .await;
                            second_map.remove(&game_to_delete);
                        }
                        _ => (),
                    }
                }
                Err(_) => (),
            }
        }

        "Slot Machine🎰" => {
            let mut glaze_mut: f32 = 0.0;
            if let Some(glaze_bet) = first_map.get_mut(&user.id.0.to_string()) {
                glaze_mut = *glaze_bet;
                second_map.insert("Slot Machine🎰".to_string(), (user.clone(), *glaze_bet));
                first_map.remove(&user.id.0.to_string());
            }

            let rewards_multiple = 10.0;

            let result = bot
                .send_dice(ChatId(-1002028434193))
                .message_thread_id(12880)
                .emoji(DiceEmoji::SlotMachine)
                .send()
                .await;
            match result {
                Ok(msg) => {
                    println!("\nbot dice msg: {:?}", msg);
                    match msg.kind.clone() {
                        MessageKind::Dice(_dice_message) => {
                            let game_to_delete = determine_win(
                                bot.clone(),
                                (user.clone(), glaze_mut),
                                msg,
                                rewards_multiple,
                            )
                            .await;
                            second_map.remove(&game_to_delete);
                        }
                        _ => (),
                    }
                }
                Err(_) => (),
            }
        }

        _ => (),
    }
    bot.clone().answer_callback_query(q.id).await?;

    if let Some(id) = q.inline_message_id {
        bot.clone()
            .edit_message_reply_markup_inline(id.clone())
            .reply_markup(InlineKeyboardMarkup::default())
            .send()
            .await?;
    }
    Ok(())
}

async fn analyze_games(
    bot: Bot,
    msg: Message,
    map_id_bet: Arc<Mutex<HashMap<String, f32>>>,
    //map_game_userandbet: Arc<Mutex<HashMap<String, (User, u64)>>>,
) -> Result<(), RequestError> {
    // -1002028434193 GLAZE MAIN CHAT
    // -1002030944296 GLAZE GAMES CHAT
    if msg.chat.id.ne(&ChatId(-1002028434193)) {
        return Ok(());
    }
    println!("\n\n{:?}", msg);

    let mut first_map = map_id_bet.lock().await;
    //let mut second_map = map_game_userandbet.lock().await;

    let dice = InlineKeyboardButton::new("🎲", CallbackData("Dice🎲".to_string()));
    let darts = InlineKeyboardButton::new("🎯", CallbackData("Darts🎯".to_string()));
    let bowling = InlineKeyboardButton::new("🎳", CallbackData("Bowling🎳".to_string()));
    let soccer = InlineKeyboardButton::new("⚽️", CallbackData("Soccer⚽️".to_string()));
    let basketball = InlineKeyboardButton::new("🏀", CallbackData("Basketball🏀".to_string()));
    let slotmachine = InlineKeyboardButton::new("🎰", CallbackData("Slot Machine🎰".to_string()));

    let ikm = InlineKeyboardMarkup::default()
        .append_row(vec![dice, darts, bowling])
        .append_row(vec![soccer, basketball, slotmachine]);

    let inline_keyboard = ReplyMarkup::InlineKeyboard(ikm);

    if let Some(text) = msg.text() {
        if msg.thread_id.is_none() {
            return Ok(());
        }
        if let Some(thread_id) = msg.thread_id {
            match thread_id {
                12880 => (),
                _ => return Ok(()),
            }
        }
        // 2.- JUGADOR INGRESA MONTO
        let txt_to_digits: Option<f32> = text
            .chars() // Convert the input string into an iterator of characters
            .filter(|&c| c.is_digit(10) || c == '.') // Filter out non-digit characters and the dot '.'
            .collect::<String>() // Collect the filtered characters into a String
            .parse()
            .ok();
        // check if user has enough balance
        if let Some(digits) = txt_to_digits {
            if let Some(user) = msg.from().clone() {
                // SE TOMA REGISTRO EN MAP <ID-JUGADOR, MONTO-APUESTA> / SE REALIZA TRANSFERENCIA
                first_map.insert(user.id.0.to_string(), digits);

                // CHECK WALLET BALANCE
                //
                //
                let processing_msg = bot
                    .send_message(
                        ChatId(-1002028434193),
                        format!("<i>Checking your balance ...</i>",),
                    )
                    .message_thread_id(12880)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .send()
                    .await?;

                if !check_balance(bot.clone(), user.id.0.to_string(), digits).await? {
                    first_map.remove(&user.id.0.to_string());
                    //deleting processing message
                    bot.delete_message(ChatId(-1002028434193), processing_msg.id)
                        .send()
                        .await?;

                    bot.send_message(
                        ChatId(-1002028434193),
                        format!(
                            "You don't have enough $ICP in your balance. Please enter a correct amount."
                        ),
                    )
                    .message_thread_id(12880)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .send()
                    .await?;
                    return Ok(());
                }

                bot.delete_message(ChatId(-1002028434193), processing_msg.id)
                    .send()
                    .await?;

                // change buttons
                // change buttons
                // change buttons
                //let mut f: numfmt::Formatter = "[n.0]".parse().unwrap_or_default();

                //let fmt_digits = f.fmt2(digits);
                bot.send_message(
                    ChatId(-1002028434193),
                    format!(
                        "Your bet is {} $ICP.\nSelect a Game to play. Good Luck!",
                        digits
                    ),
                )
                .message_thread_id(12880)
                .reply_markup(inline_keyboard)
                .parse_mode(teloxide::types::ParseMode::Html)
                .send()
                .await?;
            };
        }
    };

    Ok(())
}

async fn determine_win(
    bot: Bot,
    second_map: (User, f32),
    msg: Message,
    rewards_multiple: f32,
) -> String {
    let mut f: numfmt::Formatter;
    f = "[n.0]".parse().unwrap_or_default();
    let mut resultado = String::from("");

    println!("\nInto: determine_win fn");
    match msg.clone().kind {
        MessageKind::Dice(dice_message) => {
            println!("Into: match msg.kind {{MessageKind::Dice}}");
            let emoji = dice_message.dice.emoji;
            let value = dice_message.dice.value;

            let game_result: bool = match emoji {
                DiceEmoji::Dice => value == 6,
                DiceEmoji::Darts => value == 6,
                DiceEmoji::Bowling => value == 6,
                DiceEmoji::Basketball => value == 5,
                DiceEmoji::Football => value == 5,
                DiceEmoji::SlotMachine => value == 64,
            };
            println!("game_result: {}", game_result);

            let game_name_for_result = match emoji {
                DiceEmoji::Dice => {
                    r#"at the <a href="https://t.me/c/2028434193/12880">Dice🎲</a> game. <b>1</b> in <b>6</b> chance."#
                }
                DiceEmoji::Darts => {
                    r#"at the <a href="https://t.me/c/2028434193/12880">Darts🎯</a> game. <b>1</b> in <b>6</b> chance."#
                }
                DiceEmoji::Bowling => {
                    r#"at the <a href="https://t.me/c/2028434193/12880">Bowling🎳</a> game. <b>1</b> in <b>6</b> chance."#
                }
                DiceEmoji::Basketball => {
                    r#"at the <a href="https://t.me/c/2028434193/12880">Basketball🏀</a> game. <b>1</b> in <b>5</b> chance."#
                }
                DiceEmoji::Football => {
                    r#"at the <a href="https://t.me/c/2028434193/12880">Soccer⚽️</a> game. <b>1</b> in <b>5</b> chance."#
                }
                DiceEmoji::SlotMachine => {
                    r#"against the <a href="https://t.me/c/2028434193/12880">Slot Machine🎰</a>. <b>1</b> in <b>64</b> chance !"#
                }
            };
            let game_name = match emoji {
                DiceEmoji::Dice => "Dice🎲",
                DiceEmoji::Darts => "Darts🎯",
                DiceEmoji::Bowling => "Bowling🎳",
                DiceEmoji::Basketball => "Basketball🏀",
                DiceEmoji::Football => "Soccer⚽️",
                DiceEmoji::SlotMachine => "Slot Machine🎰",
            };

            resultado = game_name.to_string();

            let user = second_map.0;
            let icp_bet = second_map.1;

            println!("ICP bet was found");

            if game_result {
                let rewards = icp_bet.clone() * rewards_multiple;
                let name_reference = format!(
                    r#"<a href="tg://user?id={}">{}</a>"#,
                    user.id.0,
                    user.full_name()
                );
                // if transfer bet not possible then is not possible to receive the price
                let transfer_ok = transfer_bet(user.id.0.to_string(), icp_bet.clone()).await;
                if transfer_ok.not() {
                    let _msg_to_forward = bot
                        .send_message(
                            ChatId(-1002028434193),
                            format!(
                                "{} turned 0 into 0 $ICP {}",
                                name_reference, game_name_for_result
                            ),
                        )
                        .message_thread_id(12880)
                        .parse_mode(teloxide::types::ParseMode::Html)
                        .reply_markup(ReplyMarkup::kb_remove())
                        .send()
                        .await;
                    return resultado;
                }
                // if transfer bet not possible then is not possible to receive the prize
                println!("Game result: WON");

                // SEND GLAZEMASTER

                treasury_transfer(user.id.0.to_string(), rewards).await;

                let treasury_balance = treasury_check_balance().await;

                let _msg_to_forward = bot
                    .send_message(
                        ChatId(-1002028434193),
                        format!(
                            "{} turned {} into {} $ICP {}",
                            name_reference,
                            f.clone().fmt2(icp_bet),
                            f.clone().fmt2(rewards),
                            game_name_for_result
                        ),
                    )
                    .message_thread_id(12880)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(ReplyMarkup::kb_remove())
                    .send()
                    .await;

                //add glaze treasury here
                let mg_balance = treasury_balance
                    .unwrap_or_default()
                    .parse::<i32>()
                    .unwrap_or_default();
                let fmt_mg_balance = f.fmt2(mg_balance);
                let _treasury_message = bot
                    .send_message(
                        ChatId(-1002028434193),
                        format!("ICP Treasury <pre>{}♾️</pre>", fmt_mg_balance),
                    )
                    .message_thread_id(12880)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(ReplyMarkup::kb_remove())
                    .send()
                    .await;

                let _betmessage = bot
                    .send_message(
                        ChatId(-1002028434193),
                        format!("Please type the amount of $ICP you wish to bet."),
                    )
                    .message_thread_id(12880)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .send()
                    .await;
            } else {
                //let mut f: numfmt::Formatter = "[n.0]".parse().unwrap_or_default();

                let name_reference = format!(
                    r#"<a href="tg://user?id={}">{}</a>"#,
                    user.id.0,
                    user.full_name()
                );

                let treasury_balance = treasury_check_balance().await;

                transfer_bet(user.id.0.to_string(), icp_bet.clone()).await;

                let mg_balance = treasury_balance
                    .unwrap_or_default()
                    .parse::<i32>()
                    .unwrap_or_default();
                
                let fmt_mg_balance = f.fmt2(mg_balance);

                let mut f: numfmt::Formatter = "[n.0]".parse().unwrap_or_default();
                let fmt_glaze_bet = f.fmt2(icp_bet.clone());

                println!("Player lost the game.");
                let _lose = bot
                    .send_message(
                        ChatId(-1002028434193),
                        format!("{} lost {} $ICP.", name_reference, fmt_glaze_bet),
                    )
                    .message_thread_id(12880)
                    .reply_markup(ReplyMarkup::kb_remove())
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .send()
                    .await;

                //put treasury message here but like 10000 + newgained $GLAZE
                let _treasury_message = bot
                    .send_message(
                        ChatId(-1002028434193),
                        format!(
                            "ICP♾️ Treasury <pre>{}♾️ + {}♾️</pre>",
                            fmt_mg_balance, fmt_glaze_bet
                        ),
                    )
                    .message_thread_id(12880)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .reply_markup(ReplyMarkup::kb_remove())
                    .send()
                    .await;

                let _betmessage = bot
                    .send_message(
                        ChatId(-1002028434193),
                        format!("Please type the amount of $ICP you wish to bet."),
                    )
                    .message_thread_id(12880)
                    .parse_mode(teloxide::types::ParseMode::Html)
                    .send()
                    .await;
            }
            //determine_win(bot, emoji, value, cloned_map, msg, 4).await;
        }
        _ => (),
    }
    return resultado;
}

pub async fn transfer_bet(user_id: String, amount: f32) -> bool {
    let indentity = user_id;
    let fmt_amount = to_eight_decimal_representation(amount.clone()).await;

    let principal_id = match std::process::Command::new("dfx")
        .arg("identity")
        .arg("--identity")
        .arg(indentity.clone())
        .arg("get-principal")
        .output()
    {
        Ok(output2) => output2,
        Err(err) => {
            eprintln!("{}", err);
            println!("DEBUG: dfx identity --identity 'get-principal' failed.");

            return false;
        }
    };
    let principal_id = String::from_utf8_lossy(&principal_id.stdout)
        .trim()
        .to_string();

    println!("user principal: {}", principal_id);

    let balance: std::process::Output = match std::process::Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg("--ic")
        .arg("ryjl3-tyaaa-aaaaa-aaaba-cai")
        .arg("icrc1_balance_of")
        .arg(format!(
            r#"(record {{ owner = principal "{}";}})"#,
            principal_id
        ))
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            println!("{:?}", err);
            return false;
        }
    };
    let balance_result = String::from_utf8_lossy(&balance.stdout).to_string();
    println!("user balance: {}", balance_result);

    let clean_balance_result: String =
        balance_result.chars().filter(|&c| c.is_digit(10) || c == '.').collect();

    // if balance result < amount NO SE PUEDE SEGUIR
    let parsed_balance: f32 = clean_balance_result.parse::<f32>().unwrap_or_default();
    println!("parsed balance: {}", parsed_balance);
    println!("amount: {}", amount);

    if parsed_balance < amount {
        return false;
    }

    let treasury_principal = "hjeru-fryw4-tc7c3-e7yx5-4ssj5-udjrq-nkjev-v7cfe-e2ium-pqccc-6ae";

    let transaction = match std::process::Command::new("dfx")
        .arg("canister")
        .arg("--identity")
        .arg(indentity.clone())
        .arg("call")
        .arg("--ic")
        .arg("ryjl3-tyaaa-aaaaa-aaaba-cai")
        .arg("icrc1_transfer")
        .arg(format!(
            r#"(record {{ to = record {{ owner = principal "{}";}}; amount = {}:nat;}})"#,
            treasury_principal, fmt_amount
        ))
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            print!("{:?}", err);
            return false;
        }
    };

    let status = transaction.status.to_string();
    println!("tx status: {}", status);
    let result = String::from_utf8_lossy(&transaction.stdout).to_string();
    println!("tx result:{}", result);
    let stderr = String::from_utf8_lossy(&transaction.stderr);
    println!("tx stderr {}", stderr);
    if result.contains("(variant { Ok") {
        return true;
    }

    false
}

async fn treasury_transfer(user_id: String, reward_amount: f32) {
    println!("inside treasury_transfer fn:");
    let fmt_reward_amount = to_eight_decimal_representation(reward_amount).await;

    let principal_id = match std::process::Command::new("dfx")
        .arg("identity")
        .arg("--identity")
        .arg(user_id)
        .arg("get-principal")
        .output()
    {
        Ok(output2) => output2,
        Err(_) => return,
    };
    let principal_id = String::from_utf8_lossy(&principal_id.stdout)
        .trim()
        .to_string();

    let transaction = match std::process::Command::new("dfx")
        .arg("canister")
        .arg("--identity")
        .arg("TREASURY")
        .arg("call")
        .arg("--ic")
        .arg("ryjl3-tyaaa-aaaaa-aaaba-cai")
        .arg("icrc1_transfer")
        .arg(format!(
            r#"(record {{ to = record {{ owner = principal "{}";}}; amount = {}:nat;}})"#,
            principal_id, fmt_reward_amount
        ))
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            print!("{:?}", err);
            return;
        }
    };

    let status = transaction.status.to_string();
    println!("tx status: {}", status);
    let result = String::from_utf8_lossy(&transaction.stdout).to_string();
    println!("tx result:{}", result);
    let stderr = String::from_utf8_lossy(&transaction.stderr);
    println!("tx stderr {}", stderr);
}

async fn check_balance(_bot: Bot, user_id: String, input_number: f32) -> Result<bool, RequestError> {
    println!(
        "ON check_balance fn: user id: {} - number: {}",
        user_id, input_number
    );

    let principal_id = match std::process::Command::new("dfx")
        .arg("identity")
        .arg("--identity")
        .arg(user_id.to_string())
        .arg("get-principal")
        .output()
    {
        Ok(output2) => output2,
        Err(err) => {
            eprintln!("{}", err);
            println!("DEBUG: dfx identity --identity 'get-principal' failed.");

            return Ok(false);
        }
    };
    let principal_id = String::from_utf8_lossy(&principal_id.stdout)
        .trim()
        .to_string();
    println!("User principal: {}", principal_id);

    let output = match std::process::Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg("--ic")
        .arg("ryjl3-tyaaa-aaaaa-aaaba-cai")
        .arg("icrc1_balance_of")
        .arg(format!(
            r#"(record {{ owner = principal "{}";}})"#,
            principal_id
        ))
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            eprintln!("{}", err);
            return Ok(false);
        }
    };
    let balance_result = String::from_utf8_lossy(&output.stdout).to_string();

    println!("balance result: {}", &balance_result);

    let clean_str: String = balance_result.chars().filter(|&c| c.is_digit(10) || c == '.').collect();

    let num = from_eight_decimal_string_representation(clean_str).await;

    if input_number <= num {
        return Ok(true);
    }

    Ok(false)
}

async fn treasury_check_balance() -> Option<String> {
    let treasury_principal = "hjeru-fryw4-tc7c3-e7yx5-4ssj5-udjrq-nkjev-v7cfe-e2ium-pqccc-6ae";

    let output = match std::process::Command::new("dfx")
        .arg("canister")
        .arg("call")
        .arg("--ic")
        .arg("ryjl3-tyaaa-aaaaa-aaaba-cai")
        .arg("icrc1_balance_of")
        .arg(format!(
            r#"(record {{ owner = principal "{}";}})"#,
            treasury_principal
        ))
        .output()
    {
        Ok(output) => output,
        Err(err) => {
            eprintln!("{}", err);
            return None;
        }
    };
    let balance_result = String::from_utf8_lossy(&output.stdout).to_string();

    println!("balance result: {}", &balance_result);

    let cleaned_str: String = balance_result.chars().filter(|&c| c.is_digit(10)).collect();

    let fmt_result = from_eight_decimal_string_representation(cleaned_str).await;
    /*
    let num: u64 = match num_str.parse() {
        Ok(num) => num,
        Err(err) => {
            println!("Error parsing balance number from str to u64");
            eprintln!("Error parsing number: {}", err);
            0
        }
    }; */

    Some(fmt_result.to_string())
}

async fn to_eight_decimal_representation(num: f32) -> u64 {
    // Multiply the f32 number by 10^8
    let multiplied = (num * 1_000_000_00.0) as u64;
    multiplied
}

async fn from_eight_decimal_string_representation(num_str: String) -> f32 {
    // Parse the string into a float and divide by 10^8
    let divided = num_str.parse::<f32>().unwrap_or_default() / 1_000_000_00.0;
    divided
}
