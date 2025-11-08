use songbird::input::{Input, YoutubeDl};
use reqwest::Client;

pub fn ytdlp_input(url: &str) -> Input {
    let http = Client::new();
    let mut ytdl = YoutubeDl::new(http, url.to_owned());

    ytdl = ytdl.user_args(vec![
        "--js-runtimes".into(), "deno".into(),
        "--cookies".into(), "./cookies.txt".into(),
        "--no-playlist".into(),
        "-f".into(), "ba[ext=m4a]/ba[acodec^=opus][protocol!=m3u8]".into(),
    ]);

    ytdl.into()
}
