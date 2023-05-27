mod directory;
use directory::{DirectoryItem,sort_directory};
use ulid::Ulid;

fn main() {
    let dir1 = DirectoryItem {
        podcast_name: "B My pod".to_string(),
        feed_url: "http://mypod.fm".to_string(),
        podcast_id: Ulid::new()
    };
    
    let dir2 = DirectoryItem {
        podcast_name: "A My pod 2".to_string(),
        feed_url: "http://mypod.fm/2".to_string(),
        podcast_id: Ulid::new()
    };
    
    let mut pub_directory: Vec<DirectoryItem> = Vec::new();
    pub_directory.push(dir1);
    pub_directory.push(dir2);
    
    sort_directory(&mut pub_directory);
    
    println!("{:?}", pub_directory);
}
