#[cfg(test)]
mod rtvec3_tests {
    use std::ops::{Add, Sub, Mul, Div, Neg};
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
        let add = RtVec3::new(5.0, 7.0, 9.0);
        assert!(add == v1 + v2);
        assert!(add == v1.add(v2));
    }

    #[test]
    fn sub() {
        let (v1, v2) = setup();
        let sub = RtVec3::new(-3.0, -3.0, -3.0);
        assert!(sub == v1 - v2);
        assert!(sub == v1.sub(v2));
    }

    #[test]
    fn mul() {
        let (v1, v2) = setup();
        let mul = RtVec3::new(4.0, 10.0, 18.0);
        assert!(mul == v1 * v2);
        assert!(mul == v1.mul(v2));
    }

    #[test]
    fn div() {
        let (v1, v2) = setup();
        let div = RtVec3::new(0.25, 0.4, 0.5);
        assert!(div == v1 / v2);
        assert!(div == v1.div(v2));
    }

    #[test]
    fn mul_s() {
        let (v1, _) = setup();
        let mul_s = RtVec3::new(2.0, 4.0, 6.0);
        assert!(mul_s == v1.multiply_scalar(2.0));
        assert!(mul_s == v1*2.0);
    }

    #[test]
    fn div_s() {
        let (v1, _) = setup();
        let div_s = RtVec3::new(0.5, 1.0, 1.5);
        assert!(div_s == v1.divide_scalar(2.0));
        assert!(div_s == v1/2.0);
    }

    #[test]
    fn length() {
        let (v1, v2) = setup();
        let known_length: f32 = 12.4499;
        let v3 = v1.add(v2);
        let length = v3.length();
        assert!(length == known_length);
    }

    #[test]
    fn dot() {
        let (v1, v2) = setup();
        let dot: f32 = 32.0;
        assert!(dot == v1.dot(&v2));
    }

    #[test]
    fn cross() {
        let (v1, v2) = setup();
        let cross = RtVec3::new(-3.0, 6.0, -3.0);
        assert!(cross == v1.cross(&v2));
    }

    #[test]
    fn unit_vector() {
        let (v1, _) = setup();
        let known_uv = RtVec3::new(0.26726124, 0.5345225, 0.8017837);
        let uv: RtVec3 = v1.unit_vector();
        assert!(known_uv == uv);
    }
    
    #[test]
    fn neg() {
        let (v1, _) = setup();
        let known_neg = RtVec3::new(-1.0, -2.0, -3.0);
        let neg: RtVec3 = v1.neg();
        assert!(known_neg == neg);
    }
}