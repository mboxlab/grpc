use discord_rich_presence::{
    activity::{self},
    DiscordIpc, DiscordIpcClient,
};
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn main() -> () {
    let mut client = DiscordIpcClient::new("1122498317755101195").expect("Failed to create client");

    let time_unix: i64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    let activity = activity::Activity::new()
        .state("Играет")
        .assets(
            activity::Assets::new()
                .large_image("https://gmodmbox.online/box_motorola.png")
                .large_text("MBOX SandBox")
                .small_image("https://developer.valvesoftware.com/w/images/e/ea/Gmod_icon.png")
                .small_text("Garry's mod"),
        )
        .buttons(vec![
            activity::Button::new("Подключиться!", "https://gmodmbox.online/connect"),
            activity::Button::new("Дискорд", "https://discord.com/invite/5zxJ5XvpRP"),
        ])
        .timestamps(activity::Timestamps::new().start(time_unix));

    match client.connect() {
        Ok(_) => {
            println!("Client connected to Discord successfully.");
        }
        Err(_) => {
            println!("Client failed to connect to Discord, Please try again or relaunch Discord.");
        }
    };

    match client.set_activity(activity) {
        Ok(_) => {
            println!("Client set activity successfully.");
        }
        Err(_) => {
            println!("Client failed to set activity, Please try again or relaunch Discord.");
        }
    };
    loop {
        thread::sleep(Duration::from_secs(10));
    }
}
