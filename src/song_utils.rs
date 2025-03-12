pub struct SongInfo {
    //TODO:
    // fingerprint: ?
    uid: u32,
    name: String,
    mp3_path: String,
}

//TODO: database storage

/// to be implemented: save to a database, rn will save to assets/songs
pub fn save_song_to(song_info: SongInfo) {}
