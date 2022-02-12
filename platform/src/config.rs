use roc_std::{RocList, RocStr};

#[derive(Default, Debug)]
#[repr(C)]
// When it comes to passing this around, ownership can get problematic.
// If something wants to borrow one piece of data an something else wants to own a different piece of data,
// rust will still see it as breaking ownership on thist struct.
// There are some tricks for dealing with that and splitting up ownership.
// A simple one is passes parts of the struct around where possible instead of all of it.
pub struct Config {
    pub outputFilePath: RocStr,
    pub points1: RocList<(i32, i32)>,
    pub points2: RocList<(i32, i32)>,
    pub subtitle: RocStr,
    pub title: RocStr,
    pub height: u32,
    pub width: u32,
}

pub fn roc_config() -> Config {
    extern "C" {
        #[link_name = "roc__configForHost_1_exposed_generic"]
        fn call(_: &mut Config);
    }
    let mut config = Config::default();
    unsafe { call(&mut config) };
    config
}
