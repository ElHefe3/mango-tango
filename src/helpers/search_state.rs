use dashmap::DashMap;
use once_cell::sync::Lazy;
use serenity::model::id::{GuildId, UserId};

use crate::adapters::youtube::YtSearchResult;

static LAST_RESULTS: Lazy<DashMap<(GuildId, UserId), Vec<YtSearchResult>>> =
    Lazy::new(DashMap::new);

pub fn store_results(guild: GuildId, user: UserId, results: Vec<YtSearchResult>) {
    LAST_RESULTS.insert((guild, user), results);
}

pub fn get_results(guild: GuildId, user: UserId) -> Option<Vec<YtSearchResult>> {
    LAST_RESULTS.get(&(guild, user)).map(|entry| entry.clone())
}
