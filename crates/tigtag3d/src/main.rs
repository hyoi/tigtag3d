//external crates
use bevy::
{   prelude::*,
    input::keyboard::NativeKeyCode,
    sprite::MaterialMesh2dBundle,
    utils::{ HashMap, HashSet },
    audio::Volume,
};
use rand::prelude::*;

//standard library
use std::
{   sync::LazyLock,
    ops::Range,
    f32::consts::{ PI, TAU },
    ops::{ Add, AddAssign },
    cmp::Ordering,
    collections::VecDeque,
};

//other crates in this package
use template::*;

//internal submodules
mod tigtag_inside;
mod tigtag3d_inside;

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