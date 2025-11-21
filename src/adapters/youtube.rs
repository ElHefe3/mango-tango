use anyhow::{Result, Context};
use songbird::input::{Input, YoutubeDl};
use reqwest::Client;
use tokio::process::Command;

#[derive(Debug, Clone)]
pub struct YtSearchResult {
    pub title: String,
    pub url: String,
    pub duration: Option<u64>,
}

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

pub async fn yt_search(query: &str, limit: usize) -> Result<Vec<YtSearchResult>> {
    let search_expr = format!("ytsearch{limit}:{query}");

    let output = Command::new("yt-dlp")
        .args([
            "--skip-download",
            "--no-warnings",
            "--print",
            "%(title)s\t%(webpage_url)s\t%(duration)s",
            &search_expr,
        ])
        .output()
        .await
        .context("failed to run yt-dlp")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("yt-dlp failed: {stderr}"));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);

    let results = stdout
        .lines()
        .filter_map(|line| {
            let parts: Vec<_> = line.split('\t').collect();
            if parts.len() < 2 {
                return None;
            }

            let title = parts[0].to_string();
            let url   = parts[1].to_string();

            let duration = parts
                .get(2)
                .and_then(|d| d.parse::<u64>().ok());

            Some(YtSearchResult { title, url, duration })
        })
        .collect();

    Ok(results)
}
