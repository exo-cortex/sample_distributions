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

    pub fn with_radius(mut self, radial_sigma: f64) -> Self {
        self.radial_sigma = Some(radial_sigma);
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

impl Distribution for RingNormalDistribution {
    fn probability_at(&self, p: &Point2D) -> f64 {
        let distance_from_center =
            ((p.x - self.center.x).powi(2) + (p.y - self.center.y).powi(2)).sqrt();

        (-(distance_from_center / self.radial_sigma - self.radius)).exp()
    }
}
