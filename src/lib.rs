use std::ops::Index;

#[derive(Debug, PartialEq)]
struct LinearSignal {
    values: Vec<i32>,
}

impl LinearSignal {
    fn new(values: Vec<i32>) -> Self {
        Self { values }
    }

    fn len(&self) -> usize {
        self.values.len()
    }
}

impl Index<usize> for LinearSignal {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.values[index]
    }
}

fn impulse_decomposition(signal: LinearSignal) -> Vec<LinearSignal> {
    let mut output = Vec::new();
    for i in 0..signal.len() {
        let mut new_values = vec![0; signal.len()];
        new_values[i] = signal[i];
        output.push(LinearSignal::new(new_values));
    }
    output
}

fn step_decomposition(signal: LinearSignal) -> Vec<LinearSignal> {
    let mut output = Vec::new();
    output.push(LinearSignal::new(vec![0; signal.len()]));
    for i in 1..signal.len() {
        let diff = signal[i] - signal[i-1];
        let mut new_values = vec![0; signal.len()];
        for j in i..signal.len() {
            new_values[j] = diff;
        }
        output.push(LinearSignal::new(new_values));
    }
    output
}

fn even_odd_decomposition(signal: LinearSignal) -> Vec<LinearSignal> {
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
    vec![LinearSignal::new(even), LinearSignal::new(odd)]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn impulse_single() {
        let sig = LinearSignal::new(vec![4]);
        assert_eq!(impulse_decomposition(sig), vec![LinearSignal::new(vec![4])]);
    }

    #[test]
    fn impulse_multiple() {
        let sig = LinearSignal::new(vec![4, 2, 5]);
        let result = vec![
            LinearSignal::new(vec![4, 0, 0]),
            LinearSignal::new(vec![0, 2, 0]),
            LinearSignal::new(vec![0, 0, 5])];
        assert_eq!(impulse_decomposition(sig), result);
    }

    #[test]
    fn step_single() {
        let sig = LinearSignal::new(vec![10]);
        assert_eq!(step_decomposition(sig), vec![LinearSignal::new(vec![0])]);
    }

    #[test]
    fn step_multiple() {
        let sig = LinearSignal::new(vec![4, 2, 5]);
        let result = vec![
            LinearSignal::new(vec![0, 0, 0]),
            LinearSignal::new(vec![0, -2, -2]),
            LinearSignal::new(vec![0, 0, 3])];
        assert_eq!(step_decomposition(sig), result);
    }

    #[test]
    fn even_uneven() {
        let sig = LinearSignal::new(vec![4, 1, -3, -4, 10, 5, 7]);
        let even = LinearSignal::new(vec![4, 4, 1, 3, 3, 1, 4]);
        let odd = LinearSignal::new(vec![0, -3, -4, -7, 7, 4, 3]);
        assert_eq!(even_odd_decomposition(sig), vec![even, odd]);
    }
}
