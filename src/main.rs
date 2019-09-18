#![recursion_limit="512"]

use std::fmt;

macro_rules! xprintln {
	($($tt:tt)*) => {
		println!("{}", format_xml::format_xml!($($tt)*));
	};
}
macro_rules! xwrite {
	($w:expr, $($tt:tt)*) => {
		write!($w, "{}", format_xml::format_xml!{$($tt)*})
	};
}

fn main() {
	xprintln! {
		<!DOCTYPE html>
		<html>
		<head>
			<title>"Projectile Solver"</title>
		</head>
			<body>
				|f| {stationary(f)}
				|f| {moving(f)}
				|f| {arbitrary(f)}
			</body>
		</html>
	}
}

/// Generate a sequence of timestamps from `0` to `until` with increments of `0.05` ending with the value of `until`.
fn seq(until: f32) -> impl Clone + Iterator<Item = f32> {
	let step = 0.05;
	(0..)
		.map(move |i| i as f32 * step)
		.take_while(move |&time| time < until + step)
		.map(move |time| f32::min(time, until))
}
/// Render an iterator of 2D points to SVG path syntax.
fn path(mut input: impl Iterator<Item = [f32; 2]>) -> String {
	let start = input.next().unwrap();
	let mut s = format!("M{:.2} {:.2}", start[0], start[1]);
	use std::fmt::Write;
	for p in input {
		let _ = s.write_fmt(format_args!("L{:.2} {:.2}", p[0], p[1]));
	}
	return s;
}

fn moving(f: &mut dyn fmt::Write) -> fmt::Result {
	let weapon = Weapon {
		speed: 600.0,
		gravity: 400.0,
	};
	let target = Target {
		position: [450.0, 0.0],
		velocity: [100.0, 50.0],
		..Default::default()
	};
	let s1 = Solver::lob(weapon, target).solve().unwrap();
	let s2 = Solver::optimal(weapon, target).solve().unwrap();
	xwrite! { f,
		<svg width="800" height="450" viewBox="-50 -600 800 450" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
			<g transform="translate(0, -200) scale(1,-1)">
				<line x0="-1000" x1="2000" y0="0" y1="0" stroke="black" vector-effect="non-scaling-stroke" shape-rendering="crispEdges" />
				<line y0="-1000" y1="1000" x0="0" x1="0" stroke="black" vector-effect="non-scaling-stroke" shape-rendering="crispEdges" />
				<path d={path(seq(s1.time + 0.3).map(|time| target.predict(time)))} fill="none" stroke="green" style="opacity: 0.5;" id="A_s0" />
				<path d={path(seq(s1.time).map(|time| weapon.fire(s1.angle, time)))} fill="none" stroke="red" style="opacity: 0.5;" id="A_s1" />
				<path d={path(seq(s2.time).map(|time| weapon.fire(s2.angle, time)))} fill="none" stroke="blue" style="opacity: 0.5;" id="A_s2" />
				<circle r="5" fill="green">
					<animateMotion dur={s1.time + 0.3} fill="freeze" begin="0s;A_m.end+1s">
					<mpath xlink:href="#A_s0"></mpath>
					</animateMotion>
				</circle>
				<circle r="5" fill="red">
					<animateMotion id="A_m" dur={s1.time} fill="freeze" begin="0s;A_m.end+1s">
					<mpath xlink:href="#A_s1"></mpath>
					</animateMotion>
				</circle>
				<circle r="5" fill="blue">
					<animateMotion dur={s2.time} fill="freeze" begin="0s;A_m.end+1s">
					<mpath xlink:href="#A_s2"></mpath>
					</animateMotion>
				</circle>
			</g>
		</svg>
	}
}

