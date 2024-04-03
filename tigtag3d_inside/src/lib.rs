//external crates
use bevy::
{   prelude::*,
    utils::{ HashMap, HashSet },
    sprite::Anchor,
    render::camera::Viewport,
};
use regex::Regex;

//import names from other crates in this package
use public::*;
use tigtag_inside::prelude as tigtag;

//ゲームロジック
mod schedule;
pub use schedule::Schedule;

//マップ、自機、追手の処理
mod map;
mod player;
mod chasers;

//End of code.