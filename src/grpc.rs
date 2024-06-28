use std::time::{ SystemTime, UNIX_EPOCH };

use discord_rich_presence::{ activity, DiscordIpc, DiscordIpcClient };
use lua::luaL_checkstring;
use once_cell::sync::Lazy;
use rglua::*;
use rglua::{ interface, lua::LuaState };

// че за хуйня??
static mut TIMESTAMP: i64 = 0;
static mut LARGE_IMAGE: &str = "https://gmodmbox.online/meteor.png";
static mut LARGE_TEXT: &str = "METEOR SANDBOX";
static mut SMALL_IMAGE: &str = "https://developer.valvesoftware.com/w/images/e/ea/Gmod_icon.png";
static mut SMALL_TEXT: &str = "Garry's mod";
static mut STATE: &str = "empty";
static mut BUTTON1_LABEL: &str = "Подключиться!";
static mut BUTTON1_URL: &str = "steam://connect/89.111.141.237:27015";
static mut BUTTON2_LABEL: &str = "Дискорд";
static mut BUTTON2_URL: &str = "https://discord.com/invite/5zxJ5XvpRP";

static mut CLIENT: Lazy<DiscordIpcClient> = Lazy::new(||
    DiscordIpcClient::new("1027974934498123777").expect("Failed to create client")
);

unsafe fn update_internal() {
    let activity = activity::Activity
        ::new()
        .state(STATE)
        .assets(
            activity::Assets
                ::new()
                .large_image(LARGE_IMAGE)
                .large_text(LARGE_TEXT)
                .small_image(SMALL_IMAGE)
                .small_text(SMALL_TEXT)
        )
        .buttons(
            vec![
                activity::Button::new(BUTTON1_LABEL, BUTTON1_URL),
                activity::Button::new(BUTTON2_LABEL, BUTTON2_URL)
            ]
        )
        .timestamps(activity::Timestamps::new().start(TIMESTAMP));
    println!("update");
    println!("{}", LARGE_IMAGE);
    if CLIENT.set_activity(activity).is_err() {
        println!("Reconnecting grpc...");
        CLIENT.reconnect().unwrap();
    }
}

#[lua_function]
pub(crate) fn set_large_image(l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        LARGE_IMAGE = rstr!(luaL_checkstring(l, 1));
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn set_large_text(l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        LARGE_TEXT = rstr!(luaL_checkstring(l, 1));
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn set_small_image(l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        SMALL_IMAGE = rstr!(luaL_checkstring(l, 1));
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn set_small_text(l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        SMALL_TEXT = rstr!(luaL_checkstring(l, 1));
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn set_state(l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        STATE = rstr!(luaL_checkstring(l, 1));
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn set_button_1(l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        BUTTON1_LABEL = rstr!(luaL_checkstring(l, 1));
        BUTTON1_URL = rstr!(luaL_checkstring(l, 2));
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn set_button_2(l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        BUTTON2_LABEL = rstr!(luaL_checkstring(l, 1));
        BUTTON2_URL = rstr!(luaL_checkstring(l, 2));
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn set_client_id(l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        CLIENT.client_id = rstr!(luaL_checkstring(l, 1)).to_owned();
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn close(_l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        let _ = CLIENT.close();
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn reconnect(_l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        let _ = CLIENT.reconnect();
    }
    Ok(0)
}
#[lua_function]
pub(crate) fn connect_grpc(l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        match CLIENT.connect() {
            Ok(_) => printgm!(l, "grpc connected"),
            Err(_) => {
                printgm!(
                    l,
                    "Client failed to connect to Discord, Please try again or relaunch Discord."
                );
                0;
            }
        }
    }
    Ok(0)
}

#[lua_function]
pub(crate) fn update(_l: LuaState) -> Result<i32, interface::Error> {
    unsafe {
        update_internal();
    }
    Ok(0)
}

pub(crate) fn init(_l: LuaState) {
    unsafe {
        TIMESTAMP = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
    }
}

pub(crate) fn deinit() {
    unsafe {
        let _ = CLIENT.close();
    }
}
