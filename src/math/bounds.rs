use sdl3::render::{FPoint, FRect};

pub trait Bounds {
    fn top(&self) -> f32;
    fn bottom(&self) -> f32;
    fn left(&self) -> f32;
    fn right(&self) -> f32;
    fn center(&self) -> FPoint;

    fn is_inside(&self, point: FPoint) -> bool {
        let top = self.top();
        let bottom = self.bottom();
        let left = self.left();
        let right = self.right();

        point.x >= left && point.x <= right && point.y >= top && point.y <= bottom
    }

    fn intersects<T: Bounds>(&self, other: T) -> bool {
        let top = other.top();
        let bottom = other.bottom();
        let left = other.left();
        let right = other.right();

        let s_top = self.top();
        let s_bottom = self.bottom();
        let s_left = self.left();
        let s_right = self.right();

        let any_point_inside_self = self.is_inside(FPoint {
            x: left,
            y: top,
        }) || self.is_inside(FPoint {
            x: right,
            y: top,
        }) || self.is_inside(FPoint {
            x: left,
            y: bottom,
        }) || self.is_inside(FPoint {
            x: right,
            y: bottom,
        });

        let any_point_inside_other = other.is_inside(FPoint {
            x: s_left,
            y: s_top,
        }) || other.is_inside(FPoint {
            x: s_right,
            y: s_top,
        }) || other.is_inside(FPoint {
            x: s_left,
            y: s_bottom,
        }) || other.is_inside(FPoint {
            x: s_right,
            y: s_bottom,
        });

        let vertical_area_overlaps =
            s_top > top && s_bottom < bottom && (s_left < left && s_right > left || s_left < right && s_right > right);

        let horizontal_area_overlaps =
            s_left > left && s_right < right && (s_top < top && s_bottom > top || s_top < bottom && s_bottom > bottom);

        any_point_inside_self || any_point_inside_other || vertical_area_overlaps || horizontal_area_overlaps
    }
}

impl Bounds for FRect {
    fn top(&self) -> f32 {
        if self.h < 0.0 {
            self.y + self.h
        } else {
            self.y
        }
    }

    fn bottom(&self) -> f32 {
        if self.h < 0.0 {
            self.y
        } else {
            self.y + self.h
        }
    }

    fn left(&self) -> f32 {
        if self.w < 0.0 {
            self.x + self.w
        } else {
            self.x
        }
    }

    fn right(&self) -> f32 {
        if self.w < 0.0 {
            self.x
        } else {
            self.x + self.w
        }
    }

    fn center(&self) -> FPoint {
        FPoint {
            x: (self.left() + self.right())/2.0,
            y: (self.top() + self.bottom())/2.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use sdl3::render::{FPoint, FRect};
    use crate::math::bounds::Bounds;

    #[test]
    fn test_bounds() {
        let b1 = FRect { x: 0.0, y: 0.0, w: 10.0, h: 10.0 };
        let b2 = FRect { x: 10.0, y: 10.0, w: -10.0, h: -10.0 };

        assert_eq!(b1.top(), 0.0);
        assert_eq!(b1.left(), 0.0);
        assert_eq!(b1.right(), 10.0);
        assert_eq!(b1.bottom(), 10.0);

        assert_eq!(b2.top(), 0.0);
        assert_eq!(b2.left(), 0.0);
        assert_eq!(b2.right(), 10.0);
        assert_eq!(b2.bottom(), 10.0);
    }

    #[test]
    fn test_inside() {
        let bounds = FRect { x: 0.0, y: 0.0, w: 10.0, h: 10.0 };
        let p1 = FPoint { x: 0.0, y: 0.0 };
        let p2 = FPoint { x: 10.0, y: 10.0 };
        let p3 = FPoint { x: 5.0, y: 5.0 };
        let p4 = FPoint { x: 0.0, y: 100.0 };
        let p5 = FPoint { x: 100.0, y: 0.0 };
        let p6 = FPoint { x: -10.0, y: 0.0 };

        assert!(bounds.is_inside(p1));
        assert!(bounds.is_inside(p2));
        assert!(bounds.is_inside(p3));
        assert!(!bounds.is_inside(p4));
        assert!(!bounds.is_inside(p5));
        assert!(!bounds.is_inside(p6));
    }

    #[test]
    fn test_inside_inverse() {
        let bounds = FRect { x: 10.0, y: 10.0, w: -10.0, h: -10.0 };
        let p1 = FPoint { x: 0.0, y: 0.0 };
        let p2 = FPoint { x: 10.0, y: 10.0 };
        let p3 = FPoint { x: 5.0, y: 5.0 };
        let p4 = FPoint { x: 0.0, y: 100.0 };
        let p5 = FPoint { x: 100.0, y: 0.0 };
        let p6 = FPoint { x: -10.0, y: 0.0 };

        assert!(bounds.is_inside(p1));
        assert!(bounds.is_inside(p2));
        assert!(bounds.is_inside(p3));
        assert!(!bounds.is_inside(p4));
        assert!(!bounds.is_inside(p5));
        assert!(!bounds.is_inside(p6));
    }

    #[test]
    fn test_intersects() {
        let canvas = FRect { x: 0.0, y: 0.0, w: 10.0, h: 10.0 };

        // r1 is completely inside of canvas
        let r1 = FRect { x: 0.0, y: 0.0, w: 1.0, h: 1.0 };

        // a point of r2 is inside of canvas
        let r2 = FRect { x: -1.0, y: -1.0, w: 1.0, h: 1.0 };

        // no point of r4 is inside canvas
        let r3 = FRect { x: 11.0, y: 0.0, w: 1.0, h: 1.0 };

        // r4 and canvas form a cross
        let r4 = FRect { x: 4.0, y: -1.0, w: 1.0, h: 20.0 };

        // r5.x is between canvas.left and canvas.right, but there is no overlap
        let r5 = FRect { x: 4.0, y: 100.0, w: 1.0, h: 1.0 };

        assert!(canvas.intersects(r1));
        assert!(canvas.intersects(r2));
        assert!(!canvas.intersects(r3));
        assert!(canvas.intersects(r4));
        assert!(!canvas.intersects(r5));
    }
}