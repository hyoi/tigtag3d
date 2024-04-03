use super::*;

//ゲームロジック
mod schedule;
pub use schedule::Schedule;

//マップ、自機、追手の処理
mod map;
mod player;
mod chasers;

//tigtagの名前の輸入
mod tigtag;

//End of code.