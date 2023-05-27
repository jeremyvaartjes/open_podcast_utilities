mod directory;
use directory::DirectoryItem;
use ulid::Ulid;

fn main() {
    let dir1 = DirectoryItem {
        podcast_name: "My pod".to_string(),
        feed_url: "http://mypod.fm".to_string(),
        podcast_id: Ulid::new()
    };
    
    let dir2 = DirectoryItem {
        podcast_name: "My pod 2".to_string(),
        feed_url: "http://mypod.fm/2".to_string(),
        podcast_id: Ulid::new()
    };
    
    let mut pub_directory: Vec<DirectoryItem> = Vec::new();
    pub_directory.push(dir1);
    pub_directory.push(dir2);
    
    pub_directory.sort_by(|a, b| a.podcast_name.to_lowercase().as_str().cmp(b.podcast_name.to_lowercase().as_str()));
    
    println!("{:?}", pub_directory);
}
