
extern crate image;

pub use packer::Packer;
pub use shelf_packer::ShelfPacker;
pub use guillotine_packer::GuillotinePacker;
pub use maxrect_packer::MaxrectPacker;
pub use skyline_packer::SkylinePacker;
pub use rect::Rect;
pub use buffer2d::Buffer2d;
pub use image_buffer::ImageBuffer;
pub use color::{
    ColorType,
    RGBA,
    RGB,
    Grey,

    Color,
    RGBA8,
    RGB8,
    Grey8,
};

mod packer;
mod shelf_packer;
mod guillotine_packer;
mod maxrect_packer;
mod skyline_packer;
mod rect;
mod buffer2d;
mod image_buffer;
mod color;

