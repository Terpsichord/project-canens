use anyhow::{anyhow, bail, Context};
use derive_more::{Display, From};
use itertools::Itertools;
use metadata_filter::filters;
use metadata_filter::rules::{
    clean_explicit_filter_rules, live_filter_rules, remastered_filter_rules,
    trim_whitespace_filter_rules, version_filter_rules,
};
use num_enum::{FromPrimitive, TryFromPrimitive};
use rspotify::model::{AudioFeatures, FullTrack, Id, Modality};
use std::fmt;
use std::fmt::Formatter;

pub mod card;
pub mod info;

#[derive(PartialEq, Clone, Debug)]
pub struct SongPreview {
    pub id: String,
    pub title: String,
    pub artist: String,
    pub cover_url: String,
}

impl TryFrom<FullTrack> for SongPreview {
    type Error = anyhow::Error;

    fn try_from(track: FullTrack) -> anyhow::Result<Self> {
        Ok(Self {
            id: track
                .id
                .ok_or_else(|| anyhow!("Track had no id"))?
                .id()
                .into(),
            title: track.name,
            artist: track
                .artists
                .into_iter()
                .map(|artist| artist.name)
                .join(", "),
            cover_url: track
                .album
                .images
                .into_iter()
                .next()
                .context("Track had no images")?
                .url,
        })
    }
}

#[derive(TryFromPrimitive, Display, Clone, Debug, PartialEq)]
#[repr(i32)]
pub enum Note {
    C,
    Db,
    D,
    Eb,
    E,
    F,
    Gb,
    G,
    Ab,
    A,
    Bb,
    B,
    Unknown = -1,
}

impl Note {
    // This only works if the variants of the `Key` enum are named accordingly
    pub fn is_accidental(&self) -> bool {
        self.to_string().ends_with("b")
    }
}

#[derive(Display, Clone, PartialEq, Debug)]
pub enum Mode {
    Major,
    Minor,
    Unknown,
}

impl From<Modality> for Mode {
    fn from(modality: Modality) -> Self {
        match modality {
            Modality::Major => Mode::Major,
            Modality::Minor => Mode::Minor,
            Modality::NoResult => Mode::Unknown,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Key {
    pub note: Note,
    pub mode: Mode,
}

impl fmt::Display for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mode_str = if self.note != Note::Unknown {
            self.mode.to_string()
        } else {
            "".into()
        };
        write!(f, "{} {}", self.note, mode_str)
    }
}

#[derive(From, Clone, Debug, PartialEq)]
pub struct Tempo(f32);

impl fmt::Display for Tempo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0 as i32)
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct Song {
    pub id: String,
    pub title: String,
    pub filtered_title: String,
    pub artist: String,
    pub cover_url: String,
    pub key: Key,
    pub tempo: Tempo,
}

#[derive(Default)]
pub struct SongBuilder {
    preview: Option<SongPreview>,
    full_track: Option<FullTrack>,
    audio_features: Option<AudioFeatures>,
}

impl SongBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn preview(mut self, preview: SongPreview) -> Self {
        self.preview = Some(preview);
        self
    }

    pub fn full_track(mut self, full_track: FullTrack) -> Self {
        self.full_track = Some(full_track);
        self
    }

    pub fn audio_features(mut self, audio_features: AudioFeatures) -> Self {
        self.audio_features = Some(audio_features);
        self
    }

    pub fn build(self) -> anyhow::Result<Song> {
        if self.preview.is_some() && self.full_track.is_some() {
            bail!("Was provided with both preview and full_track");
        }
        let preview = self.preview.unwrap_or(
            self.full_track
                .ok_or(anyhow!("Neither preview nor full_track provided"))?
                .try_into()?,
        );

        let id = preview.id;
        let title = preview.title;
        let artist = preview.artist;
        let cover_url = preview.cover_url;

        let filtered_title = filters::apply_rules(
            &title,
            &[
                clean_explicit_filter_rules(),
                remastered_filter_rules(),
                live_filter_rules(),
                version_filter_rules(),
                trim_whitespace_filter_rules(),
            ]
            .concat(),
        );

        let audio_features = self
            .audio_features
            .ok_or(anyhow!("audio_features not provided"))?;
        let tempo = audio_features.tempo.into();

        let note = audio_features.key.try_into()?;
        let mode = audio_features.mode.into();
        let key = Key { note, mode };

        Ok(Song {
            id,
            title,
            filtered_title,
            artist,
            cover_url,
            key,
            tempo,
        })
    }
}

impl Song {
    pub fn builder() -> SongBuilder {
        SongBuilder::default()
    }
}
