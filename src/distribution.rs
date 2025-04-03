use crate::point::Point2D;

use rand::Rng;

pub fn sample(p: Point2D, distribution: &dyn Distribution, rng: &mut impl Rng) -> Option<Point2D> {
    let random_number = rng.gen_range(0.0..1.0);
    if random_number < distribution.probability_at(&p) {
        Some(p)
    } else {
        None
    }
}

pub trait Distribution: 'static {
    fn probability_at(&self, p: &Point2D) -> f64;
}

pub struct NormalDistribution {
    center: Point2D,
    sigma: f64,
}

impl Distribution for NormalDistribution {
    fn probability_at(&self, p: &Point2D) -> f64 {
        let distance_from_center =
            ((p.x - self.center.x).powi(2) + (p.y - self.center.y).powi(2)).sqrt();

        (-distance_from_center / self.sigma).exp()
    }
}

pub struct NormalDistributionBuilder {
    pub center: Option<Point2D>,
    pub sigma: Option<f64>,
}

#[allow(dead_code)]
impl NormalDistributionBuilder {
    pub fn new() -> Self {
        NormalDistributionBuilder {
            center: None,
            sigma: None,
        }
    }

    pub fn with_center_coordinates(mut self, center_x: f64, center_y: f64) -> Self {
        self.center = Some(Point2D::new(center_x, center_y));
        self
    }

    pub fn with_center_point(mut self, center_point: Point2D) -> Self {
        self.center = Some(center_point);
        self
    }

    pub fn with_sigma(mut self, sigma: f64) -> Self {
        self.sigma = Some(sigma);
        self
    }

    pub fn build(self) -> NormalDistribution {
        let center = self.center.unwrap_or_default();
        let sigma = self.sigma.unwrap_or(1.0);
        NormalDistribution { center, sigma }
    }
}

pub struct RingNormalDistribution {
    center: Point2D,
    radius: f64,
    radial_sigma: f64,
}

impl Distribution for RingNormalDistribution {
    fn probability_at(&self, p: &Point2D) -> f64 {
        let distance_from_center =
            ((p.x - self.center.x).powi(2) + (p.y - self.center.y).powi(2)).sqrt();

        (-((distance_from_center - self.radius) / self.radial_sigma).powi(2)).exp()
    }
}

pub struct RingNormalDistributionBuilder {
    center: Option<Point2D>,
    radius: Option<f64>,
    radial_sigma: Option<f64>,
}

#[allow(dead_code)]
impl RingNormalDistributionBuilder {
    pub fn new() -> Self {
        RingNormalDistributionBuilder {
            center: None,
            radius: None,
            radial_sigma: None,
        }
    }

    pub fn with_center_coordinates(mut self, center_x: f64, center_y: f64) -> Self {
        self.center = Some(Point2D::new(center_x, center_y));
        self
    }

    pub fn with_center_point(mut self, center_point: Point2D) -> Self {
        self.center = Some(center_point);
        self
    }

    pub fn with_radius(mut self, radius: f64) -> Self {
        self.radius = Some(radius);
        self
    }

    pub fn with_radial_sigma(mut self, radial_sigma: f64) -> Self {
        self.radial_sigma = Some(radial_sigma);
        self
    }

    pub fn build(self) -> RingNormalDistribution {
        let center = self.center.unwrap_or_default();
        let radius = self.radius.unwrap_or(1.0);
        let radial_sigma = self.radial_sigma.unwrap_or(0.1);
        RingNormalDistribution {
            center,
            radius,
            radial_sigma,
        }
    }
}

pub struct LineDistribution {
    start_point: Point2D,
    end_point: Point2D,
    sigma: f64,
}

pub struct LineDistributionBuilder {
    build_method: LineDistributionBuilderMethod,
    start_point: Option<Point2D>,
    end_point: Option<Point2D>,
    middle_point: Option<Point2D>,
    separation: Option<f64>,
    angle: Option<f64>,
    sigma: Option<f64>,
}

impl Distribution for LineDistribution {
    fn probability_at(&self, p: &Point2D) -> f64 {
        let distance_from_line = (self.end_point.y - self.start_point.y) * p.x
            - (self.end_point.x - self.start_point.x) * p.y
            + self.end_point.x * self.start_point.y
            - self.end_point.y * self.start_point.x
                / ((self.end_point.y - self.start_point.y).powi(2)
                    + (self.end_point.x - self.start_point.x).powi(2))
                .sqrt();
        let value = 0.0;

        todo!("condition for if point is not in band parallel to line connecting start and end");
        // value
    }
}

enum LineDistributionBuilderMethod {
    Unset,
    TwoPoints,
    MiddleSeparationAngle,
}

impl LineDistributionBuilder {
    pub fn new() -> Self {
        LineDistributionBuilder {
            build_method: LineDistributionBuilderMethod::Unset,
            start_point: None,
            end_point: None,
            middle_point: None,
            separation: None,
            angle: None,
            sigma: None,
        }
    }
    pub fn from_points(mut self, start_point: Point2D, end_point: Point2D) -> Self {
        self.build_method = LineDistributionBuilderMethod::TwoPoints;
        self.start_point = Some(start_point);
        self.end_point = Some(end_point);

        self
    }
    pub fn from_middle_distance_angle(
        mut self,
        middle_point: Point2D,
        separation: f64,
        angle: f64,
    ) -> Self {
        self.build_method = LineDistributionBuilderMethod::MiddleSeparationAngle;
        self.middle_point = Some(middle_point);
        self.separation = Some(separation);
        self.angle = Some(angle);
        self
    }
    pub fn with_sigma(mut self, sigma: f64) -> Self {
        self.sigma = Some(sigma);
        self
    }

    pub fn build(mut self) -> LineDistribution {
        match self.build_method {
            LineDistributionBuilderMethod::Unset | LineDistributionBuilderMethod::TwoPoints => {
                LineDistribution {
                    start_point: self.start_point.unwrap_or(Point2D { x: -1.0, y: 0.0 }),
                    end_point: self.end_point.unwrap_or(Point2D { x: 1.0, y: 0.0 }),
                    sigma: self.sigma.unwrap_or(0.25),
                }
            }
            LineDistributionBuilderMethod::MiddleSeparationAngle => {
                let middle = self.middle_point.unwrap_or(Point2D { x: 0.0, y: 0.0 });
                let separation = self.separation.unwrap_or(1.0);
                let angle = self.angle.unwrap_or(0.0);
                let start_point = middle
                    + Point2D {
                        x: separation * angle.cos(),
                        y: separation * angle.sin(),
                    };
                let end_point = middle
                    + Point2D {
                        x: -separation * angle.cos(),
                        y: separation * angle.sin(),
                    };

                LineDistribution {
                    start_point,
                    end_point,
                    sigma: self.sigma.unwrap_or(0.25),
                }
            }
        }
    }
}
