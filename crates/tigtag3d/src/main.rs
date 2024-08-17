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