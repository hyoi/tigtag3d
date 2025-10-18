// external crates
use bevy::{
    prelude::*,
    ecs::{error::warn, system::SystemParam, component::Mutable},
    log::LogPlugin,
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
    window::{EnabledButtons, WindowMode},
    input::{
        // keyboard::NativeKeyCode,
        gamepad::GamepadInput,
        mouse::{MouseMotion, MouseWheel},
    },
    asset::{LoadedUntypedAsset, LoadState},
    color::palettes::*,
    camera::Viewport,
    sprite::Anchor,
    // audio::Volume,
    // ecs::query::QueryFilter,
    // utils::Duration,
    // dev_tools::ui_debug_overlay,
    // utils::{ HashMap, HashSet },
};

use rustc_hash::{FxHashSet, FxHashMap};
use rand::prelude::*;
use regex::Regex;
// use chrono::prelude::Local as time_local; //「Local」がbevyとバッティングするのでaliasを使う

// standard library
use std::
{
    slice::Iter,
    ops::{ Range, Deref, DerefMut, Add, AddAssign},
    f32::consts::{ PI, TAU },
    // sync::LazyLock,
    // cmp::Ordering,
    // collections::VecDeque,
};

// internal submodules
mod core_logic; // 3Dビジュアライザ
mod tigtag2d;   // ゲームロジック

mod my_utils; // 共通ライブラリ
use my_utils::prelude::*;

mod config; // 設定各種
use config::*;

// mod demo_play; // demoロジック

// proc-macro
use macros::MyState;
use macros::derive_appctrl_input;
use macros::{OverlayMessage, Blinking, CountDown};
use macros::{OverlayMenu, ScalingItem};

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
        .add_plugins(core_logic::Schedule) //3Dビジュアライザ
        .add_plugins(tigtag2d::Schedule)   //tigtagのゲームロジック
        // アプリ実行
        .run()
}

//メイン関数
// fn main() -> AppExit
// {
//     //アプリの生成
//     let mut app = App::new();

//     //メイン処理
//     app
//     .add_plugins( template::Schedule        ) //アプリの雛型
//     .add_plugins( tigtag_inside::Schedule   ) //tigtagのゲームロジック
//     .add_plugins( tigtag3d_inside::Schedule ) //3Dビジュアライザ
//     ;

//     //アプリの実行
//     app.run()
// }

////////////////////////////////////////////////////////////////////////////////

//End of code.
