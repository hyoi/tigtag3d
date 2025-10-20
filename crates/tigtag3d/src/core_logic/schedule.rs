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
            // スケジュールの追加
            // .add_plugins(init_app::Schedule { next: MyState::Initialize } ) // アプリ初期化とアセットロード
            // .add_plugins(demo_play::Schedule)                               // デモプレイ
            // .add_plugins(overlay_ui::pause_menu::Schedule)                  // Pauseメニュー

            // Resourceの登録
            .init_resource::<map::Dots3D>() //3DドットEntityの保存用2次元vec

            // .init_resource::<CameraSettings>()                  // カメラの設定を登録
            // .init_resource::<Record>()                          // ゲームの成績
            // .init_resource::<misc::MaskHitAnyKeyInput>()        // 「Hit Any Key」の入力マスク
            // .init_resource::<map::Map>()                        // ステージのマップ
            // .insert_resource(handle_input::MappingKeyboard::from(KEYBOARD_MAP)) // マッピング
            // .insert_resource(handle_input::MappingGamepad::from(GAMEPAD_MAP))   // マッピング

            // Messageの登録
            // .add_message::<misc::AnyButtonPressed>() //「Hit Any Key」の入力通知
            // .add_message::<SkipOverlayMessage>()     // 全画面メッセージ表示のスキップに使用
            // .add_message::<CountDownEnded>()         // カウントダウンの終了通知
            // .add_message::<handle_input::MessUserAction>() // デバイスからの入力
            // .add_message::<DotsAllEaten >()          // ステージクリアの伝達用
            // .add_message::<DotEaten>()               // スコアリングの伝達用
            // .add_message::<PlayerCaught>()           // ゲームオーバーの伝達用
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
        // 常に実行する処理（Update without MyState）
        // application
        //     // ループ処理
        //     .add_systems(
        //         Update, // without MyState
        //         (
        //             // ヘッダーとフッターの表示情報を更新する
        //             information::update_header_footer,
        //             // スプライトアニメーション
        //             animate_sprites::<player::Player>, // プレイヤー
        //             animate_sprites::<chaser::Chaser>, // チェイサー
        //             // スプライト表示OFFの場合のアニメーション
        //             chaser::rotate_chaser_shape.run_if(SPRITE_OFF), // チェイサー回転
        //         ),
        //     );

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
        // application
        //     // 前処理
        //     .add_systems(
        //         OnEnter(MyState::StageStart),
        //         (
        //             // 全画面メッセージの表示スキップ指示があるなら即メインループへ
        //             misc::set_next_state(MyState::MainLoop)
        //                 .run_if(on_message::<SkipOverlayMessage>),
        //             // 全画面メッセージ（ステージ開始）表示
        //             (
        //                 OverlayStageStart::init(),
        //                 misc::show_component::<OverlayStageStart>
        //                     .after(OverlayStageStart::init()),
        //             )
        //                 .run_if(not(on_message::<SkipOverlayMessage>)),
        //             // ステージ初期化
        //             map::make_new_stage_data, // マップデータ
        //             (
        //                 map::spawn_sprite,    // マップスプライト
        //                 player::spawn_sprite, // プレーヤースプライト
        //                 chaser::spawn_sprite, // チェイサースプライト
        //             )
        //                 .after(map::make_new_stage_data),
        //         ),
        //     )
        //     // ループ処理
        //     .add_systems(
        //         Update, // within MyState::StageStart
        //         (
        //             // カウントダウン完了後にState遷移
        //             overlay_ui::effect::countdown::<OverlayStageStart>,
        //             misc::set_next_state(MyState::MainLoop)
        //                 .run_if(on_message::<CountDownEnded>)
        //                 .after(overlay_ui::effect::countdown::<OverlayStageStart>),
        //         )
        //             .run_if(in_state(MyState::StageStart)),
        //     )
        //     // 後処理
        //     .add_systems(
        //         OnExit(MyState::StageStart),
        //         (
        //             // 全画面メッセージ（ステージ開始）非表示
        //             misc::hide_component::<OverlayStageStart>,
        //         ),
        //     );

        // --------------------------------------------------------------------------
        // メインループ処理（MyState::MainLoop）
        // application
        //     // ループ処理
        //     .add_systems(
        //         Update, // within MyState::MainLoop
        //         (
        //             (
        //                 // スプライトの位置を更新する
        //                 handle_input::check_keyboard, // キー
        //                 handle_input::check_gamepad,  // ゲームパッド
        //                 player::move_sprite
        //                     .after(handle_input::check_keyboard)
        //                     .after(handle_input::check_gamepad)
        //                     .run_if(on_message::<handle_input::MessUserAction>),
        //                 chaser::move_sprite,
        //             ),
        //             // スコアリング＆クリア判定
        //             detecting_change::scoring_and_stage_clear,
        //             misc::set_next_state(MyState::StageClear)
        //                 .run_if(on_message::<DotsAllEaten>),
        //             // 衝突判定
        //             detecting_change::collisions_and_gameover
        //                 // DotsAllEaten ➡ スキップ
        //                 .run_if(not(on_message::<DotsAllEaten>)),
        //             misc::set_next_state(MyState::GameOver)
        //                 .run_if(on_message::<PlayerCaught>),
        //         )
        //             .chain()
        //             .run_if(in_state(MyState::MainLoop)),
        //     );

        //--------------------------------------------------------------------------
        // ステージクリアの処理（MyState::StageClear）
        // application
        //     // 前処理
        //     .add_systems(
        //         OnEnter(MyState::StageClear),
        //         (
        //             // 全画面メッセージ（ステージクリア）表示
        //             OverlayStageClear::init(),
        //             misc::show_component::<OverlayStageClear>
        //                 .after(OverlayStageClear::init()),
        //         ),
        //     )
        //     // ループ処理
        //     .add_systems(
        //         Update, // within MyState::StageClear
        //         (
        //             // カウントダウン完了後にState遷移
        //             overlay_ui::effect::countdown::<OverlayStageClear>,
        //             misc::set_next_state(MyState::StageStart)
        //                 .run_if(on_message::<CountDownEnded>)
        //                 .after(overlay_ui::effect::countdown::<OverlayStageClear>),
        //         )
        //             .run_if(in_state(MyState::StageClear)),
        //     )
        //     // 後処理
        //     .add_systems(
        //         OnExit(MyState::StageClear),
        //         (
        //             // 全画面メッセージ（ステージクリア）非表示
        //             misc::hide_component::<OverlayStageClear>,
        //             // 後続の MyState::StageStart で全画面メッセージを表示しない
        //             misc::set_message::<SkipOverlayMessage>,
        //         ),
        //     );

        //--------------------------------------------------------------------------
        // ゲームオーバーの処理（MyState::GameOver）
        // application
        //     // 前処理
        //     .add_systems(
        //         OnEnter(MyState::GameOver),
        //         (
        //             // 全画面メッセージ（ステージクリア）表示
        //             OverlayGameOver::init(),
        //             misc::show_component::<OverlayGameOver>
        //                 .after(OverlayGameOver::init()),
        //         ),
        //     )
        //     // ループ処理
        //     .add_systems(
        //         Update, // within MyState::GameOver
        //         (
        //             // カウントダウン完了後にState遷移
        //             overlay_ui::effect::countdown::<OverlayGameOver>,
        //             misc::set_next_state(MyState::TitleDemo)
        //                 .run_if(on_message::<CountDownEnded>)
        //                 .after(overlay_ui::effect::countdown::<OverlayGameOver>),
        //             // Replay? の明滅
        //             overlay_ui::effect::blinking_text::<OverlayGameOver>,
        //             // Hit ANY Key に反応あればState遷移
        //             misc::check_hit_any_key
        //                 .in_set(misc::execution_order::Target::HitAnyKey),
        //             misc::set_next_state(MyState::StageStart)
        //                 .in_set(misc::execution_order::After::HitAnyKey)
        //                 .run_if(on_message::<misc::AnyButtonPressed>),
        //         )
        //             .run_if(in_state(MyState::GameOver)),
        //     )
        //     // 後処理
        //     .add_systems(
        //         OnExit(MyState::GameOver),
        //         (
        //             // 全画面メッセージ（ステージクリア）非表示
        //             misc::hide_component::<OverlayGameOver>,
        //             // scoreとstageをゼロクリアする
        //             detecting_change::initialize_score_stage,
        //         ),
        //     );
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
    let x = (tigtag2d::core_logic::map::MAP_WIDTH_IN_CELLS - 1) as f32 * 0.5;
    let neg_y = (tigtag2d::core_logic::map::MAP_HEIGHT_IN_CELLS - 1) as f32 * -0.5;
    let look_at = Vec3::new(x, neg_y, 0.0);
    let vec3 = Vec3::Z * 20.25 + look_at;
    let transform = Transform::from_translation(vec3);
    cmds.entity(entity).insert(transform);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// End of code.
