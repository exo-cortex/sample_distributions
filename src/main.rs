use std::{
    fs::File,
    io::{BufWriter, Write},
};

use crate::{
    aabb::AxisAlignedBoundingBox,
    distribution::{Distribution, NormalDistributionBuilder},
    point::Point2D,
};

use aabb::AxisAlignedBoundingBoxBuilder;
use distribution::{sample, RingNormalDistributionBuilder};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sampling_task::SamplingTask;

mod aabb;
mod distribution;
mod point;
mod sampling_task;

fn main() {
    let mut rng = SmallRng::seed_from_u64(0);

    let sample_area = AxisAlignedBoundingBoxBuilder::new()
        .with_center_coordinates(2.0, 2.0)
        .with_width_and_height(1.0, 1.0)
        .build();

    let normal_distribution = NormalDistributionBuilder::new()
        .with_center_coordinates(2.0, 2.0)
        .with_sigma(1.0)
        .build();

    let ring_distribution = RingNormalDistributionBuilder::new()
        .with_center_coordinates(1.0, 0.5)
        .with_radius(2.0)
        .with_radial_sigma(0.5)
        .build();

    sample_and_save_tasks(
        &sample_area,
        &[
            SamplingTask::new(&normal_distribution, 1000, "b"),
            SamplingTask::new(&ring_distribution, 1000, "a"),
        ],
        "combined.txt",
        &mut rng,
    );
}

pub fn sample_and_save_tasks<'a>(
    sample_area: &AxisAlignedBoundingBox,
    tasks: &[SamplingTask<'a>],
    filename: &str,
    rng: &mut impl Rng,
) {
    let filename = format!("./{}", filename);
    let file = File::create(filename).unwrap();
    let mut buffer = BufWriter::new(file);

    for task in tasks {
        (0..task.get_num_sampling_attempts())
            .filter_map(|_| {
                let random_point = Point2D::random_uniform_in_aabb(sample_area, rng);
                sample(random_point, task.get_distribution(), rng)
            })
            .for_each(|p| writeln!(&mut buffer, "{}\t{}\t{}", p.x, p.y, task.get_label()).unwrap());
    }
}
