#![feature(variant_count)]
#![feature(generic_arg_infer)]

use rand::prelude::*;
use raylib::prelude::*;
use std::mem;

#[derive(Clone,Copy,Debug)]
enum Type {
	Red,
	Blue,
}
const NULL_TYPE: Type = Type::Red;
const TYPE_COUNT: usize = mem::variant_count::<Type>();

#[derive(Clone,Copy,Debug)]
struct Particle {
	p: Vector2,
	v: Vector2,
	a: Vector2,
	id: Type,
}
const NULL_PARTICLE: Particle = Particle {
	p: Vector2 { x: 0.0, y: 0.0 },
	v: Vector2 { x: 0.0, y: 0.0 },
	a: Vector2 { x: 0.0, y: 0.0 },
	id: NULL_TYPE,
};

fn emit_particles<const COUNT: usize>(width: u32, height: u32, particle_type: Type) -> [Particle; COUNT] {
	let mut rng = rand::thread_rng();

	let mut particles = [NULL_PARTICLE; COUNT];
	for item in particles.iter_mut() {
		item.p = Vector2 {
			x: (rng.r#gen::<u32>() % width) as f32,
			y: (rng.r#gen::<u32>() % height) as f32,
		};
		item.v = Vector2::zero();
		item.a = Vector2::zero();
		item.id = particle_type;
	}
	particles
}

fn compute_force(forces: [f32; TYPE_COUNT * TYPE_COUNT], here: Particle, there: Particle) -> f32 {
	const MIN_DISTANCE: f32 = 5.0;
	const PEAK_DISTANCE: f32 = 20.0;
	const MAX_FORCE: f32 = 100.0;

	let distance = here.p.distance_to(there.p);
	let force_parameter = forces[here.id as usize * there.id as usize * TYPE_COUNT];
	
	if distance <= MIN_DISTANCE { distance * MAX_FORCE / MIN_DISTANCE - MAX_FORCE }
	else { force_parameter * MAX_FORCE - (distance - PEAK_DISTANCE).abs() * force_parameter * MAX_FORCE / (PEAK_DISTANCE - MIN_DISTANCE) }
}

fn render_particles(d: &mut RaylibDrawHandle, colors: [Color; TYPE_COUNT], center: Vector2, particles: &[Particle]) {
	const SCALE: f32 = 10.0;

	for particle in particles.iter() {
		let p = (particle.p + center - 0.5) * SCALE;
		d.draw_rectangle(
			p.x as i32,
			p.y as i32,
			SCALE as i32,
			SCALE as i32,
			colors[particle.id as usize]
		);
	}
}


fn main() {
	// Generate Force Table
	let forces: [f32; TYPE_COUNT * TYPE_COUNT] = [
		1.0, 1.0,
		1.0, 1.0,
	];

	let mut colors: [Color; TYPE_COUNT] = [Color::RED; TYPE_COUNT];
	colors[Type::Red as usize] = Color::RED;
	colors[Type::Blue as usize] = Color::BLUE;

	const PARTICLE_COUNT: usize = 10;
	let red_particles = emit_particles::<PARTICLE_COUNT>(10, 10, Type::Red);
	let blue_particles = emit_particles::<PARTICLE_COUNT>(10, 10, Type::Blue);

	const WIDTH: i32 = 1000;
	const HEIGHT: i32 = 1000;
	const CENTER: Vector2 = Vector2 { x: WIDTH as f32, y: HEIGHT as f32 };
	let (mut rl, thread) = raylib::init()
		.size(WIDTH, HEIGHT)
		.title("Colored Particles")
		.build();
	 
	while !rl.window_should_close() {
		let mut d = rl.begin_drawing(&thread);
		 
		d.clear_background(Color::BLACK);
		render_particles(&mut d, colors, CENTER, &red_particles);
		render_particles(&mut d, colors, CENTER, &blue_particles);
	}
}