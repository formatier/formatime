use std::sync::Arc;

use crate::binding::{canvas::{self, draw}, context};

pub struct Point(i32, i32);
pub struct Rect(i32, i32);

pub struct CanvasId(i32);

pub enum DrawContext {
    Window,
    Widget(CanvasId),
}

impl DrawContext {
    fn get_id(&self) -> i32 {
        match self {
            Self::Window => 0,
            Self::Widget(id) => id.0,
        }
    }
}

pub struct Rgba(u64);

impl Rgba {
    pub fn new(red: u8, blue: u8, green: u8, alpha: u8) {
        let mut color = 0u64;
        color |= (red as u64) << 48;
        color |= (blue as u64) << 32;
        color |= (green as u64) << 16;
        color |= alpha as u64;

        Self(color);
    }

    fn get_color(&self) -> (u8, u8, u8, u8) {
        let color = self.0;
        let red = ((color >> 48) & 0xFFF) as u8;
        let blue = ((color >> 32) & 0xFFF) as u8;
        let green = ((color >> 16) & 0xFFF) as u8;
        let alpha = (color & 0xFFF) as u8;

        return (red, blue, green, alpha);
    }
}

pub struct Canvas {
    context: Arc<context::Context>,
    draw_context: DrawContext,

    ready_to_draw: bool,
    buf: Vec<Option<u64>>,

    frame_width: i32,
    frame_height: i32,
}

impl Canvas {
    pub fn new(
        ctx: Arc<context::Context>,
        draw_context: DrawContext,
        frame_width: i32,
        frame_height: i32,
    ) -> Result<Self, ()> {
        Ok(Self {
            context: ctx,
            draw_context,
            ready_to_draw: false,
            frame_width,
            frame_height,

            buf: Vec::new(),
        })
    }

    pub fn draw_rect(&mut self, Point(x, y): Point, Rect(width, height): Rect) {
        for x_ofset in x..=(x + width) {
            draw(self., color, x, y)
        }
    }
    pub fn draw_line(&mut self, Point(x, y): Point) {}
    pub fn draw_point(&mut self, Point(x, y): Point) {}
    pub fn flush(&mut self) {
        for (idx, pixel) in self.buf.iter().enumerate() {
            let x = idx as i32 % self.frame_height;
            let y = self.frame_height / idx as i32;
        }
    }
    pub fn reset(&mut self) {
        self.frame_width = unsafe { canvas::poll_width(self.draw_context.get_id()).unwrap_or(0) };
        self.frame_height = unsafe { canvas::poll_height(self.draw_context.get_id()).unwrap_or(0) };
        self.buf = vec![None]
    }
}
