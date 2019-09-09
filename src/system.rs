use super::{Direction, Enemy, Player, Position, TilePosition, TILE_SIZE};
use coffee::graphics::{Color, Mesh, Rectangle, Shape};
use coffee::input::keyboard::KeyCode;
use coffee::input::KeyboardAndMouse;
use legion::query::filter::entity_data;
use legion::query::{IntoQuery, Query, Read, Write};
use legion::World;

pub fn interact(world: &mut World, input: &mut KeyboardAndMouse) {
    let query = <(Write<Player>)>::query();
    for player in query.iter(world) {
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

pub fn update_player_tile_position(world: &mut World) {
    let query = <(Write<Player>, Write<TilePosition>)>::query();
    for (player, tile_position) in query.iter(world) {
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
}

pub fn update_enemy_tile_position(world: &mut World) {
    let query = <Write<TilePosition>>::query().filter(entity_data::<Enemy>());
    for tile_position in query.iter(world) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        let mut direction = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        direction.shuffle(&mut rng);

        match direction[0] {
            Direction::Up => tile_position.y -= 1.0,
            Direction::Down => tile_position.y += 1.0,
            Direction::Left => tile_position.x -= 1.0,
            Direction::Right => tile_position.x += 1.0,
        };
    }
}

pub fn update_position(world: &mut World) {
    let query = <(Write<Position>, Read<TilePosition>)>::query();
    for (position, tile_position) in query.iter(world) {
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

pub fn draw_entities(world: &World, mesh: &mut Mesh) {
    let query = <(Read<Position>, Read<Color>)>::query();
    for (position, color) in query.iter(world) {
        let rect = Rectangle {
            x: position.x,
            y: position.y,
            width: TILE_SIZE,
            height: TILE_SIZE,
        };
        mesh.fill(Shape::Rectangle(rect), *color);
    }
}
