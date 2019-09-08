use coffee::graphics::{Color, Frame, Image, Point, Quad, Rectangle, Window, WindowSettings};
use coffee::input::keyboard::KeyCode;
use coffee::input::KeyboardAndMouse;
use coffee::load::Task;
use coffee::{Game, Result, Timer};

const PRUSSIAN_BLUE: Color = Color {
    r: 0.0,
    g: 0.1922,
    b: 0.3255,
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
struct Player {
    position: Point,
    tile_position: Point,
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
    palette: Image,
    player: Player,
}

impl Game for FrogBeat {
    type Input = KeyboardAndMouse;
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<FrogBeat> {
        Task::using_gpu(|gpu| Image::from_colors(gpu, &[PRUSSIAN_BLUE])).map(|palette| FrogBeat {
            palette,
            player: Player {
                position: Point::new(0.0, 0.0),
                tile_position: Point::new(0.0, 0.0),
                direction: None,
            },
        })
    }

    fn interact(&mut self, input: &mut Self::Input, _window: &mut Window) {
        if input.was_key_released(KeyCode::W) {
            self.player.direction = Some(Direction::Up);
        }
        if input.was_key_released(KeyCode::S) {
            self.player.direction = Some(Direction::Down);
        }
        if input.was_key_released(KeyCode::A) {
            self.player.direction = Some(Direction::Left);
        }
        if input.was_key_released(KeyCode::D) {
            self.player.direction = Some(Direction::Right);
        }
    }

    fn update(&mut self, _window: &Window) {
        match &self.player.direction {
            Some(direction) => {
                match direction {
                    Direction::Up => self.player.tile_position.y -= 1.0,
                    Direction::Down => self.player.tile_position.y += 1.0,
                    Direction::Left => self.player.tile_position.x -= 1.0,
                    Direction::Right => self.player.tile_position.x += 1.0,
                }
                self.player.direction = None;
            }
            None => {}
        }

        if self.player.position.y < self.player.tile_position.y * TILE_SIZE {
            self.player.position.y += 10.0;
        }
        if self.player.position.y > self.player.tile_position.y * TILE_SIZE {
            self.player.position.y -= 10.0;
        }
        if self.player.position.x < self.player.tile_position.x * TILE_SIZE {
            self.player.position.x += 10.0;
        }
        if self.player.position.x > self.player.tile_position.x * TILE_SIZE {
            self.player.position.x -= 10.0;
        }
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color::BLACK);

        let target = &mut frame.as_target();
        self.palette.draw(
            Quad {
                source: Rectangle {
                    x: 0.0,
                    y: 0.0,
                    width: 1.0,
                    height: 1.0,
                },
                position: self.player.position,
                size: (TILE_SIZE, TILE_SIZE),
            },
            target,
        );
    }
}
