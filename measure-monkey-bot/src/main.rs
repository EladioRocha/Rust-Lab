// 1.- Obtener color RGB
// 2.- Crear mecanicas del bot

use autogui::{ AutoGUI, Position, MouseButton };
use std::{
  ptr::null_mut,
  time,
  thread
};
use winapi::um::{
  winuser::{GetDC, ReleaseDC},
  wingdi::GetPixel
};

const VEL_PER_PIXEL: f32 = 0.002635;

trait Pixel {
  fn pixel(&self) -> (u8, u8, u8);
}

impl Pixel for Position {
  fn pixel(&self) -> (u8, u8, u8) {
    let hdc = unsafe { GetDC(null_mut()) };
    let pixel = unsafe { GetPixel(hdc, self.x as i32, self.y as i32) };
    unsafe { ReleaseDC(null_mut(), hdc) };
    (
      (pixel & 255) as u8,
      ((pixel >> 8) & 255) as u8,
      ((pixel >> 16) & 255) as u8
    )
  }
}

fn main() {
  let gui = AutoGUI::new();
  let mut mouse = gui.mouse;
  let mut position = Position::new(574, 490);
  mouse = mouse.move_to(position);
  let (r, g, b) = position.pixel();
  loop {
    for x in 610..855 {
      position = Position::new(x, 490);
      let rgb = position.pixel();
      if rgb.0 == r && rgb.1 == g && rgb.2 == b && x > 610 {
        let milis = (VEL_PER_PIXEL * (x as f32 - 610.0)) * 1000.0;
        mouse = mouse.down(MouseButton::Left);
        thread::sleep(time::Duration::from_millis(milis as u64));
        mouse = mouse.up(MouseButton::Left);
        break;
      }
    }

    thread::sleep(time::Duration::from_millis(3000));
  }
}