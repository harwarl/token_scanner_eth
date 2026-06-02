use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    requests::Requester,
    types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};
use url::Url;

use crate::{config::Config, types::TokenInfo};

pub async fn send_tg_message(bot: &Bot, token_info: TokenInfo) {
    let message = format!(
        "🌞 Token Name | {}\n\
        📜 CA: <code>{}</code>\n\n\
        🎎 Total Supply: {}\n\
        🏁 Contract Verified: {}\n\
        💧 Liquidity: ${:.2}\n\
        🔒 LP Lock: {}\n\
        🏠 Renounced: {}\n\
        💰 Tax: Buy {}% | Sell {}%\n\
        🏆 Market Cap: ${:.2}\n\
        🍯 Honeypot: {}\n\
        👩‍🍳 Deployer: <code>{}</code>\n\n\
        📈 <a href=\"https://www.dextools.io/app/en/ether/pair-explorer/{}\">Dextools</a> | \
        <a href=\"https://dexscreener.com/ethereum/{}\">DexScreener</a> | \
        <a href=\"https://dexspy.io/eth/token/{}\">DexSpy</a> | \
        <a href=\"https://www.dexview.com/eth/{}\">DexView</a>",
        token_info.name,
        token_info.address,
        token_info.total_supply,
        if token_info.verified { "✅" } else { "❌" },
        token_info.liquidity_usd,
        if token_info.lp_locked { "✅" } else { "❌" },
        if token_info.renounced { "✅" } else { "❌" },
        token_info.buy_tax,
        token_info.sell_tax,
        token_info.market_cap_usd,
        if token_info.honeypot {
            "🍯 Yes"
        } else {
            "✅ No"
        },
        token_info.deployer,
        token_info.address,
        token_info.address,
        token_info.address,
        token_info.address,
    );

    let keyboard = InlineKeyboardMarkup::new(vec![
        vec![InlineKeyboardButton::url(
            "🍌 BUY with Banana Sniper 🍌",
            Url::parse("https://t.me/BananaGunSniper_bot").unwrap(),
        )],
        vec![
            InlineKeyboardButton::url(
                "🤖 BUY with Maestro 🤖",
                Url::parse("https://t.me/maestro").unwrap(),
            ),
            InlineKeyboardButton::url(
                "🤖 BUY with Maestro Pro 🤖",
                Url::parse("https://t.me/maestro").unwrap(),
            ),
        ],
        vec![InlineKeyboardButton::url(
            "🎯 BUY with Magnum Sniper 🎯",
            Url::parse("https://t.me/magnum_trade_bot").unwrap(),
        )],
    ]);

    let chat_id = ChatId(-1002048202426);

    if let Err(e) = bot
        .send_message(chat_id, message)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await
    {
        tracing::error!("Failed to send Telegram message: {}", e);
    }
}
