mod client;

fn main() {
    let bot = client::KikClient{username: "username", password: "password", device_id: "62030843678b7376a707ca3d11e87836", android_id: "849d4ffb0c020de6"};
    let connection = bot.check_connection();
    if connection {
        bot.start_stream();
    }
}
