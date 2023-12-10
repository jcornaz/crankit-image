#![no_std]

use crankit_game_loop::{game_loop, Game, Playdate};
use crankit_image::{impls::playdate_sys_v02::Image, DrawImage, LoadImage, ToColumns, ToRows};

pub struct MyGame {
    cols: [Image<'static>; 2],
    rows: [Image<'static>; 2],
}

impl Game for MyGame {
    fn new(playdate: &Playdate) -> Self {
        let cols = playdate.graphics.load_from_path("cols").unwrap();
        let mut col_iter = cols.to_columns(2);
        let rows = playdate.graphics.load_from_path("rows").unwrap();
        let mut row_iter = rows.to_rows(2);
        Self {
            cols: [col_iter.next().unwrap(), col_iter.next().unwrap()],
            rows: [row_iter.next().unwrap(), row_iter.next().unwrap()],
        }
    }

    fn update(&mut self, playdate: &Playdate) {
        playdate.graphics.draw(&self.cols[0], [10, 10]);
        playdate.graphics.draw(&self.cols[1], [400 - 42, 10]);
        playdate.graphics.draw(&self.rows[0], [10, 240 - 42]);
        playdate.graphics.draw(&self.rows[1], [400 - 42, 240 - 42]);
    }
}

game_loop!(MyGame);
