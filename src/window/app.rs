use std::sync::Arc;
use std::time::Duration;

use softbuffer::{Context, Surface};
use winit::application::ApplicationHandler;
use winit::dpi::{PhysicalPosition, PhysicalSize};
use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::event_loop::{ActiveEventLoop, ControlFlow};
use winit::platform::windows::WindowAttributesExtWindows;
use winit::window::{Window, WindowId, WindowLevel};

use windows::Win32::Foundation::POINT;
use windows::Win32::UI::WindowsAndMessaging::GetCursorPos;

use crate::core::config::{
    BASE_HEIGHT, BASE_WIDTH, EXPANDED_HEIGHT, EXPANDED_WIDTH, PADDING, TOP_OFFSET, WINDOW_TITLE,
};
use crate::core::render::draw_island;

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    surface: Option<Surface<Arc<Window>, Arc<Window>>>,

    expanded: bool,
    current_w: f32,
    current_h: f32,
    current_r: f32,

    os_w: u32,
    os_h: u32,

    win_x: i32,
    win_y: i32,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);

        if self.window.is_none() {
            self.current_w = BASE_WIDTH;
            self.current_h = BASE_HEIGHT;
            self.current_r = 13.5;

            self.os_w = (EXPANDED_WIDTH + PADDING) as u32;
            self.os_h = (EXPANDED_HEIGHT + PADDING) as u32;

            let attrs = Window::default_attributes()
                .with_title(WINDOW_TITLE)
                .with_inner_size(PhysicalSize::new(self.os_w, self.os_h))
                .with_transparent(true)
                .with_decorations(false)
                .with_window_level(WindowLevel::AlwaysOnTop)
                .with_skip_taskbar(true);

            let window = Arc::new(event_loop.create_window(attrs).unwrap());
            self.window = Some(window.clone());

            if let Some(monitor) = window.current_monitor() {
                let mon_size = monitor.size();
                let mon_pos = monitor.position();
                let center_x = mon_pos.x + (mon_size.width as i32) / 2;
                let top_y = mon_pos.y + TOP_OFFSET;

                self.win_x = center_x - (self.os_w as i32) / 2;
                self.win_y = top_y - (PADDING / 2.0) as i32;
                window.set_outer_position(PhysicalPosition::new(self.win_x, self.win_y));
            }

            let context = Context::new(window.clone()).unwrap();
            let mut surface = Surface::new(&context, window.clone()).unwrap();

            surface
                .resize(
                    std::num::NonZeroU32::new(self.os_w).unwrap(),
                    std::num::NonZeroU32::new(self.os_h).unwrap(),
                )
                .unwrap();
            self.surface = Some(surface);
            
            window.request_redraw();
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                let mut point = POINT::default();
                unsafe { let _ = GetCursorPos(&mut point); }
                let rel_y = point.y - self.win_y;
                let island_y = PADDING as f64 / 2.0;

                if self.expanded {
                    if (rel_y as f64) >= island_y && (rel_y as f64) < island_y + 40.0 {
                        self.expanded = false;
                    }
                } else {
                    self.expanded = true;
                }
            }
            WindowEvent::RedrawRequested => {
                if let Some(surface) = self.surface.as_mut() {
                    draw_island(surface, self.current_w, self.current_h, self.current_r, self.os_w, self.os_h);
                }
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window) = &self.window {
            let mut point = POINT::default();
            unsafe { let _ = GetCursorPos(&mut point); }
            
            let rel_x = point.x - self.win_x;
            let rel_y = point.y - self.win_y;
            
            let island_y = PADDING as f64 / 2.0;
            let offset_x = (self.os_w as f64 - self.current_w as f64) / 2.0;
            
            let is_hovering = rel_x as f64 >= offset_x 
                && (rel_x as f64) <= offset_x + self.current_w as f64
                && rel_y as f64 >= island_y 
                && (rel_y as f64) <= island_y + self.current_h as f64;
                
            let _ = window.set_cursor_hittest(is_hovering);

            let target_w = if self.expanded { EXPANDED_WIDTH } else { BASE_WIDTH };
            let target_h = if self.expanded { EXPANDED_HEIGHT } else { BASE_HEIGHT };
            let target_r = if self.expanded { 32.0 } else { 13.5 };

            let diff_w = target_w - self.current_w;
            let diff_h = target_h - self.current_h;
            let diff_r = target_r - self.current_r;

            if diff_w.abs() > 0.5 || diff_h.abs() > 0.5 {
                self.current_w += diff_w * 0.18;
                self.current_h += diff_h * 0.18;
                self.current_r += diff_r * 0.18;
                window.request_redraw();
            } else if self.current_w != target_w {
                self.current_w = target_w;
                self.current_h = target_h;
                self.current_r = target_r;
                window.request_redraw();
            }

            std::thread::sleep(Duration::from_millis(16));
        }
    }
}
