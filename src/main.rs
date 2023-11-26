fn main() -> Result<(), ()> {
    trait Complex {
        fn re(&self) -> f32;
        fn im(&self) -> f32;
        fn radius(&self) -> f32;
        fn angle(&self) -> f32;
        fn set_cartesian(&mut self, r: f32, i: f32);
        fn set_polar(&mut self, r: f32, a: f32);
    }

    struct CartesianComplex {
        re: f32,
        im: f32,
    }

    struct PolarComplex {
        radius: f32,
        angle: f32,
    }

    impl Complex for CartesianComplex {
        fn re(&self) -> f32 {
            self.re
        }

        fn im(&self) -> f32 {
            self.im
        }

        fn radius(&self) -> f32 {
            (self.re.powi(2) + self.im.powi(2)).sqrt()
        }

        fn angle(&self) -> f32 {
            self.im.atan2(self.re)
        }

        fn set_cartesian(&mut self, r: f32, i: f32) {
            self.re = r;
            self.im = i;
        }

        fn set_polar(&mut self, r: f32, a: f32) {
            self.re = r * a.cos();
            self.im = r * a.sin();
        }
    }

    impl Complex for PolarComplex {
        fn re(&self) -> f32 {
            self.radius * self.angle.cos()
        }

        fn im(&self) -> f32 {
            self.radius * self.angle.sin()
        }

        fn radius(&self) -> f32 {
            self.radius
        }

        fn angle(&self) -> f32 {
            self.angle
        }

        fn set_cartesian(&mut self, r: f32, i: f32) {
            self.radius = (r.powi(2) + i.powi(2)).sqrt();
            self.angle = i.atan2(r);
        }

        fn set_polar(&mut self, r: f32, a: f32) {
            self.radius = r;
            self.angle = a;
        }
    }

    let p = PolarComplex {
        radius: 3.0,
        angle: 1.5707964,
    };

    println!("");
    println!("p's Radius    : {}", p.radius());
    println!("p's Angular   : {}", p.angle());
    println!("p's Real      : {}", p.re());
    println!("p's Imaginary : {}", p.im());

    let q = CartesianComplex { re: 0.0, im: 3.0 };

    println!("");
    println!("q's Radius    : {}", q.radius());
    println!("q's Angular   : {}", q.angle());
    println!("q's Real      : {}", q.re());
    println!("q's Imaginary : {}", q.im());

    Ok(())
}
