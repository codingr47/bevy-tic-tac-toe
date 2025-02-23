use bevy::{color::palettes::css::BLACK, prelude::*, window::WindowResized, winit::cursor::CursorIcon};
use bevy::prelude::*;
use bevy::window::{SystemCursorIcon};

#[derive(Resource)]
struct BoardDimension(f32);

#[derive(Component)]
struct MainBoardNode;


#[derive(Resource)]
struct MainBoardContainerEntity(Entity);


#[derive(Component)]
struct BoardPieceNode(i32, i32);

#[derive(Component)]
struct BoardVerticalBorder(i32);

#[derive(Component)]
struct BoardHorizontalBorder(i32);

#[derive(Component)]
struct Hoverable;

struct BoardPieceTransform  {
    width: f32,
    height: f32,
    x: f32,
    y: f32,
}



#[derive(Event)]
struct EventBoardDimensionsChanged {
    dimension: f32,
}

#[derive(Resource)]
struct CursorIcons(Vec<CursorIcon>);


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
        .add_systems(Startup, (init_cursor_icons, find_board_dimension, setup_board, setup_pieces).chain())
        .add_systems(Update, on_window_resize)
        .add_systems(Update, (
            on_board_dimension_change_boarders_x, 
            on_board_dimension_change_boarders_y,
            on_board_dimension_change_squares,
        ))
        .add_systems(Update, detect_mouse_hover_board_pieces)
        .add_event::<EventBoardDimensionsChanged>();
    }
}
fn on_window_resize(
    mut resize_events: EventReader<WindowResized>, 
    mut event_writer: EventWriter<EventBoardDimensionsChanged>,
    mut main_board_node: Query<&mut Node, With<MainBoardNode>>,
) {
    if let Some(event) = resize_events.read().last() {
        let dim = f32::min(event.width, event.height);
        if let Some(mut board) = main_board_node.iter_mut().next() {
            board.width = Val::Px(dim);
            board.height = Val::Px(dim);
            event_writer.send(EventBoardDimensionsChanged { dimension: dim });
        }
    }
}

fn on_board_dimension_change_boarders_x(
    mut dimension_changed_events: EventReader<EventBoardDimensionsChanged>, 
    mut board_borders_x: Query<&mut Node, With<BoardVerticalBorder>>,
) {
    if let Some(event) = dimension_changed_events.read().last() {
        let dim = event.dimension;
        let mut i = 1;
        for mut board_border in board_borders_x.iter_mut() {
                board_border.height = Val::Px(dim);
                board_border.width = Val::Px(get_board_border_width(dim));
                board_border.left = Val::Px(get_board_border_position_factor(dim, i));
                i += 1;
        }
    }
}


fn on_board_dimension_change_boarders_y(
    mut dimension_changed_events: EventReader<EventBoardDimensionsChanged>, 
    mut board_borders_y: Query<&mut Node, With<BoardHorizontalBorder>>,
) {
    if let Some(event) = dimension_changed_events.read().last() {
        let dim = event.dimension;
        let mut i = 1;
        for mut board_border in board_borders_y.iter_mut() {
            board_border.width = Val::Px(dim);
            board_border.height = Val::Px(get_board_border_height(dim));
            board_border.top = Val::Px(get_board_border_position_factor(dim, i));
            i += 1;
        }
    }
}

fn on_board_dimension_change_squares(
    mut dimension_changed_events: EventReader<EventBoardDimensionsChanged>, 
    mut squares_q: Query<(&mut Node, &BoardPieceNode),With<BoardPieceNode>>,
) {
    if let Some(event) = dimension_changed_events.read().last() {
        let dim = event.dimension;
        for (mut node, piece) in &mut squares_q {
            let square_transform = get_square_transform(dim, piece.0, piece.1);
            node.width = Val::Px(square_transform.width);
            node.height = Val::Px(square_transform.height);
            node.left = Val::Px(square_transform.x);
            node.top = Val::Px(square_transform.y);
        }
    }
}

fn detect_mouse_hover_board_pieces(
    mut commands: Commands,
    window: Single<Entity, With<Window>>,
    mut query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Hoverable>)>,
    cursor_icons: Res<CursorIcons>,
) {
        for (interaction, mut color) in query.iter_mut() {
            match *interaction {
                Interaction::Hovered => {
                    println!("Mouse entered the UI element!");
                    commands
                    .entity(*window)
                    .insert(cursor_icons.0[1].clone());
                }
                Interaction::None => {
                    println!("Mouse left the UI element!");
                    commands
                    .entity(*window)
                    .insert(cursor_icons.0[0].clone());
                }
                Interaction::Pressed => {
                    println!("UI element clicked!");
                }
            }
        }   
}

fn mutate_board_dimension(dimension: f32, mut res_board_dimension: ResMut<BoardDimension>) {
    res_board_dimension.0 = dimension;
}

