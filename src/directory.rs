use ulid::Ulid;

#[derive(Debug)]
pub struct DirectoryItem {
    pub podcast_name: String,
    pub feed_url: String,
    pub podcast_id: Ulid,
}

fn compare_strings_case_invariant(a: &String, b: &String) -> std::cmp::Ordering {
    a.to_lowercase().as_str().cmp(b.to_lowercase().as_str())
}

pub fn sort_directory(dir: &mut Vec<DirectoryItem>) {
    dir.sort_by(|a, b| compare_strings_case_invariant(&a.podcast_name, &b.podcast_name));
}