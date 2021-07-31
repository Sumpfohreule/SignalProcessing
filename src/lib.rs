use std::ops::Index;

#[derive(Debug, PartialEq)]
struct AperiodicSignal {
    values: Vec<i32>,
}

impl AperiodicSignal {
    fn new(values: Vec<i32>) -> Self {
        Self { values }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn extend(self, size: usize) -> Self {
        let mut extended = Vec::new();
        for i in 0..self.len() {
            extended.push(self[i]);
        }
        for _ in self.len()..size {
            extended.push(0);
        }
        AperiodicSignal::new(extended)
    }
}

impl Index<usize> for AperiodicSignal {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

impl std::ops::Add<AperiodicSignal> for AperiodicSignal {
    type Output = AperiodicSignal;
    fn add(self, rhs: AperiodicSignal) -> Self::Output {
        let sig_1;
        let sig_2;
        if self.len() < rhs.len() {
            sig_1 = self.extend(rhs.len());
            sig_2 = rhs;
        } else if self.len() > rhs.len() {
            sig_2 = rhs.extend(self.len());
            sig_1 = self;
        } else {
            sig_1 = self;
            sig_2 = rhs;
        }

        let mut new_signal = Vec::new();
        for i in 0..sig_1.len() {
            new_signal.push(sig_1[i] + sig_2[i]);
        }
        AperiodicSignal::new(new_signal)
    }
}

fn impulse_decomposition(signal: AperiodicSignal) -> Vec<AperiodicSignal> {
    let mut output = Vec::new();
    for i in 0..signal.len() {
        let mut new_values = vec![0; signal.len()];
        new_values[i] = signal[i];
        output.push(AperiodicSignal::new(new_values));
    }
    output
}

fn step_decomposition(signal: AperiodicSignal) -> Vec<AperiodicSignal> {
    let mut output = Vec::new();
    output.push(AperiodicSignal::new(vec![0; signal.len()]));
    for i in 1..signal.len() {
        let diff = signal[i] - signal[i-1];
        let mut new_values = vec![0; signal.len()];
        for j in i..signal.len() {
            new_values[j] = diff;
        }
        output.push(AperiodicSignal::new(new_values));
    }
    output
}

fn even_odd_decomposition(signal: AperiodicSignal) -> Vec<AperiodicSignal> {
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
    vec![AperiodicSignal::new(even), AperiodicSignal::new(odd)]
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
}
