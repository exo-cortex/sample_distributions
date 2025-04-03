use std::{
    fs::File,
    io::{BufWriter, Write},
};

use rand::{rngs::SmallRng, Rng, SeedableRng};

mod aabb;
mod distribution;
mod point;
mod sampling_task;

use crate::{
    aabb::{AxisAlignedBoundingBox, AxisAlignedBoundingBoxBuilder},
    distribution::{sample, NormalDistributionBuilder, RingNormalDistributionBuilder},
    point::Point2D,
    sampling_task::SamplingTask,
};

fn main() {
    let mut rng = SmallRng::seed_from_u64(0);

    let sample_area = AxisAlignedBoundingBoxBuilder::new()
        .with_center_coordinates(0.0, 0.0)
        .with_width_and_height(12.0, 12.0)
        .build();

    println!("sampling area: {}", &sample_area);

    let normal_distribution = NormalDistributionBuilder::new()
        .with_center_coordinates(0.0, 0.0)
        .with_sigma(0.25)
        .build();

    let ring_distribution_1 = RingNormalDistributionBuilder::new()
        .with_center_coordinates(0.0, 0.0)
        .with_radius(2.0)
        .with_radial_sigma(0.4)
        .build();

    let ring_distribution_2 = RingNormalDistributionBuilder::new()
        .with_center_coordinates(3.5, 3.5)
        .with_radius(1.0)
        .with_radial_sigma(0.35)
        .build();

    let tasks = [
        SamplingTask::new(&normal_distribution, 200000, "1"),
        SamplingTask::new(&ring_distribution_1, 20000, "2"),
        SamplingTask::new(&ring_distribution_2, 150000, "3"),
    ];

    sample_and_save_tasks(&sample_area, &tasks, "combined.txt", &mut rng);
}

pub fn sample_and_save_tasks(
    sample_area: &AxisAlignedBoundingBox,
    tasks: &[SamplingTask<'_>],
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
