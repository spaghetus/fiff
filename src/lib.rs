#![feature(cursor_remaining)]
use std::io::{Cursor, Read};

pub trait FromFiff
where
	Self: Sized,
{
	fn from_fiff(fiff: &mut Cursor<&[u8]>) -> Result<Self, String>;
}

pub trait ToFiff
where
	Self: Sized,
{
	fn to_fiff(&self) -> Result<Vec<u8>, String>;
}
#[derive(Clone)]
pub struct Image {
	pub width: u16,
	pub height: u16,
	pub blocks: Vec<Block>,
}
impl FromFiff for Image {
	fn from_fiff(c: &mut Cursor<&[u8]>) -> Result<Self, String> {
		let width = {
			let mut width = [0u8; 2];
			c.read_exact(&mut width).unwrap();
			u16::from_be_bytes(width)
		};
		let height = {
			let mut height = [0u8; 2];
			c.read_exact(&mut height).unwrap();
			u16::from_be_bytes(height)
		};
		let blocks = c
			.remaining_slice()
			.chunks_exact(std::mem::size_of::<Block>())
			.flat_map(|b| Block::from_fiff(&mut std::io::Cursor::new(b)))
			.collect();
		Ok(Image {
			width,
			height,
			blocks,
		})
	}
}
impl Into<image::RgbaImage> for Image {
	fn into(self) -> image::RgbaImage {
		let mut i = image::RgbaImage::new(self.width as u32, self.height as u32);
		i.enumerate_pixels_mut().for_each(|(_, _, p)| {
			*p = image::Rgba([0, 0, 0, 0]);
		});
		for block in self.blocks {
			let left = block.aabb[0] as u32;
			let top = block.aabb[1] as u32;
			let right = left + block.aabb[2] as u32;
			let bottom = top + block.aabb[3] as u32;
			i.enumerate_pixels_mut().for_each(|(x, y, p)| {
				if left <= x && x <= right && top <= y && y <= bottom {
					*p = image::Rgba(block.color);
				}
			})
		}
		i
	}
}

#[derive(Clone)]
pub struct Block {
	pub aabb: [u16; 4],
	pub color: [u8; 4],
}
impl FromFiff for Block {
	fn from_fiff(c: &mut Cursor<&[u8]>) -> Result<Self, String> {
		let aabb = {
			let mut aabb = [0u16; 4];
			let mut buf = [0u8; 2];
			for n in 0..4 {
				c.read_exact(&mut buf).unwrap();
				aabb[n] = u16::from_be_bytes(buf);
			}
			aabb
		};
		let color = {
			let mut color = [0u8; 4];
			c.read_exact(&mut color).unwrap();
			color
		};
		Ok(Block { aabb, color })
	}
}