fn find_board_dimension(mut windows: Query<&mut Window>, board_dimension: ResMut<BoardDimension>) {
    if let Some(window) = windows.iter_mut().next() {
        mutate_board_dimension(f32::min(window.width(), window.height()), board_dimension);
    }
}

fn get_square_transform(board_dimension: f32, x: i32, y: i32) -> BoardPieceTransform {
    let rect_dim = board_dimension / 3.0;
    let lines_width =  board_dimension * LINES_WIDTH_PERCENTAGE * 0.01;
    let lines_width_half = lines_width * 0.5;
    let x_f32: f32 = x as f32;
    let y_f32 = y as f32;
    let left: f32 =  board_dimension  * (THIRD * x_f32 * 0.01) +  ( if x == 1 { 1.0 } else if x == 2 { 3.0 } else { 0.0 }) * lines_width_half;
    let top: f32 = board_dimension  * (THIRD * y_f32 * 0.01) +  ( if y == 1 { 1.0 } else if y == 2 { 3.0 } else { 0.0 }) * lines_width_half;
    return BoardPieceTransform {
        width: rect_dim  - if x == 0 { lines_width_half } else if x == 1 { 0.0 } else { lines_width },
        height: rect_dim  - if y == 0 { lines_width_half } else if y == 1 { 0.0 } else { lines_width },
        x: left,
        y: top,
    };
}


fn init_cursor_icons(
    mut commands: Commands,
) {
    commands.insert_resource(CursorIcons(vec![
        SystemCursorIcon::Default.into(),
        SystemCursorIcon::Pointer.into(),
    ]));
}


fn setup_pieces(
    mut commands: Commands,
    board_query: Option<Res<MainBoardContainerEntity>>,
    board_dimension: Res<BoardDimension>,
) {
    if let Some(board) = board_query {
        commands.entity(board.0).with_children(|parent| {
            for i in 0..=2 {
                for j in 0..=2 {
                    let board_piece_transform = get_square_transform(board_dimension.0, i, j);
                    parent.spawn((
                        BoardPieceNode(i, j),
                        Node {
                            width: Val::Px(board_piece_transform.width),
                            height: Val::Px(board_piece_transform.height),
                            position_type: PositionType::Absolute,
                            top: Val::Px(board_piece_transform.y),
                            border: UiRect { left: Val::Px(1.0), right: Val::Px(1.0), top: Val::Px(1.0), bottom: Val::Px(1.0), },
                            left: Val::Px(board_piece_transform.x),
                            ..default()
                        },
                        Hoverable,
                        Interaction::default(),
                        BackgroundColor(PURPLE), 
                        BorderColor(Color::from(BLACK)),
                    ));
                }
            }
        });
        
    }
}


fn get_board_border_position_factor(board_dimension:f32, i: i32) -> f32 {
    let i_float = i as f32;
    let lines_width =  board_dimension * LINES_WIDTH_PERCENTAGE * 0.01;
    let lines_width_half =  lines_width * 0.5;
    let position_factor: f32 =  board_dimension * (THIRD * i_float * 0.01)  +  ( if i == 1 { -1.0 } else { 1.0 }) * lines_width_half; 

    return position_factor;
}

fn get_board_border_width(board_dimension: f32) -> f32 {
    LINES_WIDTH_PERCENTAGE * board_dimension * 0.01
}

fn get_board_border_height(board_dimension: f32) -> f32 {
    LINES_WIDTH_PERCENTAGE * board_dimension * 0.01
}


fn setup_board(
    mut commands: Commands,
    board_dimension: Res<BoardDimension>,
) {
    commands.spawn(Camera2d);

    let board_dim = board_dimension.0;
    let main_board_node = commands.spawn((
            MainBoardNode,
            Node {
                width: Val::Px(board_dim),
                height: Val::Px(board_dim),
                aspect_ratio: Some(1.0),
                ..default()
            },
            BackgroundColor(BG_COLOR),
    ));
    let main_board_node_id = main_board_node.id();

    for i in 1..=2 {
        let position_factor = get_board_border_position_factor(board_dim, i);
        let board_border_vertical_width = get_board_border_width(board_dim);
        let board_border_horizontal_height = get_board_border_height(board_dim);
        
        commands.spawn((
            BoardVerticalBorder(i),
            Node {
                width: Val::Px(board_border_vertical_width),
                height: Val::Px(board_dim),
                left: Val::Px(position_factor),
                position_type: PositionType::Absolute,
                ..default()
            },
            BackgroundColor(BG_LINES),
        ));

        commands.spawn((
            BoardHorizontalBorder(i),
            Node {
                width: Val::Px(board_dim),
                height: Val::Px(board_border_horizontal_height),
                top: Val::Px(position_factor),
                position_type: PositionType::Absolute,
                ..default()
                
            },
            BackgroundColor(BG_LINES),
        ));
    }

    commands.insert_resource(MainBoardContainerEntity(main_board_node_id));

}


fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(TicTacToe)
    .run();
}