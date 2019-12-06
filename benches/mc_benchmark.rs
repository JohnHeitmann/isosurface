use criterion::{black_box, criterion_group, criterion_main, Criterion};
use isosurface::marching_cubes::MarchingCubes;
use isosurface::linear_hashed_marching_cubes::LinearHashedMarchingCubes;
use isosurface::source::{CentralDifference, Source};

pub struct Sphere {}

impl Source for Sphere {
    fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        let x = x - 0.5;
        let y = y - 0.5;
        let z = z - 0.5;
        (x * x + y * y + z * z).sqrt() - 0.5
    }
}

pub struct ExpensiveSource {}

impl Source for ExpensiveSource {
    fn sample(&self, x: f32, y: f32, z: f32) -> f32 {
        // use sphere sdf logic, but fake up a bunch of divisions as
        // well to simulate an expensive source calc
        let x = x - 0.5;
        let y = y - 0.5;
        let z = z - 0.5;
        let real_answer: f32 = (x * x + y * y + z * z).sqrt() - 0.5;
    
        let mut fake_stuff = 0f32;
        for i in 0..100 {
            fake_stuff += 1f32 / (i as f32 + 10000000f32);
        }
        real_answer + fake_stuff
    }
}

fn bench_mc_sphere(n: usize) -> usize {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut marching_cubes = MarchingCubes::new(n);
    let source = Sphere {};
    marching_cubes.extract(&source, &mut vertices, &mut indices);
    vertices.len()
}

fn bench_mc_sphere_normals(n: usize) -> usize {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut marching_cubes = MarchingCubes::new(n);
    let source = CentralDifference::new(Box::new(Sphere {}));
    marching_cubes.extract_with_normals(&source, &mut vertices, &mut indices);
    vertices.len()
}

fn bench_lmc_sphere(n: usize) -> usize {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut marching_cubes = LinearHashedMarchingCubes::new(n);
    let source = Sphere {};
    marching_cubes.extract(&source, &mut vertices, &mut indices);
    vertices.len()
}

fn bench_lmc_sphere_normals(n: usize) -> usize {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut marching_cubes = LinearHashedMarchingCubes::new(n);
    let source = CentralDifference::new(Box::new(Sphere {}));
    marching_cubes.extract_with_normals(&source, &mut vertices, &mut indices);
    vertices.len()
}

fn bench_mc_expensive_sdf(n: usize) -> usize {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut marching_cubes = MarchingCubes::new(n);
    let source = ExpensiveSource {};
    marching_cubes.extract(&source, &mut vertices, &mut indices);
    vertices.len()
}

fn bench_lmc_expensive_sdf(n: usize) -> usize {
    let mut vertices = vec![];
    let mut indices = vec![];
    let mut marching_cubes = LinearHashedMarchingCubes::new(n);
    let source = ExpensiveSource {};
    marching_cubes.extract(&source, &mut vertices, &mut indices);
    vertices.len()
}



fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("mc sphere 16", |b| b.iter(|| bench_mc_sphere(black_box(16))));
    c.bench_function("mc sphere 32", |b| b.iter(|| bench_mc_sphere(black_box(32))));
    c.bench_function("mc expensive 16", |b| b.iter(|| bench_mc_expensive_sdf(black_box(16))));    
    c.bench_function("mc expensive 32", |b| b.iter(|| bench_mc_expensive_sdf(black_box(32))));    

    c.bench_function("mc sphere normals 16", |b| b.iter(|| bench_mc_sphere_normals(black_box(16))));
    c.bench_function("mc expensive normals 16", |b| b.iter(|| bench_lmc_sphere_normals(black_box(16))));

    c.bench_function("lmc sphere 4", |b| b.iter(|| bench_lmc_sphere(black_box(4))));
    c.bench_function("lmc sphere 5", |b| b.iter(|| bench_lmc_sphere(black_box(5))));
    c.bench_function("lmc expensive 4", |b| b.iter(|| bench_lmc_expensive_sdf(black_box(4))));    
    c.bench_function("lmc expensive 5", |b| b.iter(|| bench_lmc_expensive_sdf(black_box(5))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
