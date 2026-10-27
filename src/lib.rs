//! TheAudioDB (<https://www.theaudiodb.com>) metadata provider extension
//! for LisekMovie — **skeleton for the coming audio libraries**.
//!
//! TheAudioDB has artists, albums and tracks with biographies, genres,
//! artwork (thumbs, fanart, logos, CD art) and MusicBrainz ids:
//! `https://www.theaudiodb.com/api/v1/json/{key}/search.php?s={artist}`,
//! `searchalbum.php?s={artist}&a={album}`, `album.php?m={id}`. The public
//! test key `2` covers the basic calls; a Patreon key the rest. The key
//! entered in Admin → Plugins is kept for that.
//!
//! Music needs item kinds `lisek-plugin-api` does not have yet (album,
//! artist, track — LisekMovie only knows films and series today). Until
//! the audio phase adds them, this provider is registered (Admin → Plugins
//! lists it, libraries can order it) and answers every film or series
//! question with nothing, without any network request.
//!
//! TODO: implement endpoint — with the audio item kinds: artist and album
//! search and lookup, artwork.

use async_trait::async_trait;
use lisek_plugin_api::{
    MediaKind, MetadataProvider, MovieQuery, ProviderCandidate, ProviderError, ProviderMatch,
    SeriesQuery,
};

/// The public test key of TheAudioDB.
pub const TEST_KEY: &str = "2";

/// The [`MetadataProvider`] for `"audiodb"`. Films and series are not in
/// TheAudioDB: every answer is empty.
#[derive(Debug)]
pub struct AudiodbProvider {
    api_key: String,
}

impl AudiodbProvider {
    /// An empty key means the public test key.
    pub fn new(api_key: impl Into<String>) -> Self {
        let api_key = api_key.into();
        Self {
            api_key: if api_key.trim().is_empty() {
                TEST_KEY.to_string()
            } else {
                api_key
            },
        }
    }

    /// The key requests will use.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }
}

#[async_trait]
impl MetadataProvider for AudiodbProvider {
    fn provider_key(&self) -> &'static str {
        "audiodb"
    }

    async fn find_movie(
        &self,
        _query: &MovieQuery<'_>,
        _lang: &str,
    ) -> Result<Option<ProviderMatch>, ProviderError> {
        Ok(None)
    }

    async fn find_series(
        &self,
        _query: &SeriesQuery<'_>,
        _lang: &str,
    ) -> Result<Option<ProviderMatch>, ProviderError> {
        Ok(None)
    }

    async fn search(
        &self,
        _kind: MediaKind,
        _title: &str,
        _year: Option<i32>,
        _lang: &str,
    ) -> Result<Vec<ProviderCandidate>, ProviderError> {
        tracing::debug!(
            provider = "audiodb",
            "music search arrives with audio libraries"
        );
        Ok(Vec::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn films_and_series_get_no_answer() {
        let provider = AudiodbProvider::new("");
        assert_eq!(provider.provider_key(), "audiodb");
        assert_eq!(provider.api_key(), TEST_KEY);
        assert_eq!(AudiodbProvider::new("patreon").api_key(), "patreon");
        assert!(
            provider
                .search(MediaKind::Movie, "Daft Punk", None, "en")
                .await
                .unwrap()
                .is_empty()
        );
        assert!(
            provider
                .find_series(&SeriesQuery { title: "x" }, "en")
                .await
                .unwrap()
                .is_none()
        );
    }
}
