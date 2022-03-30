use catan::map::MapConfig;

fn main() {
    let res: MapConfig = serde_json::from_str(include_str!("../../../maps/default.json")).unwrap();
    println!("{:#?}", res);
}
