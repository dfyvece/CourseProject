#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use tauri::generate_context;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod searcher;
use searcher::{Index, Token, BM25};

#[derive(Serialize)]
struct VideoSegment {
    id: String,
    start: f32,
}

#[derive(Serialize)]
struct SearchResult {
    msg: String,
    videos: Vec<VideoSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TranscriptSegments {
    text: String,
    start: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Transcript {
    id: String,
    content: String,
    segments: Vec<TranscriptSegments>,
}

type RawTranscript = Vec<TranscriptSegments>;
type TranscriptList = HashMap<String, RawTranscript>;
type ParsedTranscriptList = Vec<Transcript>;

struct LoadedData {
    transcripts: ParsedTranscriptList,
    index: Index<Transcript>,
}

static TRANSCRIPTS: &str = include_str!("../resources/transcripts.json");

fn tokenizer(s: String) -> Vec<Token> {
    s.split(' ')
        .map(|slice| slice.to_lowercase())
        .collect::<Vec<String>>()
}

fn accessor(transcript: &Transcript) -> String {
    transcript.content.clone()
}

#[tauri::command]
fn search(query: &str, state: tauri::State<LoadedData>) -> SearchResult {
    let transcripts = &state.transcripts;
    let index = &state.index;
    let mut videos = Vec::with_capacity(transcripts.len());

    let results = index.query(query, BM25::new());
    for result in results {
        fn accessor(segments: &TranscriptSegments) -> String {
            segments.text.clone()
        }
        let transcript = &transcripts[result.id];
        let transcript_index = Index::from(&transcript.segments, tokenizer, accessor);
        let top_segments = transcript_index.query(query, BM25::new());
        if top_segments.len() == 0 {
            continue;
        }
        let top_segment = top_segments[0].id;
        videos.push(VideoSegment {
            id: transcripts[result.id].id.clone(),
            start: transcript.segments[top_segment].start,
        });
    }

    let plural = if videos.len() > 1 { "s" } else { "" };

    SearchResult {
        msg: format!("Found {} result{} for: \"{}\"", videos.len(), plural, query),
        videos,
    }
}

fn main() {
    let transcripts: TranscriptList = serde_json::from_str(TRANSCRIPTS).unwrap();

    let transcripts: ParsedTranscriptList = transcripts
        .iter()
        .map(|(id, transcript)| {
            let content = transcript
                .iter()
                .map(|transcript| transcript.text.clone())
                .collect::<Vec<String>>()
                .join(" ");
            return Transcript {
                id: id.to_owned(),
                content,
                segments: transcript.clone(),
            };
        })
        .collect();

    let index = Index::from(&transcripts, tokenizer, accessor);

    println!("num transcipts: {}", transcripts.len());

    let state = LoadedData { transcripts, index };

    tauri::Builder::default()
        .manage::<LoadedData>(state.into())
        .invoke_handler(tauri::generate_handler![search])
        .run(generate_context!())
        .expect("error while running tauri application");
}
