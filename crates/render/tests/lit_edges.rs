//! A lit edge drawn into real pixels: whole where it lies on the faces it
//! bounds, gone where matter stands in front of it.
//!
//! The pixel half of #990, whose transcription is in
//! `crates/app/src/screens/viewport/render/tests.rs`. The frame is built by
//! hand, each lit end pulled towards the eye as the application pulls it: by
//! the least it may for the edge that must stay whole, by the most it may for
//! the edge that must stay hidden.
//!
//! It wants a GPU adapter. The CI runner has none, so there it prints that it
//! was skipped and passes; it is held on a machine that has one.

use cao_render::{
    OrbitCamera, SceneFrame, SceneRenderer, Size, Vertex, ViewportRect, push_solid, srgb,
};
use glam::{Vec2, Vec3};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

/// The least and the most a lit end is pulled towards the eye, in pixels'
/// worth at its depth: the bounds the application's frame is held to.
const PULLED_AT_LEAST: f32 = 2.0;
const PULLED_AT_MOST: f32 = 10.0;

const LIT_WIDTH: f32 = 3.0;

fn amber() -> [f32; 4] {
    srgb(1.0, 0.70, 0.0, 1.0)
}

fn is_amber([r, g, b, _]: [u8; 4]) -> bool {
    r >= 200 && (120..=230).contains(&g) && b <= 80
}

/// The twelve triangles of a box, each turned outwards.
fn a_box(min: Vec3, max: Vec3) -> Vec<[Vec3; 3]> {
    let corner = |x: bool, y: bool, z: bool| {
        Vec3::new(
            if x { max.x } else { min.x },
            if y { max.y } else { min.y },
            if z { max.z } else { min.z },
        )
    };
    let face = |a: Vec3, b: Vec3, c: Vec3, d: Vec3| [[a, b, c], [a, c, d]];
    [
        face(
            corner(false, false, false),
            corner(false, true, false),
            corner(true, true, false),
            corner(true, false, false),
        ),
        face(
            corner(false, false, true),
            corner(true, false, true),
            corner(true, true, true),
            corner(false, true, true),
        ),
        face(
            corner(false, false, false),
            corner(true, false, false),
            corner(true, false, true),
            corner(false, false, true),
        ),
        face(
            corner(false, true, false),
            corner(false, true, true),
            corner(true, true, true),
            corner(true, true, false),
        ),
        face(
            corner(false, false, false),
            corner(false, false, true),
            corner(false, true, true),
            corner(false, true, false),
        ),
        face(
            corner(true, false, false),
            corner(true, true, false),
            corner(true, true, true),
            corner(true, false, true),
        ),
    ]
    .into_iter()
    .flatten()
    .collect()
}

/// A plate 40 by 40 and 4 thick, and on it a boss 10 wide and 16 high along
/// its whole back: an L seen end on, whose inside edge runs along x = 10 at
/// the top of the plate.
fn a_boss_on_a_plate() -> Vec<[Vec3; 3]> {
    let mut triangles = a_box(Vec3::ZERO, Vec3::new(40.0, 40.0, 4.0));
    triangles.extend(a_box(Vec3::new(0.0, 0.0, 4.0), Vec3::new(10.0, 40.0, 20.0)));
    triangles
}

const INSIDE_EDGE: [Vec3; 2] = [Vec3::new(10.0, 0.0, 4.0), Vec3::new(10.0, 40.0, 4.0)];

/// On the boss's back face, which the eye does not see: the matter between
/// them is far thicker than the most a lit end is pulled, so this holds the
/// pass's depth test, not the ceiling of the pull.
const BEHIND_THE_BOSS: [Vec3; 2] = [Vec3::new(0.0, 15.0, 8.0), Vec3::new(0.0, 35.0, 8.0)];

fn the_camera() -> OrbitCamera {
    let mut camera = OrbitCamera::default();
    camera.focus_on(
        Vec3::new(20.0, 20.0, 8.0),
        32.0,
        WIDTH as f32 / HEIGHT as f32,
    );
    camera
}

/// How thick the wall in front of a lit edge is, along the line of sight, in
/// pixels' worth: a little over the most a lit end is pulled, and over it by
/// more than the half width the line draws at its centre's depth against an
/// inclined face — at 11.5, a few pixels of the line's side showed.
const THE_WALL_S_DEPTH: f32 = 12.5;

