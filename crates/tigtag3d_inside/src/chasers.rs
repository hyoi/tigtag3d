use super::*;

////////////////////////////////////////////////////////////////////////////////

//3D敵キャラのComponent
#[derive( Component )]
pub struct Chaser3d ( Color );

//3D敵キャラの定数
const CHASER_3D_RADIUS: f32 = 0.45;

//速度(1.0は3D空間での1グリッドの距離)
const CHASER_SPEED: f32 = 1.0 / tigtag::CHASER_TIME_PER_GRID;

////////////////////////////////////////////////////////////////////////////////

//3D自キャラをspawnする
pub fn spawn_3d_chasers
(   qry_chasers: Query<&tigtag::Chaser>,
    qry_entity: Query<Entity, With<Chaser3d>>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{   //既存のEntityがあれば削除する
    qry_entity.iter().for_each( | id | cmds.entity( id ).despawn_recursive() );

    //3D自キャラをspawnする
    for chaser in qry_chasers.iter()
    {   let vec3 = chaser.grid.to_3dxz();
        cmds.spawn( ( PbrBundle::default(), Chaser3d ( chaser.color ) ) )
        .insert( meshes.add( Sphere::new( CHASER_3D_RADIUS ).mesh().uv( 36, 18 ) ) )
        .insert( Transform::from_translation( vec3 ) )
        .insert( materials.add( chaser.color ) )
        ;
    }
}

////////////////////////////////////////////////////////////////////////////////

//敵キャラの表示を更新する
pub fn update_3d_chasers
(   mut evt_timer: EventReader<tigtag::EventTimerChasers>,
    mut qry_transform: Query<( &mut Transform, &Chaser3d )>,
    qry_chasers: Query<&tigtag::Chaser>,
    time: Res<Time>,
)
{   //イベントが発生した敵キャラの色をハッシュに保存
    let mut evt_color = HashSet::new();
    for x in evt_timer.read()
    {   let vec = &x.0;
        vec.iter().for_each( |c| { evt_color.insert( c.as_rgba_u32() ); } );
    }

    //敵キャラの色をキーに3Dの位置を保存
    let mut hash_transform = HashMap::new();
    for ( transform, chaser3d ) in qry_transform.iter_mut()
    {   hash_transform.insert( chaser3d.0.as_rgba_u32(), transform );
    }

    //前回からの経過時間
    let time_delta = time.delta();

    //複数の敵キャラをループで処理
    for chaser in qry_chasers.iter()
    {   let color_u32 = chaser.color.as_rgba_u32();
        let transform =  &mut hash_transform.get_mut( &color_u32 ).unwrap();
        let time_delta = time_delta.mul_f32( chaser.speedup );

        if ! evt_color.contains( &color_u32 )
        {   if ! chaser.is_stop
            {   //移動中の中割座標
                let delta = CHASER_SPEED * time_delta.as_secs_f32();
                match chaser.direction
                {   tigtag::News::North => transform.translation.y += delta,
                    tigtag::News::South => transform.translation.y -= delta,
                    tigtag::News::East  => transform.translation.x += delta,
                    tigtag::News::West  => transform.translation.x -= delta,
                }
            }
        }
        else
        {   //Entityをグリッドに配置する
            transform.translation = chaser.grid.to_3dxz();
        }
    }
}

////////////////////////////////////////////////////////////////////////////////

//End of code.