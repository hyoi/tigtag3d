use super::*;

////////////////////////////////////////////////////////////////////////////////

// プラグインの設定
pub struct Schedule;
impl Plugin for Schedule
{
    fn build(&self, application: &mut App)
    {
        //--------------------------------------------------------------------------
        // 各種登録
        application
            // Resourceの登録
            .init_resource::<map::Dots3D>() //3DドットEntityの保存用2次元vec
            ;

        //--------------------------------------------------------------------------
        // 初期化（MyState::Initialize）
        application
            // 後処理
            .add_systems(
                OnExit(MyState::Initialize),
                (
                    // ゲーム画面の枠を表示
                    spawn_screen_frame,
                    // 3Dライトをspawnする
                    spawn_simple_light3d,
                    // 既存の3Dカメラにviewportをセット、位置と注視点を変更する
                    change_camera3d_settings,
                ),
            );

        //--------------------------------------------------------------------------
        // タイトル画面の処理（MyState::TitleDemo）
        application
            // 前処理
            .add_systems(
                OnEnter(MyState::TitleDemo),
                (
                    // 3Dのマップをspawnする
                    map::spawn_3d_map_entity
                        .after(tigtag2d::core_logic::map::make_new_stage_data),
                    // 3Dのプレイヤーをspawnする
                    player::spawn_3d_player
                        .after(tigtag2d::core_logic::player::spawn_sprite),
                    // 3Dのチェイサーをspawnする
                    chaser::spawn_3d_chasers
                        .after(tigtag2d::core_logic::chaser::spawn_sprite),
                    // ミニマップカメラをspawnする(2Dプレイヤーの子にする)
                    player::spawn_minimap_camera
                        .after(tigtag2d::core_logic::player::spawn_sprite),
                ),
            )
            // ループ処理
            .add_systems(
                Update, // within MyState::TitleDemo
                (
                    // 3Dマップのドットを消す処理
                    map::update_3d_map
                        .run_if(on_message::<tigtag2d::core_logic::DotEaten>),
                    // 3Dプレイヤーの移動
                    player::update_3d_player,
                    // 3Dチェイサーの移動
                    chaser::update_3d_chasers,
                )
                    .run_if(in_state(MyState::TitleDemo)),
            );

        //--------------------------------------------------------------------------
        // ゲーム開始処理（MyState::StageStart）
        application
            // 前処理
            .add_systems(
                OnEnter(MyState::StageStart),
                (
                    // 3Dのマップをspawnする
                    map::spawn_3d_map_entity
                        .after(tigtag2d::core_logic::map::make_new_stage_data),
                    // 3Dのプレイヤーをspawnする
                    player::spawn_3d_player
                        .after(tigtag2d::core_logic::player::spawn_sprite),
                    // 3Dのチェイサーをspawnする
                    chaser::spawn_3d_chasers
                        .after(tigtag2d::core_logic::chaser::spawn_sprite),
                    // ミニマップカメラをspawnする(2Dプレイヤーの子にする)
                    player::spawn_minimap_camera
                        .after(tigtag2d::core_logic::player::spawn_sprite),
                ),
            );

        // --------------------------------------------------------------------------
        // メインループ処理（MyState::MainLoop）
        application
            // ループ処理
            .add_systems(
                Update,
                (
                    // 3Dマップのドットを消す処理
                    map::update_3d_map
                        .run_if(on_message::<tigtag2d::core_logic::DotEaten>),
                    // 3Dプレイヤーの移動
                    player::update_3d_player,
                    // 3Dチェイサーの移動
                    chaser::update_3d_chasers,
                )
                    .run_if(in_state(MyState::MainLoop)),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////

//ゲームの枠を表示する
fn spawn_screen_frame(mut cmds: Commands, asset_svr: Res<AssetServer>)
{
    let custom_size = Some(CELL_CUSTOM_SIZE);
    let alpha = if misc::DEBUG() { 0.5 } else { 1.0 }; //DEBUG時に透過させる
    let color = Color::srgba(1.0, 1.0, 1.0, alpha);
    let regex = Regex::new(SCREEN_FRAME_LABEL_REGEX).unwrap();
    let adjust = Vec2::X * PIXELS_PER_GRID / 2.0;

    for (y, line) in ScreenFrame::default().design.iter().enumerate()
    {
        //レンガのスプライトを敷き詰める
        for (x, char) in line.chars().enumerate()
        {
            if char == SCREEN_FRAME_SPACE_CHAR
            {
                continue;
            }

            let vec2 = IVec2::new(x as i32, y as i32).to_screen_pixels();
            let vec3 = vec2.extend(DEPTH_SPRITE_GAME_FRAME);

            cmds.spawn((
                Sprite {
                    image: asset_svr.load(ASSETS_SPRITE_BRICK_WALL),
                    custom_size,
                    color,
                    ..default()
                },
                Transform::from_translation(vec3),
            ));
        }

        //ラベル文字列があるなら
        for m in regex.find_iter(line)
        {
            let value = m.as_str().to_string();
            let vec2 =
                IVec2::new(m.start() as i32, y as i32).to_screen_pixels() - adjust;
            let vec3 = vec2.extend(DEPTH_SPRITE_GAME_FRAME + 1.0);

            cmds.spawn((
                Text2d::new(value),
                TextFont {
                    font: asset_svr.load(ASSETS_FONT_PRESSSTART2P_REGULAR),
                    font_size: PIXELS_PER_GRID,
                    ..default()
                },
                TextColor(COLOR_GOLD),
                Transform::from_translation(vec3),
                Anchor::CENTER_LEFT,
            ));
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

// 3Dライトをspawnする
pub fn spawn_simple_light3d(mut cmds: Commands)
{
    cmds.spawn((
        DirectionalLight {
            illuminance: SIMPLE_LIGHT3D_BRIGHTNESS,
            shadows_enabled: true,
            ..default()
        },
        Transform::from_translation(SIMPLE_LIGHT3D_POSITION)
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

////////////////////////////////////////////////////////////////////////////////

// 3Dカメラにviewportをセット、位置と注視点を変更する
fn change_camera3d_settings(
    mut query_camera3d: Query<(Entity, &mut Camera), With<SimpleCamera3dOrbit>>,
    mut cmds: Commands,
) -> Result
{
    // 準備
    let (entity, mut camera) = query_camera3d.single_mut()?;

    // viewportをセットする
    let screen_frame = ScreenFrame::default();
    let viewport = Viewport {
        physical_position: screen_frame.viewport.origin.as_uvec2(),
        physical_size: screen_frame.viewport.size.as_uvec2(),
        ..default()
    };
    camera.viewport = Some(viewport);

    // カメラの位置と注視点を変更する
    let x = (MAP_WIDTH_IN_CELLS - 1) as f32 * 0.5;
    let neg_y = (MAP_HEIGHT_IN_CELLS - 1) as f32 * -0.5;
    let look_at = Vec3::new(x, neg_y, 0.0);
    let vec3 = Vec3::Z * 20.25 + look_at;
    let transform = Transform::from_translation(vec3);
    cmds.entity(entity).insert(transform);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