/// A wall 30 wide and 16 high, its back face on x = 20, so thin that the
/// line of sight to `BEHIND_THE_WALL` crosses `THE_WALL_S_DEPTH` pixels' worth
/// of matter and no more.
fn a_thin_wall(camera: &OrbitCamera) -> Vec<[Vec3; 3]> {
    let [start, end] = BEHIND_THE_WALL;
    let middle = start.lerp(end, 0.5);
    let sight = (camera.eye() - middle).normalize();
    let along_the_sight = a_pixel_s_worth(camera, middle) * THE_WALL_S_DEPTH;
    let thickness = along_the_sight * sight.x;
    a_box(
        Vec3::new(20.0, 5.0, 0.0),
        Vec3::new(20.0 + thickness, 35.0, 16.0),
    )
}

const BEHIND_THE_WALL: [Vec3; 2] = [Vec3::new(20.0, 15.0, 8.0), Vec3::new(20.0, 25.0, 8.0)];

fn a_pixel_s_worth(camera: &OrbitCamera, point: Vec3) -> f32 {
    let depth = (point - camera.eye()).dot(camera.forward());
    camera.world_units_per_pixel(HEIGHT as f32) * depth / camera.distance()
}

/// `point` pulled towards the eye along its own line of sight by `pixels`
/// pixels' worth at its depth: it lands on the same pixel, nearer.
fn pulled(camera: &OrbitCamera, point: Vec3, pixels: f32) -> Vec3 {
    point + (camera.eye() - point).normalize() * a_pixel_s_worth(camera, point) * pixels
}

fn a_frame(
    camera: &OrbitCamera,
    matter: &[[Vec3; 3]],
    lit: [Vec3; 2],
    pulled_by: f32,
) -> SceneFrame {
    let mut solids = Vec::new();
    push_solid(
        &mut solids,
        matter,
        srgb(0.62, 0.64, 0.68, 1.0),
        camera.forward(),
    );
    let lit_lines = lit
        .map(|end| Vertex::line(pulled(camera, end, pulled_by), amber(), LIT_WIDTH))
        .to_vec();
    let whole = ViewportRect {
        x: 0.0,
        y: 0.0,
        width: WIDTH as f32,
        height: HEIGHT as f32,
    };
    SceneFrame {
        scene_view_projection: camera.view_projection(WIDTH as f32 / HEIGHT as f32),
        scene_viewport: whole,
        scene_background: Vec::new(),
        scene_world_lines: Vec::new(),
        scene_surfaces: Vec::new(),
        scene_veiled_matter: 0..0,
        scene_solids: solids,
        scene_lines: Vec::new(),
        scene_lit_lines: lit_lines,
        cube_view_projection: glam::Mat4::IDENTITY,
        cube_triangles: Vec::new(),
        cube_edges: Vec::new(),
        cube_viewport: ViewportRect {
            x: 0.0,
            y: 0.0,
            width: 0.0,
            height: 0.0,
        },
    }
}

struct Picture(Vec<u8>);

impl Picture {
    fn at_point(&self, at: Vec2) -> Option<[u8; 4]> {
        let inside = (0.0..WIDTH as f32).contains(&at.x) && (0.0..HEIGHT as f32).contains(&at.y);
        inside.then(|| self.at(at.x as u32, at.y as u32))
    }

    fn at(&self, x: u32, y: u32) -> [u8; 4] {
        let start = ((y * WIDTH + x) * 4) as usize;
        [
            self.0[start],
            self.0[start + 1],
            self.0[start + 2],
            self.0[start + 3],
        ]
    }
}

/// Where a point of the world lands in the picture, in pixels.
fn on_screen(camera: &OrbitCamera, point: Vec3) -> Vec2 {
    let ndc = camera
        .view_projection(WIDTH as f32 / HEIGHT as f32)
        .project_point3(point);
    Vec2::new(
        (ndc.x + 1.0) * 0.5 * WIDTH as f32,
        (1.0 - ndc.y) * 0.5 * HEIGHT as f32,
    )
}

fn a_gpu() -> Option<(wgpu::Device, wgpu::Queue)> {
    pollster::block_on(async {
        let adapter = wgpu::Instance::default()
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .ok()?;
        adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .ok()
    })
}

