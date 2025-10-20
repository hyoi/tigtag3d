use super::*;

////////////////////////////////////////////////////////////////////////////////

// 3DプレイヤーのComponent
#[derive(Component)]
pub struct Player3d;

// ミニマップ用2DカメラのComponent
#[derive(Component)]
pub struct MinimapCamera;

// 3Dプレイヤーの定数
const PLAYER_3D_RADIUS: f32 = 0.45;
const PLAYER_3D_COLOR: Color = COLOR_YELLOW;

// 速度(1.0は3D空間での1グリッドの距離)
const PLAYER_SPEED: f32 = 1.0 / tigtag2d::core_logic::player::Player::TIME_PER_GRID;

////////////////////////////////////////////////////////////////////////////////

// 3Dプレイヤーをspawnする
pub fn spawn_3d_player(
    query_player: Query<&tigtag2d::core_logic::player::Player>,
    query_entity: Query<Entity, With<Player3d>>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result
{
    // 準備
    let player = query_player.single()?;

    // 既存のEntityがあれば削除する
    query_entity.iter().for_each(|id| cmds.entity(id).despawn());

    // 3Dプレイヤーをspawnする
    let vec3 = IVec3::new(player.cell.x, -player.cell.y, 0).as_vec3();

    cmds.spawn((
        Mesh3d(meshes.add(Sphere::new(PLAYER_3D_RADIUS).mesh().uv(36, 18))),
        MeshMaterial3d(materials.add(PLAYER_3D_COLOR)),
        Transform::from_translation(vec3),
        Player3d,
    ));

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// プレイヤーの子Entityとしてミニマップ用2Dカメラをspawnする
pub fn spawn_minimap_camera(
    player: Query<Entity, With<tigtag2d::core_logic::player::Player>>,
    mut cmds: Commands,
) -> Result
{
    // 準備
    let player_id = player.single()?;

    // ミニマップ用2Dカメラをspawnする
    let screen_frame = ScreenFrame::default();
    let zero = screen_frame.minimap.zero.as_vec2() * PIXELS_PER_GRID;
    let size = screen_frame.minimap.size.as_vec2() * PIXELS_PER_GRID;
    let viewport = Viewport {
        physical_position: zero.as_uvec2(),
        physical_size: size.as_uvec2(),
        ..default()
    };
    let order = CAMERA_ORDER_MINIMAP_2D;
    let child = cmds
        .spawn((
            Camera {
                viewport: Some(viewport),
                order,
                clear_color: COLOR_NONE.into(),
                ..default()
            },
            Camera2d,
            MinimapCamera,
        ))
        .id();
    cmds.entity(player_id).add_child(child);

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

// プレイヤーの表示を更新する
pub fn update_3d_player(
    mut query_transform: Query<&mut Transform, With<Player3d>>,
    query_player: Query<&tigtag2d::core_logic::player::Player>,
    time: Res<Time>,
    message_timer: MessageReader<tigtag2d::core_logic::PlayerPositionAdjusted>,
) -> Result
{
    // 準備
    let mut transform = query_transform.single_mut()?;
    let player = query_player.single()?;

    // 前回からの経過時間にスピードアップ係数をかける
    let time_delta = time.delta().mul_f32(player.speedup);

    // tigtag(2d)からプレイヤーの移動タイマーのfinishedが通知されたなら
    if !message_timer.is_empty()
    {
        let vec3 = IVec3::new(player.cell.x, -player.cell.y, 0).as_vec3();
        transform.translation = vec3;
    }
    else if !player.is_stop
    {
        // 移動中の中割座標
        let delta = time_delta.as_secs_f32() * PLAYER_SPEED;
        match player.direction
        {
            tigtag2d::core_logic::News::North => transform.translation.y += delta,
            tigtag2d::core_logic::News::South => transform.translation.y -= delta,
            tigtag2d::core_logic::News::East => transform.translation.x += delta,
            tigtag2d::core_logic::News::West => transform.translation.x -= delta,
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

//End of code.
