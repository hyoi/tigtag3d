#![allow(dead_code)]
use super::*;

pub mod core_logic; // ゲームロジック(tigtag)
pub use core_logic::Schedule;

mod demo_play; // demoロジック(tigtag)

mod config; // 設定各種
pub mod config_prelude
{
    pub use super::config::prelude::
    {
        PIXELS_PER_GRID,

        LOG_FILTER_DEVELOP,
        LOG_FILTER_RELEASE,

        MyState,

        PRELOAD_ASSETS,
        ASSETS_FONT_ORBITRON_BLACK,
        ASSETS_FONT_PRESSSTART2P_REGULAR,
        ASSETS_FONT_REGGAEONE_REGULAR,
        ASSETS_SPRITE_KANI_DOTOWN,
        ASSETS_SPRITE_BRICK_WALL,
        ASSETS_SPRITESHEET_PLAYER,
        ASSETS_SPRITESHEET_CHASER_RED,
        ASSETS_SPRITESHEET_CHASER_GREEN,
        ASSETS_SPRITESHEET_CHASER_BLUE,
        ASSETS_SPRITESHEET_CHASER_PINK,
        ASSETS_SOUND_BEEP,
        VOLUME_SOUND_BEEP,

        SimpleCamera2d,

        ATTACH_VIEWPORT,
        SPRITE_OFF,

        DEPTH_SPRITE_KANI_DOTOWN,
        DEPTH_SPRITE_CHASER,
        DEPTH_SPRITE_PLAYER,
        DEPTH_SPRITE_DOT,
        DEPTH_SPRITE_BRICK_WALL,

        SPRITE_DOT_RADIUS,
        SPRITE_DOT_COLOR,

        KEYBOARD_MAP,
        GAMEPAD_MAP,

        HEADER_FOOTER,
        SPRITE_KANI_GRID_X,
        SPRITE_KANI_GRID_Y,
        SPRITE_KANI_MAGNIFY,
        SPRITE_KANI_ALPHA,
        NUM3_2,
        FOOTER_FPS,

        OverlayTitleDemo,
        OverlayStageStart,
        OverlayStageClear,
        OverlayGameOver,
        OverlayPauseMenu,
        PAUSE_MENU_BG_COLOR,
        PAUSE_SELECTED_COLOR,
        PAUSE_NORMAL_COLOR,
    };
}

// End of code.
