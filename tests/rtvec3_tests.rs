/*
pub fn example_rtvec3() {
    let v1 = RtVec3::new(1.0, 2.0, 3.0);
    let v2 = RtVec3::new(4.0, 5.0, 6.0);

    let v_add = v1 + v2; // Using operator overloading
    let v_sub = v1 - v2; // Using operator overloading
    let v_mul = v1 * v2; // Using operator overloading
    let v_div = v1 / v2; // Using operator overloading
    let v_mul_s = v1.multiply_scalar(2.0); // Using operator overloading
    let v_div_s = v1.divide_scalar(2.0); // Using operator overloading
    println!("+ : {:?}", v_add);
    println!("- : {:?}", v_sub);
    println!("* : {:?}", v_mul);
    println!("/ : {:?}", v_div);
    println!("*s : {:?}", v_mul_s);
    println!("/s : {:?}", v_div_s);

    let (x, y, z) = (v1.x(), v1.y(), v1.z());
    let example = RtVec3 {
        x,
        y,
        z,
    };
    println!("{:?}", example);
    
    let v3 = v1.add(v2);
    let length = v3.length();
    println!("Length of v3: {}", length);
}
*/

#[cfg(test)]
mod tests {
    use std::ops::{Add, Sub};
    use raytracing_in_a_weekend::RtVec3;
    // let v1 = RtVec3::new(1.0, 2.0, 3.0);
    // let v2 = RtVec3::new(4.0, 5.0, 6.0);

    #[test]
    fn example_rtvec3() {
        let v1 = RtVec3::new(1.0, 2.0, 3.0);
        let v2 = RtVec3::new(4.0, 5.0, 6.0);

        let self_ref_check = RtVec3::new(v1.x(), v1.y(), v1.z());
        assert!(self_ref_check == v1);

        let v_add = RtVec3::new(5.0, 7.0, 9.0);
        assert!(v_add == v1 + v2);
        assert!(v_add == v1.add(v2));

        let v_sub = RtVec3::new(-3.0, -3.0, -3.0);
        assert!(v_sub == v1 - v2);
        assert!(v_sub == v1.sub(v2));

        let v_mul = RtVec3::new(4.0, 10.0, 18.0);
        assert!(v_mul == v1 * v2);
        
        let v_div = RtVec3::new(0.25, 0.4, 0.5);
        assert!(v_div == v1 / v2);
        
        let v_mul_s = RtVec3::new(2.0, 4.0, 6.0);
        assert!(v_mul_s == v1.multiply_scalar(2.0));

        let v_div_s = RtVec3::new(0.5, 1.0, 1.5);
        assert!(v_div_s == v1.divide_scalar(2.0));

        let length_known: f32 = 12.4499;
        let v3 = v1.add(v2);
        let length = v3.length();
        assert!(length == length_known)
    }
}