use super::*;

//ゲームロジック
mod schedule;
pub use schedule::Schedule;

//play_game内共通
pub mod common; //外部公開
use common::*;

//マップ、自機、追手の処理
mod map;
mod player;
mod chasers;

//tigtagの名前の輸入
mod tigtag;

//End of code.