//external crates
use bevy::
{   prelude::*,
    log::LogPlugin,
    color::palettes::css,
    window::WindowMode,
    input::mouse::{ MouseMotion, MouseWheel },
    ecs::query::QueryFilter,
    asset::{ LoadState, LoadedUntypedAsset },
    diagnostic::{ FrameTimeDiagnosticsPlugin, DiagnosticsStore },
    utils::Duration,
    input::keyboard::NativeKeyCode,
    sprite::{ MaterialMesh2dBundle, Anchor },
    utils::{ HashMap, HashSet },
    audio::Volume,
    render::camera::Viewport,
};
use bevy_dev_tools::ui_debug_overlay; //UI Node Outline Gizmos

use rand::prelude::*;
use chrono::prelude::Local as time_local; //「Local」がbevyとバッティングするのでaliasを使う
use regex::Regex;

//standard library
use std::
{   sync::LazyLock,
    f32::consts::{ PI, TAU },
    ops::{ Range, Add, AddAssign },
    cmp::Ordering,
    collections::VecDeque,
};

//proc-macro crates
use macros::MyState;

//internal submodules
mod template;
use template::*;

//ゲームロジック
mod tigtag_inside;
mod tigtag3d_inside;

////////////////////////////////////////////////////////////////////////////////

//アプリの情報
pub const APP_TITLE: &str = "TigTag3D"; //env!( "CARGO_PKG_NAME" );
pub const APP_VER  : &str = env!( "CARGO_PKG_VERSION" );
pub const COPYRIGHT: &str = "hyoi 2024 - XXXX";

//ウィンドウ縦横(Grid)
pub const SCREEN_GRIDS_WIDTH : i32 = 43; //memo: 25 best 43
pub const SCREEN_GRIDS_HEIGHT: i32 = 24; //memo: 19 best 24

//コンパイル オプションの定数
pub const SPRITE_OFF: fn() -> bool = || cfg!( feature = "sprite_off" );

////////////////////////////////////////////////////////////////////////////////

//メイン関数
fn main()
{   //アプリの生成
    let mut app = App::new();

    //メイン処理
    app
    .add_plugins( template::Schedule        ) //アプリの雛型
    .add_plugins( tigtag_inside::Schedule   ) //tigtagのゲームロジック
    .add_plugins( tigtag3d_inside::Schedule ) //3Dビジュアライザ
    ;

    //アプリの実行
    app.run();
}

////////////////////////////////////////////////////////////////////////////////

//End of code.