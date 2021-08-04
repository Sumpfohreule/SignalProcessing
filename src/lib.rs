use std::ops::Index;

trait Signal: Index<i32, Output=i32> + Index<usize, Output=i32> + Sized + Clone {
    fn new(values: Vec<i32>) -> Self;
    fn len(&self) -> usize;
    fn fold(&self, rhs: &Self) -> Self {
        let n = self.len();
        let m = rhs.len();
        let mut output = Vec::new();
        for i in 0..(n + m - 1) as i32 {
            let mut sum = 0;
            for j in 0..(m) as i32 {
                sum += rhs[j] * self[i - j];
            }
            output.push(sum);
        }
        Self::new(output)
    }
}

#[derive(Debug, PartialEq, Clone)]
struct AperiodicSignal {
    values: Vec<i32>,
}

impl Signal for AperiodicSignal {
    fn new(values: Vec<i32>) -> Self {
        Self { values }
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

impl Index<usize> for AperiodicSignal {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= self.len() {
            &0
        } else {
            &self.values[index]
        }
    }
}

impl Index<i32> for AperiodicSignal {
    type Output = i32;

    fn index(&self, index: i32) -> &Self::Output {
        if index < 0 || index >= self.len() as i32 {
            &0
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

fn impulse_decomposition<S: Signal>(signal: S) -> Vec<S> {
    let mut output = Vec::new();
    for i in 0..signal.len() {
        let mut new_values = vec![0; signal.len()];
        new_values[i] = signal[i];
        output.push(S::new(new_values));
    }
    output
}

fn step_decomposition<S: Signal>(signal: S) -> Vec<S> {
    let mut output = Vec::new();
    output.push(S::new(vec![0; signal.len()]));
    for i in 1..signal.len() {
        let diff = signal[i] - signal[i-1];
        let mut new_values = vec![0; signal.len()];
        for j in i..signal.len() {
            new_values[j] = diff;
        }
        output.push(S::new(new_values));
    }
    output
}

fn even_odd_decomposition<S: Signal>(signal: S) -> Vec<S> {
    let mut even = Vec::new();
    even.reserve_exact(signal.len() + 1);
    even.push(signal[0]);
    
    let mut odd = Vec::new();
    odd.reserve_exact(signal.len() + 1);
    odd.push(0);

    for i in 1..signal.len() {
        let front_index = i % signal.len();
        let back_index = (signal.len() - i) % signal.len();
        even.push((signal[front_index] + signal[back_index]) / 2);
        odd.push((signal[front_index] - signal[back_index]) / 2);
    }
    vec![S::new(even), S::new(odd)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_same_length_signals() {
        let sig_1 = AperiodicSignal::new(vec![1, 4, 8, 3]);
        let sig_2 = AperiodicSignal::new(vec![2, 3, 8, -1]);
        let output = AperiodicSignal::new(vec![3, 7, 16, 2]);
        assert_eq!(sig_1 + sig_2, output);
    }

    #[test]
    fn add_signals_rhs_shorter() {
        let sig_1 = AperiodicSignal::new(vec![1, 4, 8, 3]);
        let sig_2 = AperiodicSignal::new(vec![2, 3]);
        let output = AperiodicSignal::new(vec![3, 7, 8, 3]);
        assert_eq!(sig_1 + sig_2, output);
    }

    #[test]
    fn impulse_single() {
        let sig = AperiodicSignal::new(vec![4]);
        assert_eq!(impulse_decomposition(sig), vec![AperiodicSignal::new(vec![4])]);
    }

    #[test]
    fn impulse_multiple() {
        let sig = AperiodicSignal::new(vec![4, 2, 5]);
        let result = vec![
            AperiodicSignal::new(vec![4, 0, 0]),
            AperiodicSignal::new(vec![0, 2, 0]),
            AperiodicSignal::new(vec![0, 0, 5])];
        assert_eq!(impulse_decomposition(sig), result);
    }

    #[test]
    fn step_single() {
        let sig = AperiodicSignal::new(vec![10]);
        assert_eq!(step_decomposition(sig), vec![AperiodicSignal::new(vec![0])]);
    }

    #[test]
    fn step_multiple() {
        let sig = AperiodicSignal::new(vec![4, 2, 5]);
        let result = vec![
            AperiodicSignal::new(vec![0, 0, 0]),
            AperiodicSignal::new(vec![0, -2, -2]),
            AperiodicSignal::new(vec![0, 0, 3])];
        assert_eq!(step_decomposition(sig), result);
    }

    #[test]
    fn even_uneven() {
        let sig = AperiodicSignal::new(vec![4, 1, -3, -4, 10, 5, 7]);
        let even = AperiodicSignal::new(vec![4, 4, 1, 3, 3, 1, 4]);
        let odd = AperiodicSignal::new(vec![0, -3, -4, -7, 7, 4, 3]);
        assert_eq!(even_odd_decomposition(sig), vec![even, odd]);
    }

    #[test]
    fn fold_identity() {
        let signal = AperiodicSignal::new(vec![1, 2, 3, 4, 5]);
        let kernel = AperiodicSignal::new(vec![1]);
        assert_eq!(signal.fold(&kernel), signal);
    }

    #[test]
    fn fold_delay() {
        let signal = AperiodicSignal::new(vec![1, 2, 3, 4, 5]);
        let kernel = AperiodicSignal::new(vec![0, 0, 1]);
        assert_eq!(signal.fold(&kernel), AperiodicSignal::new(vec![0, 0, 1, 2, 3, 4, 5]));
    }

    #[test]
    fn fold_amplify() {
        let signal = AperiodicSignal::new(vec![1, 2, -3, 4, 5]);
        let kernel = AperiodicSignal::new(vec![2]);
        assert_eq!(signal.fold(&kernel), AperiodicSignal::new(vec![2, 4, -6, 8, 10]));
    }
}
