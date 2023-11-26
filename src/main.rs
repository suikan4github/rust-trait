// Study of the trait of the Rust language.

// In this program, we define a common complex trait "Complex".
// And implement it for both cartesian and polar representation of
// the complex number.
// We can see one trait can support both representation.

fn main() -> Result<(), ()> {
    // Create a trait to be an interface of a complex number.
    // The implementation of the complex ( cartesian or polar )
    // is not affected to here.
    trait Complex {
        fn re(&self) -> f32;
        fn im(&self) -> f32;
        fn radius(&self) -> f32;
        fn angle(&self) -> f32;
        fn set_cartesian(&mut self, r: f32, i: f32);
        fn set_polar(&mut self, r: f32, a: f32);
    }

    // an example cartesian representation of complex number
    struct CartesianComplex {
        re: f32,
        im: f32,
    }

    // an example polar representation of complex number.
    struct PolarComplex {
        radius: f32,
        angle: f32,
    }

    // Let's implement the Complex trait for the CartesianComplex type.
    impl Complex for CartesianComplex {
        // We don't need to convert.
        fn re(&self) -> f32 {
            self.re
        }

        // We don't need to convert.
        fn im(&self) -> f32 {
            self.im
        }

        // We need to convert from cartesian to polar.
        fn radius(&self) -> f32 {
            (self.re.powi(2) + self.im.powi(2)).sqrt()
        }

        fn angle(&self) -> f32 {
            self.im.atan2(self.re)
        }

        // We don't need to convert.
        fn set_cartesian(&mut self, r: f32, i: f32) {
            self.re = r;
            self.im = i;
        }

        // We need to convert from polar to cartesian .
        fn set_polar(&mut self, r: f32, a: f32) {
            self.re = r * a.cos();
            self.im = r * a.sin();
        }
    }

    // Let's implement the Complex trait for the PolarComplex type.
    impl Complex for PolarComplex {
        // We need to convert from polar to cartesian.
        fn re(&self) -> f32 {
            self.radius * self.angle.cos()
        }

        // We need to convert from polar to cartesian.
        fn im(&self) -> f32 {
            self.radius * self.angle.sin()
        }

        // We don't need to convert.
        fn radius(&self) -> f32 {
            self.radius
        }

        // We don't need to convert.
        fn angle(&self) -> f32 {
            self.angle
        }

        // We need to convert from polar to cartesian.
        fn set_cartesian(&mut self, r: f32, i: f32) {
            self.radius = (r.powi(2) + i.powi(2)).sqrt();
            self.angle = i.atan2(r);
        }

        // We don't need to convert.
        fn set_polar(&mut self, r: f32, a: f32) {
            self.radius = r;
            self.angle = a;
        }
    }

    // Now, let's see on the terminal.

    // Variable under test.
    let p = PolarComplex {
        radius: 3.0,
        angle: 1.5707964,
    };

    println!("");
    println!("p's Radius    : {}", p.radius());
    println!("p's Angular   : {}", p.angle());
    println!("p's Real      : {}", p.re());
    println!("p's Imaginary : {}", p.im());

    // Variable under test.
    let q = CartesianComplex { re: 0.0, im: 3.0 };

    println!("");
    println!("q's Radius    : {}", q.radius());
    println!("q's Angular   : {}", q.angle());
    println!("q's Real      : {}", q.re());
    println!("q's Imaginary : {}", q.im());

    Ok(())
}
