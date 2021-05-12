use glium::{
	self,
	glutin::{
		self,
		event_loop::ControlFlow
	},
	Surface,
	uniform
};

use learn_glium::*;

use std::f32::consts::PI;

fn main() {
	let event_loop = glutin::event_loop::EventLoop::new(); // handles window and device events

	let window_builder = glutin::window::WindowBuilder::new() // window attributes
							.with_title("A cool window!");

	let contxt_builder = glutin::ContextBuilder::new(); // opengl attributes
							//.with_vsync(true);

	// builds the window and context, links them together and registers the window with the event loop
	let display = glium::Display::new(window_builder, contxt_builder, &event_loop).unwrap();

	let shape: Vec<Vertex> = vec![ // The triangle's shape
		Vertex { position: [-0.5, -0.5], tex_coords: [0.0, 0.0]}, // bottom-left
		Vertex { position: [ 0.0,  0.5], tex_coords: [0.5, 1.0]}, // top
		Vertex { position: [ 0.5, -0.5], tex_coords: [1.0, 0.0]}  // bottom-right
	];

	// upload the shape to vram for faster access
	let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

	let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList); // dummy marker
	
	let texture = make_texture(&display, "texture.png");
	
	// The vertex shader will be invoked 3 times, once per vertex. it takes the position of each vertex
	// in OpenGL's coordinate system and translates it to a position on the screen
	let vertex_shader_src = r#"
		#version 140 // opengl version being used
		
		// in variables come from each vertex, out variables are passed to the fragment shader
		in vec2 position; // expects to be passed a vec2 and stores it in position
		in vec2 tex_coords;
		//out vec2 attr; // returns a vec2 to pass to the frag shader
		out vec2 v_tex_coords;
		
		// we use a 4x4 matrix (2D table of numbers that can represent a geometrical transformation)
		uniform mat4 matrix;  // a uniform variable, whose value is set by passing it from the draw function.

		void main() { // main() is called once per vertex
			//attr = position; // store the position in the attribute
			v_tex_coords = tex_coords;
			// my multiplying the vertex's position by the matrix, any transformation applied to the matrix
			// will also be applied to the vertex
			gl_Position = matrix * vec4(position, 0.0, 1.0); // tells opengl the actual position of the vertex
		}
	"#;
	
	// The fragment shader tells the GPU the color of each pixel
	let fragment_shader_src = r#"
		#version 140
		
		//in vec2 attr; // takes a vec2, passed from the vertex shader
		in vec2 v_tex_coords;
		out vec4 color; // will output a vec4, corresponding to the color of a pixel

		uniform sampler2D tex; // expects to be passed a texture by the draw call

		void main() { // main() is called once per pixel
			// we build a vec4 from a vec2 and 2 floats and return it for the color of said pixel
			//color = vec4(attr, 0.0, 1.0);
			color = texture(tex, v_tex_coords);
		}
	"#;

	// compile both shaders into a program
	let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();
	
	let mut dt: f32 = 0.0;

	// main loop
	event_loop.run(move |ev, _, control_flow| {
		*control_flow = ControlFlow::Poll;

		match ev {
			glutin::event::Event::WindowEvent {event, ..} => match event {
				glutin::event::WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
					return;
				},
				_ => ()
			},

			_ => ()
		}

		let mut target = display.draw(); // a Frame that represents the back buffer

		target.clear_color(0.3, 0.3, 0.3, 0.5);
		
		target.draw(
			&vertex_buffer, // a source of vertices
			&indices, // a source of indices
			&program, // a program
			&uniform! {  // the program's uniforms, we pass it a matrix so the GPU can calculate the transformations
				matrix: Transformation::new()
							.rotate_y(dt)
							.get(),
				tex: &texture
			},
			&Default::default() // draw parameters
		).unwrap();

		target.finish().unwrap(); // destroys the Frame object and copies the back buffer to the window

		// update dt
		dt += 0.0004;
		if dt > PI * 2.0 { dt = 0.0; } // make it loop
	})
}
