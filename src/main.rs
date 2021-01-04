#![deny(clippy::all)]
#![forbid(unsafe_code)]

extern crate time;
use std::time::{Duration, Instant};

use log::error;
use winit::dpi::LogicalSize;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use pixels::{Error, Pixels, SurfaceTexture};

use std::fs;
use n2t_wasm::Emu;

fn main() -> Result<(), Error> {

    // Reading ROM file
    let buffer = fs::read_to_string("../roms/Pong.hack").expect("ASDF");
    
    let mut emu = Emu::new();
    emu.load_rom(&buffer);
    emu.store_ram(0, 128);

    // -------------------------- Game loop ---------------------------- //
    
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        
        let size = LogicalSize::new(512 as f64, 256 as f64);
        
        WindowBuilder::new()
            .with_title("n2t-wasm")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width,
            window_size.height, &window);
        Pixels::new(512, 256, surface_texture)?
    };

    let mut time = Instant::now();

    event_loop.run(move |event, _, control_flow| {
        emu.tick();
        
        if let Event::RedrawRequested(_) = event {
            
            emu.draw(pixels.get_frame());
            
            if pixels
                .render()
                .map_err(|e| error!("pixels.render() failed: {}", e))
                .is_err()
            {
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        if input.update(event) {
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if let Some(size) = input.window_resized() {
                pixels.resize(size.width, size.height);
            }
            
            let now = Instant::now();
            let dt = now.duration_since(time);
            
            let one_frame = Duration::new(0, 100_000_000);
            if dt > one_frame {
                time = now;
                window.request_redraw();
            }
        }
    });
}


