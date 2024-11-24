use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};

use std::collections::HashMap;


#[derive(Debug, Deserialize)]
pub struct GameList {
    #[serde(deserialize_with = "deserialize_game_list")]
    games: HashMap<String, Vec<u32>>,
}

fn deserialize_game_list<'de, D>(deserializer: D) -> Result<HashMap<String, Vec<u32>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::de::{self, MapAccess};
    use std::collections::HashMap;
    use std::marker::PhantomData;

    struct GameListVisitor;

    impl<'de> de::Visitor<'de> for GameListVisitor {
        type Value = HashMap<String, Vec<u32>>;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a map of game packages to their frame rates")
        }

        fn visit_map<V>(self, mut map: V) -> Result<HashMap<String, Vec<u32>>, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut games = HashMap::new();
            while let Some((key, value)) = map.next_entry::<String, Vec<u32>>()? {
                games.insert(key, value);
            }
            Ok(games)
        }
    }

    deserializer.deserialize_map(GameListVisitor)
}


#[derive(Debug, Deserialize)]
pub struct Setting {
    pub game_list: GameList,
}


fn main() -> io::Result<()> {
    // 打开并读取TOML文件
    let mut file = File::open("games.toml")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // 将字符串内容反序列化为Setting结构体
    let setting: Setting = toml::from_str(&contents)?;

    // 打印解析后的结构体
    println!("{:#?}", setting);

    Ok(())
}
