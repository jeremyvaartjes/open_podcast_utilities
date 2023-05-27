use ulid::Ulid;

#[derive(Debug)]
pub struct DirectoryItem {
    pub podcast_name: String,
    pub feed_url: String,
    pub podcast_id: Ulid,
}