// external crates
use bevy::{
    prelude::*,
    ecs::{error::warn, system::SystemParam, component::Mutable},
    log::LogPlugin,
    diagnostic::{FrameTimeDiagnosticsPlugin, DiagnosticsStore},
    window::{EnabledButtons, WindowMode},
    input::{
        keyboard::NativeKeyCode,
        gamepad::GamepadInput,
        mouse::{MouseMotion, MouseWheel},
    },
    asset::{LoadedUntypedAsset, LoadState},
    color::palettes::*,
    camera::Viewport,
    sprite::Anchor,
    audio::Volume,
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
use std::{
    slice::Iter,
    ops::{Range, Deref, DerefMut, Add, AddAssign},
    f32::consts::{PI, TAU},
    collections::VecDeque,
};

// internal submodules
mod core_logic; // 3Dビジュアライザ
mod tigtag2d; // ゲームロジック

mod my_utils; // 共通ライブラリ
use my_utils::prelude::*;

mod config; // 設定各種
use config::*;

// proc-macro
use macros::MyState;
use macros::derive_appctrl_input;
use macros::{OverlayMessage, Blinking, CountDown};
use macros::{OverlayMenu, ScalingItem};

////////////////////////////////////////////////////////////////////////////////

// メイン関数
fn main() -> AppExit
{
    // アプリの生成
    App::new()
        // メインスケジュール
        .add_plugins(core_logic::Schedule) //3Dビジュアライザ
        .add_plugins(tigtag2d::Schedule) //tigtagのゲームロジック
        // アプリ実行
        .run()
}

////////////////////////////////////////////////////////////////////////////////

//End of code.
