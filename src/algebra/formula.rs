use std::fmt::Debug;

use crate::{Number, Real};

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
pub fn solve_quadratic<T: Real>(a: T, b: T, c: T) -> Option<PolynomialSolutions<T, 2>> {
    if a == T::ZERO || b.abs() > T::from_f64(1e12)*a.abs() {
        if b == T::ZERO {
            if c == T::ZERO {
                return None;
            }
            return None;
        }
        return Some(PolynomialSolutions::new([-c/b, T::ZERO], 1));
    }
    let mut discriminant = b*b-T::from_f64(4.0)*a*c;
    if discriminant > T::ZERO {
        discriminant = discriminant.sqrt();
        let mut solution1 = (-b+discriminant)/(T::from_f64(2.0)*a);
        let mut solution2 = (-b-discriminant)/(T::from_f64(2.0)*a);
        if solution1 > solution2 {
            std::mem::swap(&mut solution1, &mut solution2);
        }
        return Some(PolynomialSolutions::new([solution1 , solution2], 2));
    } else if discriminant == T::ZERO {
        return Some(PolynomialSolutions::new([-b/(T::from_f64(2.0)*a), T::ZERO], 1));
    } else {
        return None;
    }
}

fn solve_cubic_normed<T: Real>(a: T, b: T, c: T) -> Option<PolynomialSolutions<T, 3>> {
    let a2 = a*a;
    let mut q = T::ONE/T::from_f64(9.0)*(a2-T::from_f64(3.0)*b);
    let r = T::ONE/T::from_f64(54.0)*(a*(T::from_f64(2.0)*a2-T::from_f64(9.0)*b)+T::from_f64(27.0)*c);
    let r2 = r*r;
    let q3 = q*q*q;
    let a = a*(T::ONE/T::from_f64(3.0));
    if r2 < q3 {
        let mut t = r/q3.sqrt();
        if t < -T::ONE { t = -T::ONE };
        if t < -T::ONE { t = -T::ONE };
        t = t.acos();
        q = T::from_f64(-2.0)*q.sqrt();
        let solution1 = q*(T::ONE/T::from_f64(3.0)*t).cos() -a;
        let solution2 = q*(T::ONE/T::from_f64(3.0)*(t+T::from_f64(2.0)*T::PI)).cos()-a;
        let solution3 = q*(T::ONE/T::from_f64(3.0)*(t-T::from_f64(2.0)*T::PI)).cos()-a;
        // Sort Solutions
        return Some(PolynomialSolutions::new([
            solution1,
            solution2,
            solution3,
        ], 3));
    } else {
        let u = (if r < T::ZERO { T::ONE } else { -T::ONE })*(r.abs()+(r2-q3).sqrt()).powf(T::ONE/T::from_f64(3.0));
        let v = if u == T::ZERO { T::ZERO } else { q/u };
        let solution1 = u+v-a;
        if u == v || (u-v).abs() < T::from_f64(1e-12)*(u+v).abs() {
            return Some(PolynomialSolutions::new([solution1, T::from_f64(-0.5)*(u+v)-a, T::ZERO], 2));
        }
        return Some(PolynomialSolutions::new([solution1, T::ZERO, T::ZERO], 1));
    }
}

pub fn solve_cubic<T: Real>(a: T, b: T, c: T, d: T) -> Option<PolynomialSolutions<T, 3>> {
    if a != T::ZERO {
        let bn = b/a;
        if bn.abs() < T::from_f64(1e6) {
            return solve_cubic_normed(bn, c/a, d/a);
        }
    }
    return Some(PolynomialSolutions::from_solution(solve_quadratic(b, c, d)?));
}