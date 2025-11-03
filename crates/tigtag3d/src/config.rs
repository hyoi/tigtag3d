use super::*;

////////////////////////////////////////////////////////////////////////////////

// ウィンドウ縦横(Pixel)
pub const SCREEN_PIXELS_RESO: UVec2 =
    UVec2::new(SCREEN_PIXELS_WIDTH as u32, SCREEN_PIXELS_HEIGHT as u32);
pub const SCREEN_PIXELS_WIDTH: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_WIDTH as f32;
pub const SCREEN_PIXELS_HEIGHT: f32 = PIXELS_PER_GRID * SCREEN_GRIDS_HEIGHT as f32;

// ウィンドウ縦横(Grid)
pub const SCREEN_GRIDS_WIDTH: i32 = 43; //memo: 25 best 43
pub const SCREEN_GRIDS_HEIGHT: i32 = 24; //memo: 19 best 24

// マップ縦横幅
pub const MAP_WIDTH_IN_CELLS: i32 = 25; //SCREEN_GRIDS_WIDTH; // w <= SCREEN_GRIDS_WIDTH;
pub const MAP_HEIGHT_IN_CELLS: i32 = 19 - 2; //SCREEN_GRIDS_HEIGHT - 2; // h <= SCREEN_GRIDS_HEIGHT - 2;

// アプリの情報
pub const APP_TITLE: &str = "TigTag3D"; //env!( "CARGO_PKG_NAME" );
pub const APP_VER: &str = env!("CARGO_PKG_VERSION");
pub const COPYRIGHT: &str = "hyoi 2024 - 2025";

//ウィンドウの定義
pub static MAIN_WINDOW: LazyLock<Window> = LazyLock::new(|| {
    Window {
        resolution: SCREEN_PIXELS_RESO.into(), // ウィンドウのサイズ
        resizable: false,                      // リサイズ不可
        decorations: true,                     // タイトルバー表示
        title: format!("{APP_TITLE} v{APP_VER}"), // タイトルバーに表示するタイトル
        enabled_buttons: EnabledButtons {
            minimize: false, // 最小化ボタン非表示
            maximize: false, // 最大化ボタン非表示
            close: true,     // クローズボタン表示
        },
        // fit_canvas_to_parent: true, // v0.13で廃止(#11057)、v0.14で復活(#11278)
        ..default()
    }
});

////////////////////////////////////////////////////////////////////////////////

// カメラの情報を格納するResourceの定義
#[derive(Resource, Deref, DerefMut)]
pub struct CameraSettings(pub Vec<simple_camera::Setting>);

// カメラのComponent
#[derive(Component, Clone)]
pub struct SimpleCamera3dOrbit;

// 2Dカメラの位置
// 第四象限を利用する。左上隅が(0,0)で、X軸はプラス方向へ、Y軸はマイナス方向へ伸びる
pub const CAMERA2D_POSITION: Vec3 = Vec3::new(
    SCREEN_PIXELS_WIDTH * 0.5,
    SCREEN_PIXELS_HEIGHT * -0.5,
    999.0, // 0.0だとスプライトの子のText2dがZ軸1.0(Vec3::Z)で表示されない不具合が発生(v0.14)
);

// 3Dカメラの位置（球座標）
pub const CAMERA3D_POSITION_ORBIT: orbit_camera::Spherical =
    orbit_camera::Spherical {
        r: 21.9,
        theta: PI * 0.5, // 1.0:天頂、0.5:真横、0.0:真下
        phi: TAU * 0.0,  // 時計の6時方向が0.0で反時計回り
    };

// カメラ情報の初期化
impl Default for CameraSettings
{
    fn default() -> Self
    {
        Self(vec![
            simple_camera::Setting::from((
                2,              // カメラのレンダリング優先度（0が最後）
                COLOR_NONE,     // レンダリング時の背景色（NONEは透明）
                SimpleCamera2d, // マーカー（Component）
                Camera2d,       // カメラ種類（Component）
                Transform::from_translation(CAMERA2D_POSITION), // カメラの位置
            )),
            // ★ミニマップカメラ用にレンダリング優先度１を予約
            simple_camera::Setting::from((
                0,
                COLOR_BLACK,
                SimpleCamera3dOrbit,
                Camera3d::default(),
                Transform::from_translation(CAMERA3D_POSITION_ORBIT.into())
                    .looking_at(Vec3::ZERO, Vec3::Y),
            )),
        ])
    }
}

