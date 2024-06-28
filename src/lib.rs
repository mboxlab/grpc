pub mod grpc;

use rglua::{ interface, prelude::* };

#[gmod_open]
fn main(l: LuaState) -> Result<i32, interface::Error> {
    grpc::init(l);
    let lib =
        reg![
            "setLargeImage" => grpc::set_large_image,
            "setLargeText" => grpc::set_large_text,
            "setSmallImage" => grpc::set_small_image,
            "setSmallText" => grpc::set_small_text,
            "setState" => grpc::set_state,
            "setButton1" => grpc::set_button_1,
            "setButton2" => grpc::set_button_2,
            "setClientID" => grpc::set_client_id,
            "close" => grpc::close,
            "reconnect" => grpc::reconnect,
            "connect" => grpc::connect_grpc,
            "update" => grpc::update
    ];

    luaL_register(l, cstr!("grpc"), lib.as_ptr());
    Ok(0)
}

#[gmod_close]
fn close(_l: LuaState) -> i32 {
    grpc::deinit();
    0
}
