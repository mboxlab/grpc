pub mod grpc;

use rglua::{interface, prelude::*};

#[gmod_open]
fn main(l: LuaState) -> Result<i32, interface::Error> {
    grpc::init(l);
    let lib = reg! [
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
