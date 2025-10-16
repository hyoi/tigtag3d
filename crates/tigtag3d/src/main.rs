// external crates
use bevy::{
    prelude::*,
    // log::LogPlugin,
    // color::palettes::css,
    // window::WindowMode,
    // render::camera::Viewport,
    // input::mouse::{ MouseMotion, MouseWheel },
    // ecs::query::QueryFilter,
    // asset::{ LoadState, LoadedUntypedAsset },
    // diagnostic::{ FrameTimeDiagnosticsPlugin, DiagnosticsStore },
    // utils::Duration,
    // dev_tools::ui_debug_overlay,
    // input::keyboard::NativeKeyCode,
    // sprite::{ MaterialMesh2dBundle, Anchor },
    // utils::{ HashMap, HashSet },
    // audio::Volume,
};

// use rand::prelude::*;
// use chrono::prelude::Local as time_local; //「Local」がbevyとバッティングするのでaliasを使う
// use regex::Regex;

// standard library
// use std::
// {
//     // sync::LazyLock,
//     // f32::consts::{ PI, TAU },
//     // ops::{ Range, Add, AddAssign },
//     // cmp::Ordering,
//     // collections::VecDeque,
// };

// internal submodules
// mod core_logic; // ゲームロジック

// mod my_utils; // 共通ライブラリ
// use my_utils::prelude::*;

// mod config; // 設定各種
// use config::*;

// mod demo_play; // demoロジック

// proc-macro
// use macros::MyState;

// mod template;
// use template::*;

// ゲームロジック
// mod tigtag_inside;
// mod tigtag3d_inside;

////////////////////////////////////////////////////////////////////////////////

//アプリの情報
// pub const APP_TITLE: &str = "TigTag3D"; //env!( "CARGO_PKG_NAME" );
// pub const APP_VER  : &str = env!( "CARGO_PKG_VERSION" );
// pub const COPYRIGHT: &str = "hyoi 2024 - XXXX";

//ウィンドウ縦横(Grid)
// pub const SCREEN_GRIDS_WIDTH : i32 = 43; //memo: 25 best 43
// pub const SCREEN_GRIDS_HEIGHT: i32 = 24; //memo: 19 best 24

//コンパイル オプションの定数
// pub const SPRITE_OFF     : fn() -> bool = || cfg!( feature = "sprite_off"      );
// pub const ATTACH_VIEWPORT: fn() -> bool = || cfg!( feature = "attach_viewport" );

////////////////////////////////////////////////////////////////////////////////

// メイン関数
fn main() -> AppExit
{
    // アプリの生成
    App::new()
        // メインスケジュール
        // .add_plugins(core_logic::Schedule)
        // アプリ実行
        .run()
}

//メイン関数
// fn main() -> AppExit
// {
//     //アプリの生成
//     let mut app = App::new();

//     //メイン処理
//     // app
//     // .add_plugins( template::Schedule        ) //アプリの雛型
//     // .add_plugins( tigtag_inside::Schedule   ) //tigtagのゲームロジック
//     // .add_plugins( tigtag3d_inside::Schedule ) //3Dビジュアライザ
//     // ;

//     //アプリの実行
//     app.run()
// }

////////////////////////////////////////////////////////////////////////////////

//End of code.
