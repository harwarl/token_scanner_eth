use teloxide::{
    Bot,
    payloads::SendMessageSetters,
    requests::Requester,
    types::{ChatId, InlineKeyboardButton, InlineKeyboardMarkup, ParseMode},
};
use url::Url;

use crate::{
    types::TokenInfo,
    utils::helpers::{format_number, volatility_label},
};

pub async fn send_tg_message(bot: &Bot, token_info: TokenInfo) {
    let message = format!(
        "🌞 <b>{name}</b> • <code>{address}</code>\n\
    \n\
    🏆 MCap: <b>${mcap}</b> • 💧 Liq: <b>${liq}</b> • 📊 M/L: <b>{ml:.2}x</b>\n\
    💵 Price: <b>${price:.8}</b> • 5m: <b>{pc5m:+.1}%</b> • 1h: <b>{pc1h:+.1}%</b>\n\
    📉 Volatility: <b>{vol_label}</b> ({volatility:.1}%)\n\
    🎎 Supply: {supply}\n\
    \n\
    📊 <b>Volume</b>\n\
    ┣ 5m:  ${v5m}  •  1h:  ${v1h}\n\
    ┗ 6h:  ${v6h}  •  24h: ${v24h}\n\
    \n\
    🔄 <b>Transactions</b>\n\
    ┣ 5m:  🟢 {b5m} / 🔴 {s5m}\n\
    ┣ 1h:  🟢 {b1h} / 🔴 {s1h}\n\
    ┣ 6h:  🟢 {b6h} / 🔴 {s6h}\n\
    ┗ 24h: 🟢 {b24h} / 🔴 {s24h}\n\
    \n\
    💹 Buy Ratio: <b>{ratio:.0}%</b> • 👥 Uniq Buyers: <b>{uniq}</b>\n\
    \n\
    🏁 Verified: {verified} • 🏠 Renounced: {renounced}\n\
    🔒 LP Lock: {lp} • 🍯 Honeypot: {hp}\n\
    💰 Tax: Buy <b>{btax}%</b> • Sell <b>{stax}%</b>\n\
    \n\
    👩‍🍳 Deployer: <code>{deployer}</code>\n\
    📅 Age: <b>{age} days</b> • 🆕 Fresh: {fresh} • ⚠️ Bad Rep: {rep}\n\
    \n\
    <a href=\"https://www.dextools.io/app/en/ether/pair-explorer/{address}\">Dextools</a> • \
    <a href=\"https://dexscreener.com/ethereum/{address}\">DexScreener</a> • \
    <a href=\"https://dexspy.io/eth/token/{address}\">DexSpy</a> • \
    <a href=\"https://www.dexview.com/eth/{address}\">DexView</a> • \
    <a href=\"https://x.com/search?q=%24{name}+OR+{address}&src=typed_query&f=live\">𝕏</a>",
        name = token_info.name,
        address = token_info.address,
        mcap = format_number(token_info.market_cap_usd),
        liq = format_number(token_info.liquidity_usd),
        ml = token_info.mcap_to_liq_ratio,
        price = token_info.price_usd,
        pc5m = token_info.price_change_5m,
        pc1h = token_info.price_change_1h,
        vol_label = volatility_label(token_info.volatility),
        volatility = token_info.volatility,
        supply = format_number(token_info.total_supply),
        v5m = format_number(token_info.volume_5m),
        v1h = format_number(token_info.volume_1h),
        v6h = format_number(token_info.volume_6h),
        v24h = format_number(token_info.volume_24h),
        b5m = token_info.buys_5m,
        s5m = token_info.sells_5m,
        b1h = token_info.buys_1h,
        s1h = token_info.sells_1h,
        b6h = token_info.buys_6h,
        s6h = token_info.sells_6h,
        b24h = token_info.buys_24h,
        s24h = token_info.sells_24h,
        ratio = token_info.buy_ratio * 100.0,
        uniq = token_info.unique_buyers_count,
        verified = if token_info.verified { "✅" } else { "❌" },
        renounced = if token_info.renounced { "✅" } else { "❌" },
        lp = if token_info.lp_locked { "✅" } else { "❌" },
        hp = if token_info.honeypot {
            "🍯 Yes"
        } else {
            "✅ No"
        },
        btax = token_info.buy_tax,
        stax = token_info.sell_tax,
        deployer = token_info.deployer,
        age = token_info.deployer_age_days,
        fresh = if token_info.is_fresh_wallet {
            "⚠️ Yes"
        } else {
            "✅ No"
        },
        rep = if token_info.bad_reputation {
            "⚠️ Yes"
        } else {
            "✅ No"
        },
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

    let chat_id = ChatId(7070082881);

    if let Err(e) = bot
        .send_message(chat_id, message)
        .parse_mode(ParseMode::Html)
        .reply_markup(keyboard)
        .await
    {
        tracing::error!("Failed to send Telegram message: {}", e);
    }
}
