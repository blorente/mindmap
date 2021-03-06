use glutin::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use components::MMItem;
use femtovg::{
    renderer::OpenGl, Align, Baseline, Color, FontId, ImageFlags, ImageId, Paint, Path, Renderer,
};
use rand::prelude::*;

use log::{debug, info};
use renderable::Renderable;
use settings::{RenderItemSettings, Settings};
use simple_logger::SimpleLogger;

mod components;
mod layout;
mod mind_map;
mod renderable;
mod settings;

use crate::mind_map::MindMap;
use crate::renderable::Canvas;

fn main() {
    // Logger
    SimpleLogger::new().init().unwrap();

    // Constants/Model
    let mut settings = Settings::new();
    let mut map = MindMap::new();
    map.create_node("I'm a parent".to_string(), None);
    let parent_ref = map
        .create_node("I'm another parent".to_string(), None)
        .expect("Error creating another parent");
    map.create_node("I'm a child of parent B".to_string(), Some(parent_ref));
    let layout = layout::layout_mind_map(&map, &settings);

    // Glium Window
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

    // Renderer
    let renderer =
        OpenGl::new_from_glutin_context(&windowed_context).expect("Cannot create renderer");
    let mut canvas = Canvas::new(renderer).expect("Cannot create canvas");
    canvas.set_size(
        window_size.width as u32,
        window_size.height as u32,
        windowed_context.window().scale_factor() as f32,
    );
    let font = canvas
        .add_font(settings.font_path)
        .expect("Error loading font.");

    // Reconfigure settings with the canvas information.
    settings.render_item_settings.font = Some(font);

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

                layout.iter().for_each(|(id, coords)| {
                    map.nodes
                        .get(id)
                        .expect(&format!(
                            "Layout contains ID {}, but it's not in the map",
                            id
                        ))
                        .render(coords.clone(), &settings.render_item_settings, &mut canvas);
                });

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
