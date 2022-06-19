use glutin::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use femtovg::{
    renderer::OpenGl, Align, Baseline, Color, FontId, ImageFlags, ImageId, Paint, Path, Renderer,
};
use mmitem::MMItem;
use rand::prelude::*;

use log::{debug, info};
use renderable::Renderable;
use settings::RenderItemSettings;
use simple_logger::SimpleLogger;

mod layout;
mod mmitem;
mod renderable;
mod settings;

use crate::renderable::Canvas;

pub struct MousePosition {
    pub x: u32,
    pub y: u32,
}

fn main() {
    SimpleLogger::new().init().unwrap();
    let window_size = glutin::dpi::PhysicalSize::new(1000, 600);
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_inner_size(window_size)
        .with_resizable(false)
        .with_title("Text demo");

    let windowed_context = glutin::ContextBuilder::new()
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    let renderer =
        OpenGl::new_from_glutin_context(&windowed_context).expect("Cannot create renderer");
    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(
        window_size.width as u32,
        window_size.height as u32,
        windowed_context.window().scale_factor() as f32,
    );

    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::LoopDestroyed => return,
            Event::MainEventsCleared => windowed_context.window().request_redraw(),
            Event::WindowEvent { ref event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(*physical_size);
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            virtual_keycode: Some(keycode),
                            state: ElementState::Pressed,
                            ..
                        },
                    ..
                } => match keycode {
                    VirtualKeyCode::Escape => {
                        *control_flow = ControlFlow::Exit;
                    }
                    other => {
                        info!("Pressed {:?}", other)
                    }
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                let dpi_factor = windowed_context.window().scale_factor();
                let size = windowed_context.window().inner_size();
                canvas.set_size(size.width as u32, size.height as u32, dpi_factor as f32);
                canvas.clear_rect(
                    0,
                    0,
                    size.width as u32,
                    size.height as u32,
                    Color::rgbf(0.9, 0.9, 0.9),
                );

                let settings = RenderItemSettings::new();
                let squares: Vec<Box<dyn Renderable>> = vec![
                    Box::new(MMItem::new("Hello".to_string())),
                    Box::new(MMItem::new(
                        "World thiss is a much longer string".to_string(),
                    )),
                ];
                let layout = layout::layout_line(&squares, &settings);
                squares
                    .iter()
                    .enumerate()
                    .for_each(|(idx, square)| square.render(layout[idx], &settings, &mut canvas));

                canvas.flush();
                windowed_context.swap_buffers().unwrap();
            }
            Event::RedrawEventsCleared => (),
            Event::NewEvents(_) => (),
            _ => {
                debug!("Window event: {:?}", &event)
            }
        }
    });
}
