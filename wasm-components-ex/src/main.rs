use wit_parser::{Resolve, World};
use std::path::Path;

fn main() {
  let mut resolve = Resolve::new();
  resolve.push_path(Path::new("./todo-list.wit"));
  for (worldId, world) in resolve.worlds.iter() {
    dbg!(worldId);
    dbg!(&world.exports);
  }
}