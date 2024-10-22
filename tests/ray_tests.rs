#[cfg(test)]
mod ray_tests {
    use raytracing_in_a_weekend::{Ray, RtVec3, Point3};

    fn setup() -> (Ray, Ray) {
        let origin = Point3::new(0.0, 1.0, 2.0);
        let direction = RtVec3::new(3.0, 4.0, 5.0);
        let ray_1 = Ray::new(origin, direction);
        let ray_2 = Ray::new(origin, direction);
        (ray_1, ray_2)
    }

    #[test]
    fn self_ref_check() {
        let (ray_1, ray_2) = setup();
        assert_eq!(ray_1.origin(), ray_2.origin());
        assert_eq!(ray_1.direction(), ray_2.direction());
    }

    #[test]
    fn point_at_parameter() {
        let (ray, _) = setup();

        // Test t = 0.0
        let point_at_0 = ray.at(0.0);
        assert_eq!(point_at_0, ray.origin());

        // Test t = 1.0
        let expected_point = ray.origin() + ray.direction();
        assert_eq!(ray.at(1.0), expected_point);

        // Test t = 2.0
        let expected_point_2 = ray.origin() + 2.0 * ray.direction();
        assert_eq!(ray.at(2.0), expected_point_2);
    }

    #[test]
    fn point_at_negative_parameter() {
        let (ray, _) = setup();

        // Test t = -1.0
        let t = -1.0;
        let expected_point = ray.origin() + t * ray.direction();
        assert_eq!(ray.at(t), expected_point)
    }

    #[test] 
    fn point_at_large_parameter() {
        let (ray, _) = setup();

        // Test large t value
        let t = 1e6;
        let expected_point = ray.origin() + t * ray.direction();
        assert_eq!(ray.at(t), expected_point)
    }
}