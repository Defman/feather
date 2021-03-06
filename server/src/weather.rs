use crate::{network::Network, Game, World};
use feather_core::network::packet::implementation::ChangeGameState;
use fecs::Entity;
use rand::Rng;

const TICKS_DAY: i32 = 24_000;
const TICKS_HALF_DAY: i32 = TICKS_DAY / 2;
const TICKS_WEEK: i32 = TICKS_DAY * 7;
// const THUNDER_FACTOR: i32 = 10;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Weather {
    Clear,
    Rain,
    Thunder,
}

#[derive(Debug, Clone, Copy)]
pub struct WeatherChangeEvent {
    pub from: Weather,
    pub to: Weather,
    pub duration: i32,
}

#[allow(unused)]
pub fn clear_weather(game: &mut Game) {
    let durration = game
        .rng()
        .gen_range(TICKS_HALF_DAY, TICKS_WEEK + TICKS_HALF_DAY);
    set_weather(game, Weather::Clear, durration);
}

#[system]
pub fn handle_weather(game: &mut Game, world: &mut World) {
    if game.level.clear_weather_time >= 0 {
        game.level.clear_weather_time -= 1;
        return;
    }

    let from = game.weather();

    game.level.rain_time -= 1;
    let mut to = if game.level.rain_time <= 0 {
        if game.level.raining {
            Weather::Clear
        } else {
            Weather::Rain
        }
    } else {
        from
    };

    game.level.thunder_time -= 1;
    to = if game.level.thunder_time <= 0 {
        if game.level.thundering {
            Weather::Clear
        } else {
            Weather::Thunder
        }
    } else {
        to
    };

    if from != to {
        let duration = match to {
            Weather::Clear => game
                .rng()
                .gen_range(TICKS_HALF_DAY, TICKS_WEEK + TICKS_HALF_DAY),
            _ => game.rng().gen_range(TICKS_HALF_DAY, TICKS_DAY),
        };
        let mut event = WeatherChangeEvent { from, to, duration };
        game.on_weather_change(world, &mut event);
        if event.to != from {
            set_weather(game, event.to, event.duration);
        }
    }
}

pub fn get_weather(game: &Game) -> Weather {
    if game.level.clear_weather_time > 0 {
        Weather::Clear
    } else if game.level.thundering {
        Weather::Thunder
    } else if game.level.raining {
        Weather::Rain
    } else {
        Weather::Clear
    }
}

pub fn set_weather(game: &mut Game, weather: Weather, duration: i32) -> Weather {
    let from = get_weather(game);
    match weather {
        Weather::Rain => {
            game.level.raining = true;
            game.level.rain_time = duration;
        }
        Weather::Thunder => {
            game.level.thundering = true;
            game.level.thunder_time = duration;
        }
        Weather::Clear => {
            game.level.raining = false;
            game.level.rain_time = 0;
            game.level.thundering = false;
            game.level.thunder_time = 0;
            game.level.clear_weather_time = duration;
        }
    };
    from
}

pub fn send_weather(world: &mut World, player: Entity, to: Weather) {
    let network = world.get::<Network>(player);

    network.send(create_weather_packet(to));
}

pub fn on_weather_change_broadcast_weather(game: &mut Game, world: &mut World, to: Weather) {
    game.broadcast_global(world, create_weather_packet(to), None);
}

fn create_weather_packet(to: Weather) -> ChangeGameState {
    let reason = match to {
        Weather::Rain | Weather::Thunder => 2,
        Weather::Clear => 1,
    };

    ChangeGameState {
        reason,
        value: 0f32,
    }
}