// ★2D ミニマップ用カメラ
pub const CAMERA_ORDER_MINIMAP_2D: isize = 1;

////////////////////////////////////////////////////////////////////////////////

// スプライト重なり
pub const DEPTH_SPRITE_GAME_FRAME: f32 = 800.0; //ゲームの枠のスプライト

////////////////////////////////////////////////////////////////////////////////

// 画面デザイン(枠)の型
pub struct ScreenFrame<'a>
{
    pub design: Vec<&'a str>,
    pub viewport: ViewPortInfo, // 3Dカメラの表示領域(viewport)の情報
    pub minimap: MiniMapInfo,   // ミニマップの情報
}
pub struct ViewPortInfo
{
    pub origin: Vec2,
    pub size: Vec2,
}
pub struct MiniMapInfo
{
    pub zero: IVec2,
    pub size: IVec2,
}

// 画面デザイン(枠)
impl<'a> Default for ScreenFrame<'a>
{
    fn default() -> Self
    {
        let design = vec![
            //123456789_123456789_123456789_123456789_12
            "###########################################", //0
            "#                               ###########", //1
            "#                               ###########", //2
            "#                               ###########", //3
            "#                               ###########", //4
            "#                               ###########", //5
            "#                               ###########", //6
            "#                               ###########", //7
            "#                               ###########", //8
            "#                               ###########", //9
            "#                               ###########", //10
            "#                               ###########", //11
            "#                               #tigtag/2d#", //12
            "#                               #         #", //13
            "#                               #         #", //14
            "#                               #         #", //15
            "#                               #         #", //16
            "#                               #         #", //17
            "#                               #         #", //18
            "#                               #         #", //19
            "#                               #         #", //20
            "#                               #         #", //21
            "###########################################", //22
            "                                           ", //23
        ]; //0123456789_123456789_123456789_123456789_12

        if design[0].len() != SCREEN_GRIDS_WIDTH as usize
            || design.len() != SCREEN_GRIDS_HEIGHT as usize
        {
            panic!("APPERR: {}", ER_BAD_SCREEN_DESIGN);
        }

        //3Dカメラの表示領域(viewport)の設定
        let viewport = ViewPortInfo {
            origin: (IVec2::new(1, 1).as_vec2() - 0.5) * PIXELS_PER_GRID,
            size: (IVec2::new(31, 21).as_vec2() + 1.0) * PIXELS_PER_GRID,
        };

        //ミニマップの小窓の設定
        let minimap = MiniMapInfo {
            zero: VIEWPORT_MINIMAP_ORIGIN,
            size: VIEWPORT_MINIMAP_SIZE,
        };

        ScreenFrame {
            design,
            viewport,
            minimap,
        }
    }
}

pub const SCREEN_FRAME_SPACE_CHAR: char = ' ';
pub const SCREEN_FRAME_LABEL_REGEX: &str = r"[a-zA-Z0-9\.\,\/]+";

pub const VIEWPORT_MINIMAP_ORIGIN: IVec2 = IVec2::new(33, 13);
pub const VIEWPORT_MINIMAP_SIZE: IVec2 = IVec2::new(9, 9);

const ER_BAD_SCREEN_DESIGN: &str = "Frame design unmatch width/height parameters.";

////////////////////////////////////////////////////////////////////////////////

// アジャスタ（マップ座標から画面座標への変換調整値）
// Note: tigtag(2d)の同名のグローバル定数をモジュールの識別子探索パスを利用して置き換える
pub const ADJUST_MAP_ON_SCREEN: IVec2 = IVec2::new(
    VIEWPORT_MINIMAP_SIZE.x + SCREEN_GRIDS_WIDTH,
    VIEWPORT_MINIMAP_SIZE.y + 1,
);

////////////////////////////////////////////////////////////////////////////////

// 3Dライトの設定
pub const SIMPLE_LIGHT3D_BRIGHTNESS: f32 = 3000.0; // 明るさ
pub const SIMPLE_LIGHT3D_POSITION: Vec3 = Vec3::new(-100.0, 300.0, 300.0); //位置

////////////////////////////////////////////////////////////////////////////////

//End of code.
