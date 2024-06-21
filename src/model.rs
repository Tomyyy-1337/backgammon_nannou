use nannou::Draw;

use crate::backgammon::{self, Player};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MousePos {
    Bar(Player),
    Off(Player),
    Board(u8),
}

pub struct Model {
    width: u32,
    height: u32,
    backgammon: backgammon::Backgammon,
    pub mouse_pos: MousePos,
}

impl Model {
    pub fn new(width: u32, height: u32) -> Self {   
        Model {
            width,
            height,
            backgammon: backgammon::Backgammon::new(),
            mouse_pos: MousePos::Off(Player::White),
        }
    }

    pub fn draw(&self, draw: &mut Draw) {
        draw.background().color(nannou::color::BLACK);

        self.higlight_mouse(draw);

        self.draw_board(draw);
               

    }

    fn draw_stones(&self, draw: &mut Draw) {
        

    }

    pub fn upate_mouse_pos(&mut self, x: f32, y: f32) {
        let tile_width = self.width as f32 / 14.0;   
        let mut indx = 13 - (x + self.width as f32 / 2.0) as u32 / tile_width as u32;
        if let Some(pos) = match indx {
            7 if y > 0.0 => Some(MousePos::Bar(Player::White)),
            7 => Some(MousePos::Bar(Player::Black)),
            0 if y > 0.0 => Some(MousePos::Off(Player::White)),
            0 => Some(MousePos::Off(Player::Black)),
            _ => {
                if y > 0.0 {
                    indx = 25 - indx;
                    if (12..=17).contains(&indx) {
                        indx += 1;
                    }
                } else {
                    if (8..=13).contains(&indx) {
                        indx -= 1;
                    }
                }
                if (1..=24).contains(&indx) {
                    Some(MousePos::Board(indx as u8))
                } else {
                    None
                }
            }
        } {
            if self.mouse_pos != pos {
                println!("{:?}", pos);
            }
            self.mouse_pos = pos;
        }
    }

    fn higlight_mouse(&self, draw: &mut Draw) {
        let color = nannou::color::rgba(0.0, 1.0, 1.0, 0.04);
        let tile_width = self.width as f32 / 14.0;
        let tile_height = tile_width * 5.0;
        let start_x = self.width as f32 / -2.0;
        match self.mouse_pos {
            MousePos::Bar(player) => {
                let x = start_x + 6.0 * tile_width;
                let y = if player == Player::White {
                    self.height as f32 / 4.0
                } else {
                    self.height as f32 / -4.0
                };
                draw.rect()
                    .x_y(x + tile_width / 2.0, y)
                    .w_h(tile_width, self.height as f32 / 2.0)
                    .color(color);
            }
            MousePos::Off(player) => {
                let x = start_x + 13.0 * tile_width;
                let y = if player == Player::White {
                    self.height as f32 / 4.0
                } else {
                    self.height as f32 / -4.0
                };
                draw.rect()
                    .x_y(x + tile_width / 2.0, y)
                    .w_h(tile_width, self.height as f32 / 2.0)
                    .color(color);
            }
            MousePos::Board(indx) => {
                let (x,y,m) = match indx {
                    1..=6 => (
                        start_x + (13.0 - indx as f32) * tile_width,
                        self.height as f32 / -2.0,
                        1.0
                    ),
                    7..=12 => (
                        start_x + (12.0 - indx as f32) * tile_width,
                        self.height as f32 / -2.0,
                        1.0
                    ),
                    13..=18 => (
                        start_x + (indx as f32 - 13.0) * tile_width,
                        self.height as f32 / 2.0,
                        -1.0
                    ),
                    19..=24 => (
                        start_x + (indx as f32 - 12.0) * tile_width,
                        self.height as f32 / 2.0,
                        -1.0
                    ),
                    _ => unreachable!()
                };
                draw.tri()
                    .points(
                        nannou::geom::Point2::new(x, y),
                        nannou::geom::Point2::new(x + tile_width, y),
                        nannou::geom::Point2::new(x + tile_width / 2.0, y + tile_height * m),
                    )
                    .color(color);
            }
        }
    }

    fn draw_board(&self, draw: &mut Draw) {
        let tile_width = self.width as f32 / 14.0;
        let tile_height = tile_width * 5.0;
        let start_x = self.width as f32 / -2.0;
        draw.line()
            .start(nannou::geom::Point2::new(start_x, self.height as f32 / -2.0))
            .end(nannou::geom::Point2::new(start_x, self.height as f32 / 2.0))
            .color(nannou::color::WHITE);
        for i in 0..14 {
            if i == 6 || i == 13 {
                draw.polyline()
                    .points_closed(vec![
                        nannou::geom::Point2::new(start_x + i as f32 * tile_width, self.height as f32 / -2.0),
                        nannou::geom::Point2::new(start_x + i as f32 * tile_width + tile_width, self.height as f32 / -2.0),
                        nannou::geom::Point2::new(start_x + i as f32 * tile_width + tile_width, self.height as f32 / 2.0),
                        nannou::geom::Point2::new(start_x + i as f32 * tile_width, self.height as f32 / 2.0),
                    ])
                    .color(nannou::color::WHITE);
                draw.line()
                    .start(nannou::geom::Point2::new(start_x + i as f32 * tile_width, 0.0))
                    .end(nannou::geom::Point2::new(start_x + i as f32 * tile_width + tile_width, 0.0))
                    .color(nannou::color::WHITE);
                    
                continue;
            }
            let color = nannou::color::WHITE;
            let x = start_x + i as f32 * tile_width;
            let y = self.height as f32 / -2.0;
            draw.polyline()
                .points_closed(vec![
                    nannou::geom::Point2::new(x, y),
                    nannou::geom::Point2::new(x + tile_width, y),
                    nannou::geom::Point2::new(x + tile_width / 2.0, y + tile_height),
                ])
                .color(color);
            if i % 2 == 0 && i <= 6 || i % 2 == 1 && i >= 7{
                draw.tri()
                    .points(
                        nannou::geom::Point2::new(x, y),
                        nannou::geom::Point2::new(x + tile_width, y),
                        nannou::geom::Point2::new(x + tile_width / 2.0, y + tile_height),
                    )
                    .color(nannou::color::rgba(1.0, 1.0, 1.0, 0.015));
            }
            let y = self.height as f32 / 2.0;
            draw.polyline()
                .points_closed(vec![
                    nannou::geom::Point2::new(x, y),
                    nannou::geom::Point2::new(x + tile_width, y),
                    nannou::geom::Point2::new(x + tile_width / 2.0, y - tile_height),
                    
                ])
                .color(color);
            if i % 2 == 0 && i >= 7 || i % 2 == 1 && i <= 6 {
                draw.tri()
                    .points(
                        nannou::geom::Point2::new(x, y),
                        nannou::geom::Point2::new(x + tile_width, y),
                        nannou::geom::Point2::new(x + tile_width / 2.0, y - tile_height),
                    )
                    .color(nannou::color::rgba(1.0, 1.0, 1.0, 0.015));
            }
        }
    }
}