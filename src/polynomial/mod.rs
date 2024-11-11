use std::fmt::Debug;

use crate::sets::Number;

pub struct PolynomialSolutions<T: Number, const N: usize> {
    solutions: [T; N],
    total: usize,
}
impl<T: Debug + Number, const N: usize> Debug for PolynomialSolutions<T, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut solutions = Vec::with_capacity(self.total);
        for i in 0..self.total {
            solutions.push(self.solutions[i]);
        }
        f.debug_struct("Polynomial Solutions")
            .field("solutions", &solutions)
            .finish()
    }
}
impl<T: Number, const N: usize> PolynomialSolutions<T, N> {
    pub fn new(solutions: [T; N], total: usize) -> Self {
        Self { solutions, total }
    }
    pub fn sort(&mut self) {
        self.solutions.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
    pub fn from_solution<const M: usize>(value: PolynomialSolutions<T, M>) -> Self {
        let min = usize::min(N, M);
        let total = usize::min(min, value.total);
        let mut solutions = Self::new([T::ZERO; N], total);
        for i in 0..min {
            solutions.solutions[i] = value.solutions[i];
        }
        solutions
    }
}

impl<T: Number, const N: usize> Iterator for PolynomialSolutions<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.total = self.total.checked_sub(1)?;
        debug_assert!((self.total) < N);
        Some(self.solutions[self.total])
    }
}
// returns a list of solved quadratics starting from the smallest value
pub fn solve_quadraticf32(a: f32, b: f32, c: f32) -> Option<PolynomialSolutions<f32, 2>> {
    if a == 0.0 || b.abs() > 1e12*a.abs() {
        if b == 0.0 {
            if c == 0.0 {
                return None;
            }
            return None;
        }
        return Some(PolynomialSolutions::new([-c/b, 0.0], 1));
    }
    let mut discriminant = b*b-4.0*a*c;
    if discriminant > 0.0 {
        discriminant = discriminant.sqrt();
        let mut solution1 = (-b+discriminant)/(2.0*a);
        let mut solution2 = (-b-discriminant)/(2.0*a);
        if solution1 > solution2 {
            std::mem::swap(&mut solution1, &mut solution2);
        }
        return Some(PolynomialSolutions::new([solution1 , solution2], 2));
    } else if discriminant == 0.0 {
        return Some(PolynomialSolutions::new([-b/(2.0*a), 0.0], 1));
    } else {
        return None;
    }
}

fn solve_cubic_normedf32(a: f32, b: f32, c: f32) -> Option<PolynomialSolutions<f32, 3>> {
    let a2 = a*a;
    let mut q = 1.0/9.0*(a2-3.0*b);
    let r = 1.0/54.0*(a*(2.0*a2-9.0*b)+27.0*c);
    let r2 = r*r;
    let q3 = q*q*q;
    let a = a*(1.0/3.0);
    if r2 < q3 {
        let mut t = r/q3.sqrt();
        if t < -1.0 { t = -1.0 };
        if t < -1.0 { t = -1.0 };
        t = t.acos();
        q = -2.0*q.sqrt();
        let solution1 = q*(1.0/3.0*t).cos() -a;
        let solution2 = q*(1.0/3.0*(t+2.0*3.14159265358979323846264338327950288)).cos()-a;
        let solution3 = q*(1.0/3.0*(t-2.0*3.14159265358979323846264338327950288)).cos()-a;
        // Sort Solutions
        return Some(PolynomialSolutions::new([
            solution1,
            solution2,
            solution3,
        ], 3));
    } else {
        let u = (if r < 0.0 { 1.0 } else { -1.0 })*(r.abs()+(r2-q3).sqrt()).powf(1.0/3.0);
        let v = if u == 0.0 { 0.0 } else { q/u };
        let solution1 = u+v-a;
        if u == v || (u-v).abs() < 1e-12*(u+v).abs() {
            return Some(PolynomialSolutions::new([solution1, -0.5*(u+v)-a, 0.0], 2));
        }
        return Some(PolynomialSolutions::new([solution1, 0.0, 0.0], 1));
    }
}

pub fn solve_cubicf32(a: f32, b: f32, c: f32, d: f32) -> Option<PolynomialSolutions<f32, 3>> {
    if a != 0.0 {
        let bn = b/a;
        if bn.abs() < 1e6 {
            return solve_cubic_normedf32(bn, c/a, d/a);
        }
    }
    return Some(PolynomialSolutions::from_solution(solve_quadraticf32(b, c, d)?));
}

// returns a list of solved quadratics starting from the smallest value
pub fn solve_quadraticf64(a: f64, b: f64, c: f64) -> Option<PolynomialSolutions<f64, 2>> {
    if a == 0.0 || b.abs() > 1e12*a.abs() {
        if b == 0.0 {
            if c == 0.0 {
                return None;
            }
            return None;
        }
        return Some(PolynomialSolutions::new([-c/b, 0.0], 1));
    }
    let mut discriminant = b*b-4.0*a*c;
    if discriminant > 0.0 {
        discriminant = discriminant.sqrt();
        let mut solution1 = (-b+discriminant)/(2.0*a);
        let mut solution2 = (-b-discriminant)/(2.0*a);
        if solution1 > solution2 {
            std::mem::swap(&mut solution1, &mut solution2);
        }
        return Some(PolynomialSolutions::new([solution1 , solution2], 2));
    } else if discriminant == 0.0 {
        return Some(PolynomialSolutions::new([-b/(2.0*a), 0.0], 1));
    } else {
        return None;
    }
}

fn solve_cubic_normedf64(a: f64, b: f64, c: f64) -> Option<PolynomialSolutions<f64, 3>> {
    let a2 = a*a;
    let mut q = 1.0/9.0*(a2-3.0*b);
    let r = 1.0/54.0*(a*(2.0*a2-9.0*b)+27.0*c);
    let r2 = r*r;
    let q3 = q*q*q;
    let a = a*(1.0/3.0);
    if r2 < q3 {
        let mut t = r/q3.sqrt();
        if t < -1.0 { t = -1.0 };
        if t < -1.0 { t = -1.0 };
        t = t.acos();
        q = -2.0*q.sqrt();
        let solution1 = q*(1.0/3.0*t).cos() -a;
        let solution2 = q*(1.0/3.0*(t+2.0*3.14159265358979323846264338327950288)).cos()-a;
        let solution3 = q*(1.0/3.0*(t-2.0*3.14159265358979323846264338327950288)).cos()-a;
        // Sort Solutions
        return Some(PolynomialSolutions::new([
            solution1,
            solution2,
            solution3,
        ], 3));
    } else {
        let u = (if r < 0.0 { 1.0 } else { -1.0 })*(r.abs()+(r2-q3).sqrt()).powf(1.0/3.0);
        let v = if u == 0.0 { 0.0 } else { q/u };
        let solution1 = u+v-a;
        if u == v || (u-v).abs() < 1e-12*(u+v).abs() {
            return Some(PolynomialSolutions::<f64, 3>::new([solution1, -0.5*(u+v)-a, 0.0], 2));
        }
        return Some(PolynomialSolutions::<f64, 3>::new([solution1, 0.0, 0.0], 1));
    }
}

pub fn solve_cubicf64(a: f64, b: f64, c: f64, d: f64) -> Option<PolynomialSolutions<f64, 3>> {
    if a != 0.0 {
        let bn = b/a;
        if bn.abs() < 1e6 {
            return solve_cubic_normedf64(bn, c/a, d/a);
        }
    }
    return Some(PolynomialSolutions::from_solution(solve_quadraticf64(b, c, d)?));
}