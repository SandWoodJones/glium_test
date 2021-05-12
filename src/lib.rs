use glium;

use ndarray::{
	arr2,
	Array2
};

use image;
use std::{
	io::{
		Cursor,
		Read
	},
	fs::File
};


#[derive(Clone, Copy)]
pub struct Vertex { // describes a point in 2D space
	pub position: [f32; 2],
	pub tex_coords: [f32; 2] // texture coordinates range from the bottom left (0,0) to the top right (1,1)
}

glium::implement_vertex!(Vertex, position, tex_coords); // implements the Vertex trait for the Vertex struct


pub struct Transformation {
	matrix: Array2<f32>
}

impl Transformation {

	// creates the identity matrix
	pub fn new() -> Transformation {
		Transformation {
			matrix: arr2( &[
				[ 1.0,  0.0,  0.0,  0.0],
				[ 0.0,  1.0,  0.0,  0.0],
				[ 0.0,  0.0,  1.0,  0.0],
				[ 0.0,  0.0,  0.0,  1.0],
			] )
		}
	}

	pub fn translate(&mut self, x: f32, y: f32, z: f32) -> Transformation {
		let translated: Array2<f32> = arr2( &[
			[ 1.0,  0.0,  0.0,    x],
			[ 0.0,  1.0,  0.0,    y],
			[ 0.0,  0.0,  1.0,    z],
			[ 0.0,  0.0,  0.0,  1.0],
		] );

		Transformation { matrix: self.matrix.dot(&translated) }
	}

	pub fn scale(&mut self, x: f32, y: f32, z: f32) -> Transformation {
		let scaled: Array2<f32> = arr2( &[
			[   x,  0.0,  0.0,  0.0],
			[ 0.0,    y,  0.0,  0.0],
			[ 0.0,  0.0,    z,  0.0],
			[ 0.0,  0.0,  0.0,  1.0],
		] );
		
		Transformation { matrix: self.matrix.dot(&scaled) }
	}

	pub fn rotate_x(&mut self, angle: f32) -> Transformation {
		let ttc = angle.cos();
		let tts = angle.sin();

		let rotated: Array2<f32> = arr2( &[
			[ 1.0,  0.0,  0.0,  0.0],
			[ 0.0,  ttc, -tts,  0.0],
			[ 0.0,  tts,  ttc,  0.0],
			[ 0.0,  0.0,  0.0,  1.0]
		] );

		Transformation { matrix: self.matrix.dot(&rotated)}
	}

	pub fn rotate_y(&mut self, angle: f32) -> Transformation {
		let ttc = angle.cos();
		let tts = angle.sin();

		let rotated: Array2<f32> = arr2( &[
			[ ttc,  0.0,  tts,  0.0],
			[ 0.0,  1.0,  0.0,  0.0],
			[-tts,  0.0,  ttc,  0.0],
			[ 0.0,  0.0,  0.0,  1.0]
		] );

		Transformation { matrix: self.matrix.dot(&rotated)}
	}

	pub fn rotate_z(&mut self, angle: f32) -> Transformation {
		let ttc = angle.cos();
		let tts = angle.sin();

		let rotated: Array2<f32> = arr2( &[
			[ ttc, -tts,  0.0,  0.0],
			[ tts,  ttc,  0.0,  0.0],
			[ 0.0,  0.0,  1.0,  0.0],
			[ 0.0,  0.0,  0.0,  1.0]
		] );

		Transformation { matrix: self.matrix.dot(&rotated)}
	}
	
	pub fn get(&self) -> [[f32; 4]; 4] {
		let mut result: [[f32; 4]; 4] = [[0.0; 4]; 4];
		for y in 0 .. self.matrix.nrows() {
			for x in 0 .. self.matrix.ncols() {
				result[y][x] = self.matrix[[x, y]]
			}
		}
		result
	}
}

pub fn make_texture(display: &glium::Display, path: &str) -> glium::texture::Texture2d {
	// load a file in binary mode
	let mut file = File::open(path).unwrap();
	let mut buffer = Vec::new(); // buffer for reading the file
	file.read_to_end(&mut buffer).unwrap(); // read the entire file storing its contents in the buffer

	// convert the buffer into an ImageBuffer with rgba8 (255, 255, 255, 255)
	let image = image::load(Cursor::new(buffer), image::ImageFormat::Png).unwrap()
					.to_rgba8();
	let image_size = image.dimensions();
	// convert the ImageBuffer to a RawImage2d
	let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_size);
	
	// convert the image into an usable texture and return it
	glium::texture::Texture2d::new(display, image).unwrap()
}
