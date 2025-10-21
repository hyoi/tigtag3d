use super::*;

////////////////////////////////////////////////////////////////////////////////

// 3DチェイサーのComponent
#[derive(Component)]
pub struct Chaser3d(Color);

// 3Dチェイサーの定数
const CHASER_3D_RADIUS: f32 = 0.45;

// 速度(1.0は3D空間での1グリッドの距離)
const CHASER_SPEED: f32 = 1.0 / tigtag2d::core_logic::chaser::Chaser::TIME_PER_GRID;

////////////////////////////////////////////////////////////////////////////////

// 3Dチェイサーをspawnする
pub fn spawn_3d_chasers(
    query_chasers: Query<&tigtag2d::core_logic::chaser::Chaser>,
    query_entity: Query<Entity, With<Chaser3d>>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    // 既存のEntityがあれば削除する
    query_entity.iter().for_each(|id| cmds.entity(id).despawn());

    // 3Dチェイサーをspawnする
    for chaser in query_chasers.iter()
    {
        let vec3 = IVec3::new(chaser.cell.x, -chaser.cell.y, 0).as_vec3();

        cmds.spawn((
            Mesh3d(meshes.add(Sphere::new(CHASER_3D_RADIUS).mesh().uv(36, 18))),
            MeshMaterial3d(materials.add(Color::Srgba(chaser.color))),
            Transform::from_translation(vec3),
            Chaser3d(Color::Srgba(chaser.color)),
        ));
    }
}

////////////////////////////////////////////////////////////////////////////////

//チェイサーの表示を更新する
#[rustfmt::skip]
pub fn update_3d_chasers(
    mut query_transform: Query<(&mut Transform, &Chaser3d)>,
    qry_chasers: Query<&tigtag2d::core_logic::chaser::Chaser>,
    time: Res<Time>,
    mut message_timer: MessageReader<tigtag2d::core_logic::ChaserPositionAdjusted>,
)
{
    //イベントが発生したチェイサーの色をハッシュに保存
    let mut message_color = HashSet::new();
    for x in message_timer.read()
    {
        let vec = &x.0;
        vec.iter().for_each(|c| {
            message_color.insert(LinearRgba::from(*c).as_u32());
        });
    }

    // チェイサーの色をキーに3Dの位置を保存
    let mut hash_transform = HashMap::new();
    for (transform, chaser3d) in query_transform.iter_mut()
    {
        hash_transform.insert(LinearRgba::from(chaser3d.0).as_u32(), transform);
    }

    // 前回からの経過時間
    let time_delta = time.delta();

    // 複数のチェイサーをループで処理
    for chaser in qry_chasers.iter()
    {
        let color_u32 = LinearRgba::from(chaser.color).as_u32();
        let transform = &mut hash_transform.get_mut(&color_u32).unwrap();
        let time_delta = time_delta.mul_f32(chaser.speedup);

        // tigtag(2d)からチェイサーの移動タイマーのfinishedが通知されたなら
        if message_color.contains(&color_u32)
        {
            let vec3 = IVec3::new(chaser.cell.x, -chaser.cell.y, 0).as_vec3();
            transform.translation = vec3;
        }
        else if !chaser.is_stop
        {
            // 移動中の中割座標
            let delta = CHASER_SPEED * time_delta.as_secs_f32();
            match chaser.direction
            {
                tigtag2d::core_logic::News::North => transform.translation.y += delta,
                tigtag2d::core_logic::News::South => transform.translation.y -= delta,
                tigtag2d::core_logic::News::East => transform.translation.x += delta,
                tigtag2d::core_logic::News::West => transform.translation.x -= delta,
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.
