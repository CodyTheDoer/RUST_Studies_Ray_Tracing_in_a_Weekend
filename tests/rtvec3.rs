#[cfg(test)]
mod rtvec3_tests {
    use std::ops::{Add, Sub};
    use raytracing_in_a_weekend::RtVec3;

    fn setup() -> (RtVec3, RtVec3) {
        let v1 = RtVec3::new(1.0, 2.0, 3.0);
        let v2 = RtVec3::new(4.0, 5.0, 6.0);
        (v1, v2)
    }

    #[test]
    fn self_ref_check() {
        let (v1, _) = setup();
        let self_ref_check = RtVec3::new(v1.x(), v1.y(), v1.z());
        assert!(self_ref_check == v1);
    }

    #[test]
    fn add() {
        let (v1, v2) = setup();
        let v_add = RtVec3::new(5.0, 7.0, 9.0);
        assert!(v_add == v1 + v2);
        assert!(v_add == v1.add(v2));
    }

    #[test]
    fn sub() {
        let (v1, v2) = setup();
        let v_sub = RtVec3::new(-3.0, -3.0, -3.0);
        assert!(v_sub == v1 - v2);
        assert!(v_sub == v1.sub(v2));
    }

    #[test]
    fn mul() {
        let (v1, v2) = setup();
        let v_mul = RtVec3::new(4.0, 10.0, 18.0);
        assert!(v_mul == v1 * v2);
    }

    #[test]
    fn div() {
        let (v1, v2) = setup();
        let v_div = RtVec3::new(0.25, 0.4, 0.5);
        assert!(v_div == v1 / v2);
    }

    #[test]
    fn mul_s() {
        let (v1, _) = setup();
        let v_mul_s = RtVec3::new(2.0, 4.0, 6.0);
        assert!(v_mul_s == v1.multiply_scalar(2.0));
        assert!(v_mul_s == v1*2.0);
    }

    #[test]
    fn div_s() {
        let (v1, _) = setup();
        let v_div_s = RtVec3::new(0.5, 1.0, 1.5);
        assert!(v_div_s == v1.divide_scalar(2.0));
        assert!(v_div_s == v1/2.0);
    }

    #[test]
    fn length() {
        let (v1, v2) = setup();
        let known_length: f32 = 12.4499;
        let v3 = v1.add(v2);
        let length = v3.length();
        assert!(length == known_length);
    }
}