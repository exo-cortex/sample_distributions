use core::fmt;

use crate::point::Point2D;

pub struct AxisAlignedBoundingBox {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

#[allow(dead_code)]
enum BoxBuilderMethod {
    Unset,
    CenterWidthHeight,
    MinAndMaxCoordinates,
}

pub struct AxisAlignedBoundingBoxBuilder {
    method: BoxBuilderMethod,
    center: Option<Point2D>,
    width: Option<f64>,
    height: Option<f64>,
    min_x: Option<f64>,
    max_x: Option<f64>,
    min_y: Option<f64>,
    max_y: Option<f64>,
}

#[allow(dead_code)]
impl AxisAlignedBoundingBoxBuilder {
    pub fn new() -> Self {
        AxisAlignedBoundingBoxBuilder {
            method: BoxBuilderMethod::Unset,
            center: None,
            width: None,
            height: None,
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
        }
    }

    pub fn with_center_coordinates(mut self, center_x: f64, center_y: f64) -> Self {
        self.method = BoxBuilderMethod::CenterWidthHeight;
        self.center = Some(Point2D::new(center_x, center_y));
        self
    }

    pub fn with_center_point(mut self, center_point: Point2D) -> Self {
        self.method = BoxBuilderMethod::CenterWidthHeight;
        self.center = Some(center_point);
        self
    }

    pub fn with_width_and_height(mut self, width: f64, height: f64) -> Self {
        self.method = BoxBuilderMethod::CenterWidthHeight;
        self.width = Some(width);
        self.height = Some(height);
        self
    }

    pub fn build(self) -> AxisAlignedBoundingBox {
        match self.method {
            BoxBuilderMethod::Unset | BoxBuilderMethod::CenterWidthHeight => {
                let center = self.center.unwrap_or_default();
                let width = self.width.unwrap_or(1.0);
                let height = self.height.unwrap_or(1.0);
                AxisAlignedBoundingBox {
                    min_x: center.x - width * 0.5,
                    max_x: center.x + width * 0.5,
                    min_y: center.y - height * 0.5,
                    max_y: center.y + height * 0.5,
                }
            }
            BoxBuilderMethod::MinAndMaxCoordinates => AxisAlignedBoundingBox {
                min_x: self.min_x.unwrap_or(-1.0),
                max_x: self.max_x.unwrap_or(1.0),
                min_y: self.min_y.unwrap_or(-1.0),
                max_y: self.max_y.unwrap_or(1.0),
            },
        }
    }
}

impl std::fmt::Display for AxisAlignedBoundingBox {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(x: {}..{}), y: ({}..{})",
            self.min_x, self.max_x, self.min_y, self.max_y
        )
    }
}
