mod system;

use coffee::graphics::{Color, Frame, Mesh, Window, WindowSettings};
use coffee::input::KeyboardAndMouse;
use coffee::load::loading_screen::ProgressBar;
use coffee::load::Task;
use coffee::{Game, Result, Timer};
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
        system::interact(&mut self.world, input);
    }

    fn update(&mut self, _window: &Window) {
        system::update_player_tile_position(&mut self.world);
        system::update_position(&mut self.world);
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        let mut mesh = Mesh::new();
        system::draw_entities(&self.world, &mut mesh);
        mesh.draw(&mut frame.as_target());
    }
}
