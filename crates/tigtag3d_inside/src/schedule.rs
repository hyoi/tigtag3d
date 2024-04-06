use super::*;

////////////////////////////////////////////////////////////////////////////////

//プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{   fn build( &self, app: &mut App )
    {   app

        //Resource
        .init_resource::<map::Dots3D>() //3DドットEntityの保存用2次元vec

        ////////////////////////////////////////////////////////////////////////
        //ゲーム初期化
        .add_systems
        (   OnEnter ( MyState::InitGame ),
            (   //ゲーム枠を表示
                spawn_screen_frame,

                //3Dカメラの作り直し
                misc::despawn_component::<misc::CameraDefault3d>,
                spawn_camera3d_with_viewport,

                //タイトルロゴを書き換える
                set_title_logo
                    .after( tigtag::spawn_text_title_demo )
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //タイトル画面
        .add_systems
        (   OnEnter ( MyState::TitleDemo ),
            (   //3Dマップをspawnする
                map::spawn_3d_map_entity
                    .after( tigtag::make_new_data_map ),

                //3D自キャラをspawnする
                player::spawn_3d_player
                    .after( tigtag::spawn_sprite_player ),

                //3D敵キャラをspawnする
                chasers::spawn_3d_chasers
                    .after( tigtag::spawn_sprite_chasers ),

                //ミニマップ用カメラをspawnする(2D自キャラの子にする)
                player::spawn_minimap_camera
                    .after( tigtag::spawn_sprite_player ),
            )
        )
        .add_systems
        (   Update,
            (   //3D表示を更新する
                map::update_3d_map          //マップのドットが消える処理
                    .run_if( on_event::<tigtag::EventEatDot>() ),
                player::update_3d_player,   //自キャラの移動
                chasers::update_3d_chasers, //敵キャラの移動
            )
            .run_if( in_state( MyState::TitleDemo ) )
        )

        ////////////////////////////////////////////////////////////////////////
        //ステージ初期化
        .add_systems
        (   OnEnter ( MyState::StageStart ),
            (   //3Dマップをspawnする
                map::spawn_3d_map_entity
                    .after( tigtag::make_new_data_map ),

                //3D自キャラをspawnする
                player::spawn_3d_player
                    .after( tigtag::spawn_sprite_player ),

                //3D敵キャラをspawnする
                chasers::spawn_3d_chasers
                    .after( tigtag::spawn_sprite_chasers ),

                //ミニマップ用カメラをspawnする(2D自キャラの子にする)
                player::spawn_minimap_camera
                    .after( tigtag::spawn_sprite_player ),
            )
        )

        ////////////////////////////////////////////////////////////////////////
        //メインループ
        .add_systems
        (   Update,
            (   //3D表示を更新する
                map::update_3d_map          //マップのドットが消える処理
                    .run_if( on_event::<tigtag::EventEatDot>() ),
                player::update_3d_player,   //自キャラの移動
                chasers::update_3d_chasers, //敵キャラの移動
            )
            .run_if( in_state( MyState::MainLoop ) )
        )

        ////////////////////////////////////////////////////////////////////////
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームの枠を表示する
fn spawn_screen_frame
(   mut cmds : Commands,
    asset_svr: Res<AssetServer>,
)
{   let custom_size = Some ( GRID_CUSTOM_SIZE );
    let alpha = if DEBUG() { 0.5 } else { 1.0 }; //DEBUG時に透過させる
    let color = Color::rgba( 1.0, 1.0, 1.0, alpha );
    let regex = Regex::new( SCREEN_FRAME_LABEL_REGEX ).unwrap();
    let adjust = Vec2::X * PIXELS_PER_GRID / 2.0;

    for ( y, line ) in SCREEN_FRAME.design.iter().enumerate()
    {   //レンガのスプライトを敷き詰める
        for ( x, char ) in line.chars().enumerate()
        {   if char == SCREEN_FRAME_SPACE_CHAR { continue }

            let vec2 = IVec2::new( x as i32, y as i32 ).to_vec2_on_screen();
            let vec3 = vec2.extend( DEPTH_SPRITE_GAME_FRAME );

            cmds.spawn( SpriteBundle::default() )
            .insert( Sprite { custom_size, color, ..default() } )
            .insert( Transform::from_translation( vec3 ) )
            .insert( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) as Handle<Image> )
            ;
        }

        //ラベル文字列があるなら
        for m in regex.find_iter( line )
        {   let value = m.as_str().to_string();
            let style = TextStyle
            {   font     : asset_svr.load( ASSETS_FONT_PRESSSTART2P_REGULAR ),
                font_size: PIXELS_PER_GRID,
                color    : Color::SILVER,
            };
            let sections = vec![ TextSection { value, style } ];
            let vec2 = IVec2::new( m.start() as i32, y as i32 ).to_vec2_on_screen() - adjust;
            let vec3 = vec2.extend( DEPTH_SPRITE_GAME_FRAME + 1.0 );

            cmds.spawn( Text2dBundle::default() )
            .insert( Text { sections, ..default() } )
            .insert( Anchor::CenterLeft )
            .insert( Transform::from_translation( vec3 ) )
            ;
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//viewport付き3Dカメラをspawnする
fn spawn_camera3d_with_viewport
(   mut cmds: Commands,
)
{   //Viewport
    let viewport = Some
    (   Viewport
        {   physical_position: SCREEN_FRAME.viewport.origin.as_uvec2(),
            physical_size    : SCREEN_FRAME.viewport.size  .as_uvec2(),
            ..default()
        }
    );

    //カメラの注視点をマップ中央の3D座標とする
    let x = ( tigtag::MAP_GRIDS_WIDTH  - 1 ) as f32 * 0.5;
    let z = ( tigtag::MAP_GRIDS_HEIGHT - 1 ) as f32 * 0.5;
    let look_at = Vec3::new( x, 0.0, z );

    //3Dカメラ
    let camera = Camera
    {   order: CAMERA_ORDER_DEFAULT_3D,
        clear_color: CAMERA_BGCOLOR_3D,
        viewport,
        ..default()
    };
    let vec3 = Orbit::default().to_vec3() + look_at;
    let transform = Transform::from_translation( vec3 );

    //カメラをspawnする
    cmds.spawn( Camera3dBundle:: default() )
    .insert( camera )
    .insert( transform.looking_at( look_at, Vec3::Y ) )
    ;
}

////////////////////////////////////////////////////////////////////////////////

//タイトルロゴを書き換える
fn set_title_logo
(   mut qry_text: Query<&mut Text, With<tigtag::TextTitleLogo>>,
)
{   let Ok ( mut text ) = qry_text.get_single_mut() else { return };
    text.sections[ 0 ].value = APP_TITLE.to_string();
}

////////////////////////////////////////////////////////////////////////////////

//End of code.