fn stationary(f: &mut dyn fmt::Write) -> fmt::Result {
	let weapon = Weapon {
		speed: 650.0,
		gravity: 400.0,
	};
	let target = Target {
		position: [650.0, 150.0],
		velocity: [0.0, 0.0],
		..Default::default()
	};
	let s1 = Solver::lob(weapon, target).solve().unwrap();
	let s2 = Solver::optimal(weapon, target).solve().unwrap();
	xwrite! { f,
		<svg width="800" height="450" viewBox="-50 -600 800 450" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
			<g transform="translate(0, -200) scale(1,-1)">
				<line x0="-1000" x1="2000" y0="0" y1="0" stroke="black" vector-effect="non-scaling-stroke" shape-rendering="crispEdges" />
				<line y0="-1000" y1="1000" x0="0" x1="0" stroke="black" vector-effect="non-scaling-stroke" shape-rendering="crispEdges" />
				<circle cx={target.position[0]} cy={target.position[1]} r="5" fill="green" />
				<path d={path(seq(s1.time).map(|time| weapon.fire(s1.angle, time)))} fill="none" stroke="red" style="opacity: 0.5;" id="B_s1" />
				<path d={path(seq(s2.time).map(|time| weapon.fire(s2.angle, time)))} fill="none" stroke="blue" style="opacity: 0.5;" id="B_s2" />
				<circle r="5" fill="red">
					<animateMotion id="B_m" dur={s1.time} fill="freeze" begin="0s;B_m.end+1s">
					<mpath xlink:href="#B_s1"></mpath>
					</animateMotion>
				</circle>
				<circle r="5" fill="blue">
					<animateMotion dur={s2.time} fill="freeze" begin="0s;B_m.end+1s">
					<mpath xlink:href="#B_s2"></mpath>
					</animateMotion>
				</circle>
			</g>
		</svg>
	}
}

fn arbitrary(f: &mut dyn fmt::Write) -> fmt::Result {
	let weapon = Weapon {
		speed: 650.0,
		gravity: 400.0,
	};
	let target = Target {
		position: [650.0, 50.0],
		velocity: [0.0, 0.0],
		radius: 100.0,
		..Default::default()
	};
	let s1 = Solver::lob(weapon, target).solve().unwrap();
	let s2 = Solver::optimal(weapon, target).solve().unwrap();
	xwrite! { f,
		<svg width="800" height="450" viewBox="-50 -600 800 450" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink">
			<g transform="translate(0, -200) scale(1,-1)">
				<line x0="-1000" x1="2000" y0="0" y1="0" stroke="black" vector-effect="non-scaling-stroke" shape-rendering="crispEdges" />
				<line y0="-1000" y1="1000" x0="0" x1="0" stroke="black" vector-effect="non-scaling-stroke" shape-rendering="crispEdges" />
				<path d={path(seq(s1.time + 0.3).map(|time| target.predict(time)))} fill="none" stroke="green" style="opacity: 0.5;" id="C_s0" />
				<path d={path(seq(s1.time).map(|time| weapon.fire(s1.angle, time)))} fill="none" stroke="red" style="opacity: 0.5;" id="C_s1" />
				<path d={path(seq(s2.time).map(|time| weapon.fire(s2.angle, time)))} fill="none" stroke="blue" style="opacity: 0.5;" id="C_s2" />
				<circle r="5" fill="green">
					<animateMotion dur={s1.time + 0.3} fill="freeze" begin="0s;C_m.end+1s">
					<mpath xlink:href="#C_s0"></mpath>
					</animateMotion>
				</circle>
				<circle r="5" fill="red">
					<animateMotion id="C_m" dur={s1.time} fill="freeze" begin="0s;C_m.end+1s">
					<mpath xlink:href="#C_s1"></mpath>
					</animateMotion>
				</circle>
				<circle r="5" fill="blue">
					<animateMotion dur={s2.time} fill="freeze" begin="0s;C_m.end+1s">
					<mpath xlink:href="#C_s2"></mpath>
					</animateMotion>
				</circle>
			</g>
		</svg>
	}
}

//----------------------------------------------------------------

