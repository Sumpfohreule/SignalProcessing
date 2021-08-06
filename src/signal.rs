use std::ops::Index;

pub trait Signal: Index<i32, Output=f64> + Index<usize, Output=f64> + Sized + Clone {
    fn new(values: Vec<f64>) -> Self;
    fn len(&self) -> usize;
    fn fold(&self, rhs: &Self) -> Self {
        let n = self.len();
        let m = rhs.len();
        let mut output = Vec::new();
        for i in 0..(n + m - 1) as i32 {
            let mut sum = 0.0;
            for j in 0..(m) as i32 {
                sum += rhs[j] * self[i - j];
            }
            output.push(sum);
        }
        Self::new(output)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AperiodicSignal {
    values: Vec<f64>,
}

impl Signal for AperiodicSignal {
    fn new(values: Vec<f64>) -> Self {
        Self { values }
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

impl Index<usize> for AperiodicSignal {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            &0.0
        } else {
            &self.values[index]
        }
    }
}

impl Index<i32> for AperiodicSignal {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        if index < 0 || index >= self.len() as i32 {
            &0.0
        } else {
            &self.values[index as usize]
        }
    }
}

impl std::ops::Add<AperiodicSignal> for AperiodicSignal {
    type Output = AperiodicSignal;
    fn add(self, rhs: AperiodicSignal) -> Self::Output {
        let mut new_signal = Vec::new();
        for i in 0..self.len() {
            new_signal.push(self[i] + rhs[i]);
        }
        AperiodicSignal::new(new_signal)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_same_length_signals() {
        let sig_1 = AperiodicSignal::new(vec![1.0, 4.0, 8.0, 3.0]);
        let sig_2 = AperiodicSignal::new(vec![2.0, 3.0, 8.0, -1.0]);
        let output = AperiodicSignal::new(vec![3.0, 7.0, 16.0, 2.0]);
        assert_eq!(sig_1 + sig_2, output);
    }

    #[test]
    fn add_signals_rhs_shorter() {
        let sig_1 = AperiodicSignal::new(vec![1.0, 4.0, 8.0, 3.0]);
        let sig_2 = AperiodicSignal::new(vec![2.0, 3.0]);
        let output = AperiodicSignal::new(vec![3.0, 7.0, 8.0, 3.0]);
        assert_eq!(sig_1 + sig_2, output);
    }

    #[test]
    fn fold_identity() {
        let signal = AperiodicSignal::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let kernel = AperiodicSignal::new(vec![1.0]);
        assert_eq!(signal.fold(&kernel), signal);
    }

    #[test]
    fn fold_delay() {
        let signal = AperiodicSignal::new(vec![1.0, 2.0, 3.0, 4.0, 5.0]);
        let kernel = AperiodicSignal::new(vec![0.0, 0.0, 1.0]);
        assert_eq!(signal.fold(&kernel), AperiodicSignal::new(vec![0.0, 0.0, 1.0, 2.0, 3.0, 4.0, 5.0]));
    }

    #[test]
    fn fold_amplify() {
        let signal = AperiodicSignal::new(vec![1.0, 2.0, -3.0, 4.0, 5.0]);
        let kernel = AperiodicSignal::new(vec![2.0]);
        assert_eq!(signal.fold(&kernel), AperiodicSignal::new(vec![2.0, 4.0, -6.0, 8.0, 10.0]));
    }
}
