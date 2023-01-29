extern crate sdl2;

use std::time::Duration;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;

static SCREEN_WIDTH: u32 = 166;
static SCREEN_HEIGHT: u32 = 144;

static PIXEL_SIZE: u32 = 4;

pub fn render() {
	let sdl_context = sdl2::init().unwrap();
	let video_subsystem = sdl_context.video().unwrap();

	let window = video_subsystem.window("gbmu", SCREEN_WIDTH * PIXEL_SIZE, SCREEN_HEIGHT * PIXEL_SIZE)
		.position_centered()
		.build().unwrap();

	let mut canvas = window.into_canvas().build().unwrap();

	canvas.set_draw_color(Color::RGB(0, 255, 255));
	canvas.clear();
	canvas.present();

	let mut j: i32 = 0;
	let mut k: i32 = 0;
	while k < (SCREEN_HEIGHT * PIXEL_SIZE) as i32 {
		let x: u8 = (((j / PIXEL_SIZE as i32) * 255 / SCREEN_WIDTH as i32) % 255) as u8;
		let y: u8 = (((k / PIXEL_SIZE as i32) * 255/ SCREEN_HEIGHT as i32) % 255) as u8;
		canvas.set_draw_color(Color::RGB(x, y, 255 - x));
		canvas.fill_rect(Rect::new(j, k, PIXEL_SIZE, PIXEL_SIZE)).unwrap();
		j += PIXEL_SIZE as i32;
		if j >= (SCREEN_WIDTH * PIXEL_SIZE) as i32 {
			j = 0;
			k += PIXEL_SIZE as i32;
		}
	}
	canvas.present();

	let mut event_pump = sdl_context.event_pump().unwrap();
	'running: loop {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit {..} |
				Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
					break 'running
				},
				_ => {}
			}
		}

		std::thread::sleep(Duration::from_millis(10));
		canvas.present();
	}

}
