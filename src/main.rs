/*
 * @Description: arcade-rs
 * @Author: 鱼摆摆
 * @Email: fishercloud@qq.com
 * @Github: https://github.com/FisherCloud
 * @Date: 2020-01-06 19:38:47
 * @LastEditTime: 2020-01-06 19:50:11
 */
extern crate sdl2;

use sdl2::pixels::Color;
use std::{thread, time};

fn main() {
    // 初始化 SDL2
    let sdl_context = sdl2::init().video().build().unwrap();

    // create window
    let window = sdl_context.window("Arcrade-rs Shooter", 800, 600).position_centered().opengl().build().unwrap();

    let mut renderer = window.renderer().accelerated().build().unwrap();

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.present();

    thread::sleep(time::Duration::from_secs(3));
}