fn drawn(device: &wgpu::Device, queue: &wgpu::Queue, frame: &SceneFrame) -> Picture {
    let mut renderer = SceneRenderer::new(device, FORMAT, 1);
    let size = Size {
        width: WIDTH,
        height: HEIGHT,
        format: FORMAT,
    };
    let background = wgpu::Color {
        r: 0.11,
        g: 0.12,
        b: 0.14,
        a: 1.0,
    };
    Picture(cao_render::draw(
        device,
        queue,
        &mut renderer,
        frame,
        size,
        background,
    ))
}

#[test]
fn a_lit_inside_edge_is_drawn_whole_and_a_lit_edge_behind_the_matter_not_at_all() {
    let Some((device, queue)) = a_gpu() else {
        println!("skipped: no GPU adapter here to draw a lit edge with");
        return;
    };
    let camera = the_camera();

    let inside = drawn(
        &device,
        &queue,
        &a_frame(&camera, &a_boss_on_a_plate(), INSIDE_EDGE, PULLED_AT_LEAST),
    );
    let [start, end] = INSIDE_EDGE.map(|end| on_screen(&camera, end));
    let across = (end - start).perp().normalize();
    for (row, offset) in [
        ("on one side of", -1.0),
        ("along the centre of", 0.0),
        ("on the other side of", 1.0),
    ] {
        let along: Vec<[u8; 4]> = (5..=95)
            .filter_map(|percent| {
                inside.at_point(start.lerp(end, percent as f32 / 100.0) + across * offset)
            })
            .collect();
        let lit = along.iter().filter(|pixel| is_amber(**pixel)).count();
        assert!(
            along.len() > 80 && lit * 10 >= along.len() * 9,
            "the inside edge where the boss meets the plate, lit and pulled {PULLED_AT_LEAST} \
             pixels' worth towards the eye, is not drawn amber {row} its {LIT_WIDTH} pixels of \
             width: {lit} of {} pixels are, of 91 on screen",
            along.len(),
        );
    }

    let behind = drawn(
        &device,
        &queue,
        &a_frame(
            &camera,
            &a_boss_on_a_plate(),
            BEHIND_THE_BOSS,
            PULLED_AT_MOST,
        ),
    );
    let mut amber_pixels = Vec::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if is_amber(behind.at(x, y)) {
                amber_pixels.push((x, y));
            }
        }
    }
    assert!(
        amber_pixels.is_empty(),
        "a lit edge on the far face of the boss, pulled {PULLED_AT_MOST} pixels' worth towards \
         the eye, still shows through the matter in front of it: {} amber pixels, first {:?}",
        amber_pixels.len(),
        amber_pixels.first(),
    );
}

#[test]
fn a_lit_edge_behind_a_wall_a_little_thicker_than_the_most_it_is_pulled_is_not_drawn() {
    let Some((device, queue)) = a_gpu() else {
        println!("skipped: no GPU adapter here to draw a lit edge with");
        return;
    };
    let camera = the_camera();
    let wall = a_thin_wall(&camera);
    let amber_in = |picture: &Picture| -> Vec<(u32, u32)> {
        (0..HEIGHT)
            .flat_map(|y| (0..WIDTH).map(move |x| (x, y)))
            .filter(|(x, y)| is_amber(picture.at(*x, *y)))
            .collect()
    };
    let bare = drawn(
        &device,
        &queue,
        &a_frame(&camera, &[], BEHIND_THE_WALL, PULLED_AT_MOST),
    );
    assert!(
        amber_in(&bare).len() > 100,
        "the fixture wants the edge drawn plainly with no wall in front of it, and {} pixels \
         of it are amber",
        amber_in(&bare).len(),
    );

    let behind = drawn(
        &device,
        &queue,
        &a_frame(&camera, &wall, BEHIND_THE_WALL, PULLED_AT_MOST),
    );
    let amber_pixels = amber_in(&behind);
    assert!(
        amber_pixels.is_empty(),
        "a lit edge behind a wall {THE_WALL_S_DEPTH} pixels' worth thick along the line of \
         sight, pulled {PULLED_AT_MOST} pixels' worth towards the eye, shows through it: {} \
         amber pixels, first {:?}",
        amber_pixels.len(),
        amber_pixels.first(),
    );
}
