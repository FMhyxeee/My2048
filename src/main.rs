mod config;
use config::*;

mod game;
use game::*;

use bevy::{asset::HandleId, prelude::*, sprite::MaterialMesh2dBundle};
use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // 初始化背景颜色
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_state::<GameState>()
        .insert_resource(Game::default())
        .add_system(setup.in_schedule(OnEnter(GameState::RUNNING)))
        .run();
}

fn setup(
    mut commands: Commands,
    assert_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // 初始化存储数组
    let mut cell_value_save_temp = init_cell_value_save();
    let mut cell_background_save: Vec<HandleId> = Vec::new();

    // 计算左上方格偏移
	let side_length: f32 = (WINDOW_HEIGHT - CELL_SPACE * (CELL_SIDE_NUM as f32 + 1.0)) / CELL_SIDE_NUM as f32;
	let mut x_offset = -(side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
	let mut y_offset = (side_length + CELL_SPACE) * (CELL_SIDE_NUM as f32 / 2.0 - 0.5);
    x_offset = 2.0 * x_offset - (-1.0) * (WINDOW_WIDTH / 2.0 - CELL_SPACE) - side_length / 2.0;

    commands.spawn(Camera2dBundle::default());
    
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Box::new(WINDOW_WIDTH, WINDOW_HEIGHT, 0.0).into()).into(),
        material: materials.add(ColorMaterial::from(COLOR_BACKGROUND)),
        transform: Transform::from_xyz((WINDOW_WIDTH - WINDOW_HEIGHT) / 2.0, 0.0, 0.0),
        ..default()
    });


    let font = assert_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: side_length / 2.0,
        color: COLOR_BROWN,
    };

    let box_size = Vec2::new(side_length, side_length);


    for i in 0..CELL_SIDE_NUM {
        for j in 0..CELL_SIDE_NUM {
            let mut text = "";

            if cell_value_save_temp[i as usize][j as usize] == 2 {
                text = "2";
            }

            let materialColor = materials.add(ColorMaterial::from(cell_color(cell_value_save_temp[i as usize][j as usize].into())));


        }
    }

}

// 初始化一个空白的面板，随机生成两个2
fn init_cell_value_save() -> Vec<Vec<u8>> {
    let mut cell_value_save_temp = Vec::new();
    for i in 0..4 {
        let mut temp = Vec::new();
        for j in 0..4 {
            temp.push(0);
        }
        cell_value_save_temp.push(temp);
    }

    // 随机生成两个2
    let mut rng1 = rand::thread_rng().gen_range(0..16) as usize;
    let mut rng2 = rand::thread_rng().gen_range(0..16) as usize;
    while rng1 == rng2 {
        rng2 = rand::thread_rng().gen_range(0..16) as usize;
    }
    cell_value_save_temp[rng1 / 4][rng1 % 4] = 2;
    cell_value_save_temp[rng2 / 4][rng2 % 4] = 2;

    cell_value_save_temp
}


fn cell_color(cell_value: u32) -> bevy::render::color::Color {
    match cell_value {
        0 => COLOR_CELL_NULL,
        2 => COLOR_CELL_2,
        4 => COLOR_CELL_4,
        8 => COLOR_CELL_8,
        16 => COLOR_CELL_16,
        32 => COLOR_CELL_32,
        64 => COLOR_CELL_64,
        128 => COLOR_CELL_128,
        256 => COLOR_CELL_256,
        512 => COLOR_CELL_512,
        1024 => COLOR_CELL_1024,
        2048 => COLOR_CELL_2048,
        _ => COLOR_CELL_NULL,
    }
}