#[derive(Copy, Clone, Debug, Default)]
pub struct Target {
	pub position: [f32; 2],
	pub velocity: [f32; 2],
	pub gravity: f32,
	pub radius: f32,
}
impl Target {
	/// Extrapolates the target under freefall `time` seconds into the future.
	pub fn predict(&self, time: f32) -> [f32; 2] {
		let x = self.position[0] + self.velocity[0] * time + self.radius * f32::cos(time);
		let y = self.position[1] + self.velocity[1] * time - self.gravity * time * time * 0.5 + self.radius * f32::sin(time);
		[x, y]
	}
}

/// Projectile Weapon stats
#[derive(Copy, Clone, Debug, Default)]
pub struct Weapon {
	/// Initial speed of the projectile when fired
	pub speed: f32,
	/// Initial gravity of the projectile when fired
	pub gravity: f32,
}
impl Weapon {
	/// Simulates firing at given angle and returns the projectile's position at given time.
	pub fn fire(&self, angle: f32, time: f32) -> [f32; 2] {
		let v0 = self.speed;
		let g = self.gravity;
		let vel_x = angle.cos() * v0;
		let vel_y = angle.sin() * v0;
		[vel_x * time, vel_y * time - 0.5 * g * time * time]
	}
}

/// Projectile aiming solution.
#[derive(Copy, Clone, Debug, Default)]
pub struct Solution {
	/// Fire at this angle.
	pub angle: f32,
	/// Projectile will hit the target at this time in the future.
	pub time: f32,
}

/// Give up when the projectile fails to connect with the target this time in the future.
pub const MAX_TIME: f32 = 5.5;
/// Time in seconds between each step of the algorithm.
pub const TIME_STEP: f32 = 0.01;

/// Projectile aim solver.
#[derive(Copy, Clone, Debug)]
pub struct Solver {
	weapon: Weapon,
	target: Target,
	solve2d: fn(f32, f32, f32, f32) -> Option<Solution>,
}
impl Solver {
	pub fn optimal(weapon: Weapon, target: Target) -> Solver {
		Solver { weapon, target, solve2d: Self::solve2d_optimal }
	}
	pub fn lob(weapon: Weapon, target: Target) -> Solver {
		Solver { weapon, target, solve2d: Self::solve2d_lob }
	}
	pub fn solve(&self) -> Option<Solution> {
		let mut target_time = 0.0;
		while target_time < MAX_TIME {
			let target_pos = self.target.predict(target_time);
			let sol = self.solve2d(target_pos)?;
			if sol.time < target_time {
				return Some(sol);
			}
			target_time += TIME_STEP;
		}
		None
	}
	fn solve2d(&self, pos: [f32; 2]) -> Option<Solution> {
		(self.solve2d)(pos[0], pos[1], self.weapon.speed, self.weapon.gravity)
	}
	// Optimal solution with the lowest projectile travel time.
	// Returns None if the projectile cannot reach the target.
	fn solve2d_optimal(x: f32, y: f32, v0: f32, g: f32) -> Option<Solution> {
		let root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0 * y * v0 * v0);
		if root < 0.0 {
			return None;
		}
		let root = f32::sqrt(root);
		let angle = f32::atan((v0 * v0 - root) / (g * x));
		let time = x / (f32::cos(angle) * v0);
		Some(Solution { angle, time })
	}
	// Lobbing solution with the highest projectile travel time.
	// Returns None if the projectile cannot reach the target.
	fn solve2d_lob(x: f32, y: f32, v0: f32, g: f32) -> Option<Solution> {
		let root = v0 * v0 * v0 * v0 - g * (g * x * x + 2.0 * y * v0 * v0);
		if root < 0.0 {
			return None;
		}
		let root = f32::sqrt(root);
		let angle = f32::atan((v0 * v0 + root) / (g * x));
		let time = x / (f32::cos(angle) * v0);
		Some(Solution { angle, time })
	}
}
