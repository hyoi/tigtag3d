use super::*;

////////////////////////////////////////////////////////////////////////////////

//タイトルロゴ
pub const GAME_TITLE_LOGO: &str = "tigtag3D";

////////////////////////////////////////////////////////////////////////////////

//画面デザイン(枠)
pub struct ScreenFrame<'a>
{   pub design  : Vec<&'a str>,
    pub viewport: ViewPortInfo,
    pub minimap : MiniMapInfo,
}

//3Dカメラの表示領域(viewport)の情報
pub struct ViewPortInfo
{   pub origin: Vec2,
    pub size  : Vec2,
}

//ミニマップの情報
pub struct MiniMapInfo
{   pub zero: IVec2,
    pub size: IVec2,
}

//画面デザイン(枠)
pub const SCREEN_FRAME_SPACE_CHAR : char = ' ';
pub const SCREEN_FRAME_LABEL_REGEX: &str = r"[a-zA-Z0-9\.]+";

pub const VIEWPORT_MINIMAP_ORIGIN: IVec2 = IVec2::new( 33, 13 );
pub const VIEWPORT_MINIMAP_SIZE  : IVec2 = IVec2::new(  9,  9 );

pub static SCREEN_FRAME: Lazy<ScreenFrame> = Lazy::new
(   ||
    {   let design = vec!
        [  //0123456789_123456789_123456789_123456789_12
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
            "#                               #tigtag2D##", //12
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

        if design[ 0 ].len() != SCREEN_GRIDS_WIDTH  as usize
        || design.len()      != SCREEN_GRIDS_HEIGHT as usize
        {   panic!( "APPERR: {}", ER_BAD_SCREEN_DESIGN );
        }

        //3Dカメラの表示領域(viewport)の設定
        let viewport = ViewPortInfo
        {   origin: ( IVec2::new(  1,  1 ).as_vec2() - 0.5 ) * PIXELS_PER_GRID,
            size  : ( IVec2::new( 31, 21 ).as_vec2() + 1.0 ) * PIXELS_PER_GRID,
        };

        //ミニマップの小窓の設定
        let minimap = MiniMapInfo
        {   zero: VIEWPORT_MINIMAP_ORIGIN,
            size: VIEWPORT_MINIMAP_SIZE,
        };

        ScreenFrame { design, viewport, minimap }
    }
);

//エラーメッセージ
const ER_BAD_SCREEN_DESIGN: &str = "Frame design unmatch width/height parameters.";

////////////////////////////////////////////////////////////////////////////////

//glamの型にメソッドを追加する準備
pub trait AddOnTraitForIVec2_2
{   fn to_3dxz( &self ) -> Vec3;
}

//glamの型にメソッドを追加する
impl AddOnTraitForIVec2_2 for IVec2
{   //平面座標(IVec2)から3D直交座標(Vec3)へ変換する
    fn to_3dxz( &self ) -> Vec3
    {   let x = self.x as f32;
        let y = 0.0; //xz平面上
        let z = self.y as f32;
        Vec3::new( x, y, z )
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.