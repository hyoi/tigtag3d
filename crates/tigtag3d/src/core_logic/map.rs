use super::*;

////////////////////////////////////////////////////////////////////////////////

//mapオブジェクト関係
const WALL_CUBE_SIZE: f32 = 1.0; //壁のサイズ
const WALL_CUBE_COLOR: Color = COLOR_BISQUE; //通常Cubeの色
const WALL_CUBE_COLOR_ZERO: Color = COLOR_RED; //原点Cubeの色
const GROUND_PLANE_COLOR: Color = COLOR_MAROON; //地面の色

//3Dマップの全Entityの親に印をつけるためのComponent
#[derive(Component)]
pub struct MapZeroEntity;

//3DドットのEntityID保存用Resource(2次元vec)
#[derive(Resource)]
pub struct Dots3D
{
    entities: Vec<Vec<Option<Entity>>>,
}

impl Default for Dots3D
{
    fn default() -> Self
    {
        let usize_w = MAP_WIDTH_IN_CELLS as usize;
        let usize_h = MAP_HEIGHT_IN_CELLS as usize;
        Self {
            entities: vec![vec![None; usize_h]; usize_w],
        }
    }
}

//迷路の3Dオブジェクトをspawnする
pub fn spawn_3d_map_entity(
    option_dots3d: Option<ResMut<Dots3D>>,
    query_entity: Query<Entity, With<MapZeroEntity>>,
    map: Res<tigtag2d::core_logic::map::Map>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) -> Result
{
    // 準備
    let mut dots3d = option_dots3d.ok_or("Resource not found.")?;

    //既存のEntityがあれば削除する
    query_entity.iter().for_each(|id| cmds.entity(id).despawn());

    //壁のサイズ、原点の壁のテクスチャ、他の壁のテクスチャ、地面のテクスチャ
    let wall_size =
        Vec3::ONE * WALL_CUBE_SIZE * if misc::DEBUG() { 0.95 } else { 1.0 };
    let (texture_wall_zero, texture_wall_normal) = if misc::DEBUG()
    {
        (WALL_CUBE_COLOR_ZERO.into(), WALL_CUBE_COLOR.into())
    }
    else
    {
        let material = StandardMaterial {
            base_color_texture: Some(asset_svr.load(ASSETS_SPRITE_BRICK_WALL)),
            ..default()
        };
        (material.clone(), material)
    };
    let texture_ground = GROUND_PLANE_COLOR;
    let dot_radius = WALL_CUBE_SIZE * 0.1;

    //迷路をspawnする
    cmds.spawn((
        Mesh3d(meshes.add(Cuboid::from_size(wall_size))),
        MeshMaterial3d(materials.add(texture_wall_zero)), //単色
        Transform::from_translation(Vec3::ZERO),          //原点(全軸0.0)に配置
        MapZeroEntity,                                    //マーカー
    ))
    .with_children(|cmds| {
        //子は、親からの相対位置にspawnされる(XZ平面)
        for x in tigtag2d::core_logic::map::MAP_CELLS_X_RANGE
        {
            for y in tigtag2d::core_logic::map::MAP_CELLS_Y_RANGE
            {
                //原点は親なのでスキップ
                if x == 0 && y == 0
                {
                    continue;
                }

                //3D空間の座標
                let cell = IVec2::new(x, y);
                let vec3 = IVec3::new(x, -y, 0).as_vec3();

                //壁
                if map.is_wall(cell)
                {
                    cmds.spawn((
                        Mesh3d(meshes.add(Cuboid::from_size(wall_size))),
                        MeshMaterial3d(materials.add(texture_wall_normal.clone())), //単色
                        Transform::from_translation(vec3),
                    ));
                }

                //3D球（ドット）
                if map.is_space(cell)
                {
                    let id = cmds
                        .spawn((
                            Mesh3d(
                                meshes
                                    .add(Sphere::new(dot_radius).mesh().uv(36, 18)),
                            ),
                            MeshMaterial3d(materials.add(COLOR_YELLOW)),
                            Transform::from_translation(vec3),
                        ))
                        .id();
                    dots3d.entities[x as usize][y as usize] = Some(id);
                }
            }
        }

        //地面も相対位置でspawnする
        let width = MAP_WIDTH_IN_CELLS as f32;
        let height = MAP_HEIGHT_IN_CELLS as f32;
        let translation =
            Vec3::new(width * 0.5, height * -0.5, 0.0) - Vec3::ONE * 0.5;
        cmds.spawn((
            Mesh3d(meshes.add(Plane3d::default().mesh().size(width, height))),
            MeshMaterial3d(materials.add(texture_ground)),
            Transform::from_translation(translation)
                .with_rotation(Quat::from_rotation_x(PI * 0.5)),
        ));
    });

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

//3Dマップの表示を更新する
pub fn update_3d_map(
    option_dots3d: Option<ResMut<Dots3D>>,
    mut evt_eatdot: MessageReader<tigtag2d::core_logic::DotEaten>,
    mut cmds: Commands,
) -> Result
{
    // 準備
    let dots3d = option_dots3d.ok_or("Resource not found.")?;

    //削除されたドットがあれば3Dマップに反映する
    for event in evt_eatdot.read()
    {
        let tigtag2d::core_logic::DotEaten(IVec2 { x, y }) = event;
        if let Some(id) = dots3d.entities[*x as usize][*y as usize]
        {
            cmds.entity(id).despawn();
        }
    }

    Ok(())
}

////////////////////////////////////////////////////////////////////////////////

//End of code.
