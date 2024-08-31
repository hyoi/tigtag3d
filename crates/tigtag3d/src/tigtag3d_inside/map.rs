use super::*;

////////////////////////////////////////////////////////////////////////////////

//mapオブジェクト関係
const WALL_CUBE_SIZE      : f32 = 1.0;             //壁のサイズ
const WALL_CUBE_COLOR     : Color = Color::BISQUE; //通常Cubeの色
const WALL_CUBE_COLOR_ZERO: Color = Color::RED;    //原点Cubeの色
const GROUND_PLANE_COLOR  : Color = Color::MAROON; //地面の色

//3Dマップの全Entityの親に印をつけるためのComponent
#[derive( Component )]
pub struct MapZeroEntity;

//3DドットのEntityID保存用Resource(2次元vec)
#[derive( Resource )]
pub struct Dots3D { entities: Vec<Vec<Option<Entity>>> }

impl Default for Dots3D
{   fn default() -> Self
    {   let usize_w = tigtag::MAP_GRIDS_WIDTH  as usize;
        let usize_h = tigtag::MAP_GRIDS_HEIGHT as usize;
        Self
        {   entities: vec![ vec![ None; usize_h ]; usize_w ],
        }
    }
}

//迷路の3Dオブジェクトをspawnする
pub fn spawn_3d_map_entity
(   qry_entity: Query<Entity, With<MapZeroEntity>>,
    opt_dots3d: Option<ResMut<Dots3D>>,
    map: Res<tigtag::Map>,
    mut cmds: Commands,
    asset_svr: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{   let Some ( mut dots3d ) = opt_dots3d else { return };

    //既存のEntityがあれば削除する
    qry_entity.iter().for_each( | id | cmds.entity( id ).despawn_recursive() );

    //壁のサイズ、原点の壁のテクスチャ、他の壁のテクスチャ、地面のテクスチャ
    let wall_size = Vec3::ONE * WALL_CUBE_SIZE * if DEBUG() { 0.95 } else { 1.0 };
    let ( texture_wall_zero, texture_wall_normal )
        = if DEBUG()
        {   ( WALL_CUBE_COLOR_ZERO.into(), WALL_CUBE_COLOR.into() )
        }
        else
        {   let material = StandardMaterial
            {   base_color_texture: Some( asset_svr.load( ASSETS_SPRITE_BRICK_WALL ) ),
                ..default()
            };
            ( material.clone(), material )
        };
    let texture_ground = GROUND_PLANE_COLOR;
    let dot_radius = WALL_CUBE_SIZE * 0.1;

    //迷路をspawnする
    cmds.spawn( ( PbrBundle::default(), MapZeroEntity ) ) //Cube(親)
    .insert( meshes.add( Cuboid::from_size( wall_size ) ) )
    .insert( Transform::from_translation( Vec3::ZERO ) ) //原点
    .insert( materials.add( texture_wall_zero ) )
    .with_children
    (   | cmds |
        {   //子は、親からの相対位置にspawnされる(XZ平面)
            for x in tigtag::MAP_GRIDS_X_RANGE
            {    for y in tigtag::MAP_GRIDS_Y_RANGE
                {   //原点は親なのでスキップ
                    if x == 0 && y == 0 { continue }

                    //3D空間の座標
                    let cell = IVec2::new( x, y );
                    let vec3 = cell.to_3dxz();

                    //壁
                    if map.is_wall( cell )
                    {   cmds.spawn( PbrBundle::default() )
                        .insert( meshes.add( Cuboid::from_size( wall_size ) ) )
                        .insert( Transform::from_translation( vec3 ) )
                        .insert( materials.add( texture_wall_normal.clone() ) )
                        ;
                    }

                    //3D球（ドット）
                    if map.is_space( cell )
                    {   let id = cmds.spawn( PbrBundle::default() )
                        .insert( meshes.add( Sphere::new( dot_radius ).mesh().uv( 36, 18 ) ) )
                        .insert( Transform::from_translation( vec3 ) )
                        .insert( materials.add( Color::YELLOW ) )
                        .id()
                        ;
                        dots3d.entities[ x as usize ][ y as usize ] = Some( id );
                    }
                }
            }

            //地面も相対位置でspawnする
            let width  = tigtag::MAP_GRIDS_WIDTH  as f32;
            let height = tigtag::MAP_GRIDS_HEIGHT as f32;
            let translation = Vec3::new( width * 0.5, height * -0.5, 0.0 ) - Vec3::ONE * 0.5;
            let transform = Transform::from_translation( translation );
            cmds.spawn( PbrBundle::default() )
            .insert( meshes.add( Plane3d::default().mesh().size( width, height ) ) )
            .insert( transform.with_rotation( Quat::from_rotation_x( PI * 0.5 )) )
            .insert( materials.add( texture_ground ) )
            ;
        }
    );
}

////////////////////////////////////////////////////////////////////////////////

//3Dマップの表示を更新する
pub fn update_3d_map
(   mut evt_eatdot: EventReader<tigtag::EventEatDot>,
    opt_dots3d: Option<ResMut<Dots3D>>,
    mut cmds: Commands,
)
{   let Some ( dots3d ) = opt_dots3d else { return };

    //削除されたドットがあれば3Dマップに反映する
    for event in evt_eatdot.read()
    {   let tigtag::EventEatDot ( IVec2 { x, y } ) = event;
        if let Some ( id ) = dots3d.entities[ *x as usize ][ *y as usize ]
        {   cmds.entity( id ).despawn_recursive();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.