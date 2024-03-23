use std::time::{SystemTime, UNIX_EPOCH};

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use once_cell::sync::Lazy;
use rglua::lua::{lua_call, lua_getfield, lua_getglobal, lua_tointeger};
use rglua::*;
use rglua::{interface, lua::LuaState};
static mut TIMESTAMP: i64 = 0;
static mut CLIENT: Lazy<DiscordIpcClient> =
    Lazy::new(|| DiscordIpcClient::new("1122498317755101195").expect("Failed to create client"));

fn get_player_count(l: LuaState) -> i64 {
    lua_getglobal(l, cstr!("player"));
    lua_getfield(l, -1, cstr!("GetCount"));
    lua_call(l, 0, 1);
    lua_tointeger(l, -1).try_into().unwrap()
}
unsafe fn update_internal(player_count: String) {
    let activity = activity::Activity::new()
        .state(&player_count)
        .assets(
            activity::Assets::new()
                .large_image("https://gmodmbox.online/box_motorola.png")
                .large_text("METEOR SANDBOX")
                .small_image("https://developer.valvesoftware.com/w/images/e/ea/Gmod_icon.png")
                .small_text("Garry's mod"),
        )
        .buttons(vec![
            activity::Button::new("Подключиться!", "https://gmodmbox.online/connect"),
            activity::Button::new("Дискорд", "https://discord.com/invite/5zxJ5XvpRP"),
        ])
        .timestamps(activity::Timestamps::new().start(TIMESTAMP));
    let _ = CLIENT.set_activity(activity);
}
#[lua_function]
pub(crate) fn update(l: LuaState) -> Result<i32, interface::Error> {
    let playerscount: i64 = get_player_count(l);
    let from: String = playerscount.to_string().to_owned() + " / 128";
    unsafe { update_internal(from) };
    Ok(0)
}
pub(crate) fn init(l: LuaState) {
    unsafe {
        TIMESTAMP = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64
    };
    match unsafe { CLIENT.connect() } {
        Ok(_) => update(l),
        Err(_) => {
            println!("Client failed to connect to Discord, Please try again or relaunch Discord.");
            0
        }
    };
}

pub(crate) fn deinit() {
    unsafe {
        let _ = CLIENT.close();
    }
}
