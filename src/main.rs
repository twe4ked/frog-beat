use coffee::graphics::{Color, Frame, Mesh, Rectangle, Shape, Window, WindowSettings};
use coffee::input::keyboard::KeyCode;
use coffee::input::KeyboardAndMouse;
use coffee::load::loading_screen::ProgressBar;
use coffee::load::Task;
use coffee::{Game, Result, Timer};
use legion::query::{IntoQuery, Query, Read, Write};
use legion::{Universe, World};

const PRUSSIAN_BLUE: Color = Color {
    r: 0.0,
    g: 0.1922,
    b: 0.3255,
    a: 1.0,
};
const RED: Color = Color {
    r: 1.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};
const TILE_COUNT_Y: f32 = 16.0;
const TILE_COUNT_X: f32 = 20.0;
const TILE_SIZE: f32 = 50.0;

fn main() -> Result<()> {
    let width = (TILE_COUNT_X * TILE_SIZE) as u32;
    let height = (TILE_COUNT_Y * TILE_SIZE) as u32;

    FrogBeat::run(WindowSettings {
        title: String::from("From Beat"),
        size: (width, height),
        resizable: false,
        fullscreen: false,
    })
}

#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

impl From<&TilePosition> for Position {
    fn from(tile_position: &TilePosition) -> Self {
        Self {
            x: tile_position.x * TILE_SIZE,
            y: tile_position.y * TILE_SIZE,
        }
    }
}

#[derive(Debug)]
struct TilePosition {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Player {
    direction: Option<Direction>,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct FrogBeat {
    universe: Universe,
    world: World,
}

impl Game for FrogBeat {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ProgressBar;

    fn load(_window: &Window) -> Task<FrogBeat> {
        Task::new(move || {
            let universe = Universe::new(None);
            let mut world = universe.create_world();

            let tile_position = TilePosition {
                x: TILE_COUNT_X / 2.0,
                y: TILE_COUNT_Y - 1.0,
            };
            world.insert_from(
                (),
                vec![(
                    Position::from(&tile_position),
                    tile_position,
                    Player { direction: None },
                    PRUSSIAN_BLUE,
                )],
            );
            world.insert_from(
                (),
                (0..5).map(|i| {
                    let tile_position = TilePosition {
                        x: i as f32,
                        y: 1.0,
                    };
                    (Position::from(&tile_position), tile_position, RED)
                }),
            );

            FrogBeat { universe, world }
        })
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
        let query = <(Write<Player>)>::query();
        for player in query.iter(&self.world) {
            if input.was_key_released(KeyCode::W) {
                player.direction = Some(Direction::Up);
            }
            if input.was_key_released(KeyCode::S) {
                player.direction = Some(Direction::Down);
            }
            if input.was_key_released(KeyCode::A) {
                player.direction = Some(Direction::Left);
            }
            if input.was_key_released(KeyCode::D) {
                player.direction = Some(Direction::Right);
            }
        }
    }

    fn update(&mut self, _window: &Window) {
        let query = <(Write<Player>, Write<TilePosition>)>::query();
        for (player, tile_position) in query.iter(&self.world) {
            match &player.direction {
                Some(direction) => {
                    match direction {
                        Direction::Up => tile_position.y -= 1.0,
                        Direction::Down => tile_position.y += 1.0,
                        Direction::Left => tile_position.x -= 1.0,
                        Direction::Right => tile_position.x += 1.0,
                    }
                    player.direction = None;
                }
                None => {}
            }
        }

        let query = <(Write<Position>, Read<TilePosition>)>::query();
        for (position, tile_position) in query.iter(&self.world) {
            if position.y > tile_position.y * TILE_SIZE {
                position.y -= 10.0;
            }
            if position.y < tile_position.y * TILE_SIZE {
                position.y += 10.0;
            }
            if position.x < tile_position.x * TILE_SIZE {
                position.x += 10.0;
            }
            if position.x > tile_position.x * TILE_SIZE {
                position.x -= 10.0;
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        let mut mesh = Mesh::new();

        let query = <(Read<Position>, Read<Color>)>::query();
        for (position, color) in query.iter(&self.world) {
            let rect = Rectangle {
                x: position.x,
                y: position.y,
                width: TILE_SIZE,
                height: TILE_SIZE,
            };
            mesh.fill(Shape::Rectangle(rect), *color);
        }

        mesh.draw(&mut frame.as_target());
    }
}
