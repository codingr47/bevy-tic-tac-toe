use std::alloc::Layout;

use bevy::{color::palettes::css::BLACK, math::AspectRatio, prelude::*, window::{WindowRef, WindowResized}};

#[derive(Resource)]
struct BoardDimension(f32);

#[derive(Component)]
struct MainBoardNode;


#[derive(Resource)]
struct MainBoardContainerEntity(Entity);


pub struct TicTacToe;

const BG_COLOR: Color = Color::srgba(0.40, 0.01, 0.2, 0.15);
const BG_LINES: Color = Color::srgba(1.0, 1.0, 1.0, 0.75);
const PURPLE: Color = Color::srgb(0.5, 0.0, 0.5);
const LINES_WIDTH_PERCENTAGE: f32 = 2.0;
const THIRD: f32 = (100.0 / 3.0);

impl Plugin for TicTacToe {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(BoardDimension(0.0))
        .add_systems(Startup, (find_board_dimension, setup_board, setup_pieces).chain())
        .add_systems(Update, on_window_resize);
    }
}
fn on_window_resize(mut resize_events: EventReader<WindowResized>, mut main_board_node: Query<&mut Node,With<MainBoardNode>>) {
    if let Some(event) = resize_events.read().last() {
        let dim = f32::min(event.width, event.height);
        if let Some(mut board) = main_board_node.iter_mut().next() {
            board.width = Val::Px(dim);
            board.height = Val::Px(dim);
        }
    }
}

fn mutate_board_dimension(dimension: f32, mut res_board_dimension: ResMut<BoardDimension>) {
    res_board_dimension.0 = dimension;
}

fn find_board_dimension(mut windows: Query<&mut Window>, mut board_dimension: ResMut<BoardDimension>) {
    if let Some(window) = windows.iter_mut().next() {
        println!("Window dimensions: {}x{}", window.width(), window.height());
        mutate_board_dimension(f32::min(window.width(), window.height()), board_dimension);
    }
}

fn setup_pieces(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut commands: Commands,
    board_query: Option<Res<MainBoardContainerEntity>>,
    board_dimension: Res<BoardDimension>,
) {
    let rect_dim = board_dimension.0 / 3.0;
    let lines_width =  board_dimension.0 * LINES_WIDTH_PERCENTAGE * 0.01;
    let lines_width_half = lines_width * 0.5;
    let rectangle_asset = meshes.add(Rectangle::default());
    let material_asset = materials.add(Color::from(PURPLE));
    if let Some(board) = board_query {
        commands.entity(board.0).with_children(|parent| {
            for i in 0..=2 {
                for j in 0..=2 {
                    let i_f32: f32 = i as f32;
                    let j_f32 = j as f32;
                    let x_fac: f32 =  board_dimension.0  * (THIRD * i_f32 * 0.01) +  ( if i == 1 { 1.0 } else if i == 2 { 2.0 } else { 0.0 }) * lines_width;
                    let y_fac: f32 =  board_dimension.0  * (THIRD * j_f32 * 0.01) +  ( if j == 1 { 1.0 } else if j == 2 { 2.0 } else { 0.0 }) * lines_width;
                    println!("For i={}, j={}, then x={}, y={} and scale is {}", i_f32, j_f32, x_fac, y_fac, rect_dim );
                    parent.spawn((
                        //ImageBundle,
                        Node {
                            width: Val::Px(rect_dim  - if i == 1 { lines_width } else { lines_width_half }),
                            height: Val::Px(rect_dim - if j == 1 { lines_width } else { lines_width_half }),
                            position_type: PositionType::Absolute,
                            top: Val::Px(y_fac),
                            border: UiRect { left: Val::Px(1.0), right: Val::Px(1.0), top: Val::Px(1.0), bottom: Val::Px(1.0), },
                            left: Val::Px(x_fac),
                            ..default()
                        },
                        BackgroundColor(PURPLE),
                        BorderColor(Color::from(BLACK)),
                    ));
                }
            }
        });
        
    }
}

fn setup_board(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut board_dimension: Res<BoardDimension>,
) {
    commands.spawn(Camera2d);
    let board_dim = board_dimension.0;
    let mut main_board_node = commands.spawn((
            MainBoardNode,
            Node {
                width: Val::Px(board_dim),
                height: Val::Px(board_dim),
                align_items: AlignItems::Center,
                align_self: AlignSelf::Center, 
                aspect_ratio: Some(1.0),
                ..default()
            },
            BackgroundColor(BG_COLOR),
    ));
    let main_board_node_id = main_board_node.id();
    main_board_node.with_children(|parent| { 
        for i in 1..=2 {
            let i_float = i as f32;
            let lines_width =  board_dim * LINES_WIDTH_PERCENTAGE * 0.01;
            let lines_width_half =  lines_width * 0.5;
            let position_factor: f32 =  board_dim * (THIRD * i_float * 0.01)  +  ( if i == 1 { -1.0 } else { 1.0 }) * lines_width_half; 
            println!("HO {}", lines_width);
            println!("HI {}", position_factor);
            parent.spawn((
                Node {
                    width: Val::Px(LINES_WIDTH_PERCENTAGE * board_dim * 0.01),
                    height: Val::Px(board_dim),
                    left: Val::Px(position_factor),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                BackgroundColor(BG_LINES),
            ));
            parent.spawn((
                Node {
                    width: Val::Px(board_dim),
                    height: Val::Px(LINES_WIDTH_PERCENTAGE * board_dim * 0.01),
                    top: Val::Px(position_factor),
                    position_type: PositionType::Absolute,
                    ..default()
                    
                },
                BackgroundColor(BG_LINES),
            ));
        }
    });

    commands.insert_resource(MainBoardContainerEntity(main_board_node_id));

}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(TicTacToe)
    .run();
}