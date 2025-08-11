use std::collections::HashMap;

pub fn steamdeck_compatibility_map() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();
    map.insert(1, "SteamDeckVerified_Category_Verified");
    map.insert(2, "SteamDeckVerified_Category_Playable");
    map.insert(3, "SteamDeckVerified_Category_Unsupported");
    map.insert(4, "SteamDeckVerified_Category_Unknown");
    map
}
