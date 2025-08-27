use std::collections::HashSet;

pub type Point = (i32, i32, i32);
pub type Size = (i32, i32, i32);

struct EllipseParameters {
	centerx: f32,
	centerz: f32,
	dx2: f32,
	dz2: f32,
	davg: i32,
}

const FACTOR: f32 = 1.7_f32;

pub fn ellipse(width: u32, depth: u32) -> HashSet<Point> {
	let dx = width as i32;
	let dz = depth as i32;

	let params = EllipseParameters {
		centerx: if dx % 2 == 0 { -0.5_f32 } else { 0_f32 },
		centerz: if dz % 2 == 0 { -0.5_f32 } else { 0_f32 },
		dx2: if dx == 0 { 0.5_f32 } else { ((dx as f32 + FACTOR) * (dx as f32 + FACTOR)) / 4.0_f32 },
		dz2: if dz == 0 { 0.5_f32 } else { ((dz as f32 + FACTOR) * (dz as f32 + FACTOR)) / 4.0_f32 },
		davg: ((dx as f32 + dz as f32 + FACTOR * 2_f32) / 2_f32) as i32,
	};

	let mut res = HashSet::new();

	for ox in 0..dx {
		let x = ox - dx / 2;

		for oz in 0..dz {
			let z = oz - dz / 2;

			if is_inside(&params, x, z) {
				res.insert((x, 0, z));
			}
		}
	}

	res
}

pub fn neighbours(point: &Point, set: &HashSet<Point>) -> u8 {
	let top = (point.0, point.1, point.2 + 1);
	let bottom = (point.0, point.1, point.2 - 1);
	let right = (point.0 + 1, point.1, point.2);
	let left = (point.0 - 1, point.1, point.2);

	vec!(set.contains(&top), set.contains(&bottom), set.contains(&left), set.contains(&right))
		.iter()
		.filter(|&x| *x)
		.count() as u8
}

fn is_inside(params: &EllipseParameters, x: i32, z: i32) -> bool {
	let distance = f32::sqrt(squared_distance_2d(params.centerx, params.centerz, x as f32, z as f32, params.dx2, params.dz2));
	let d1 = (distance * (params.davg / 2 + 1) as f32) as i32;
	let d2 = params.davg / 2 - 1;

	d1 <= d2
}

fn squared_distance_2d(cx: f32, cz: f32, x1: f32, z1: f32, dx2: f32, dz2: f32) -> f32 {
	(x1 - cx) * (x1 - cx) / dx2 + (z1 - cz) * (z1 - cz) / dz2
}
