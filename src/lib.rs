mod signal;
use signal::Signal;
use signal::AperiodicSignal;

use std::f64::consts::PI;

pub fn impulse_decomposition<S: Signal>(signal: S) -> Vec<S> {
    let mut output = Vec::new();
    for i in 0..signal.len() {
        let mut new_values = vec![0.0; signal.len()];
        new_values[i] = signal[i];
        output.push(S::new(new_values));
    }
    output
}

pub fn step_decomposition<S: Signal>(signal: S) -> Vec<S> {
    let mut output = vec![S::new(vec![0.0; signal.len()])];
    for i in 1..signal.len() {
        let diff = signal[i] - signal[i-1];
        let mut new_values = vec![0.0; signal.len()];
        for item in new_values.iter_mut().take(signal.len()).skip(i) {
            *item = diff;
        }
        output.push(S::new(new_values));
    }
    output
}

pub fn even_odd_decomposition<S: Signal>(signal: S) -> Vec<S> {
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
    use signal::AperiodicSignal;

    #[test]
    fn impulse_single() {
        let sig = AperiodicSignal::new(vec![4.0]);
        assert_eq!(impulse_decomposition(sig), vec![AperiodicSignal::new(vec![4.0])]);
    }

    #[test]
    fn impulse_multiple() {
        let sig = AperiodicSignal::new(vec![4.0, 2.0, 5.0]);
        let result = vec![
            AperiodicSignal::new(vec![4.0, 0.0, 0.0]),
            AperiodicSignal::new(vec![0.0, 2.0, 0.0]),
            AperiodicSignal::new(vec![0.0, 0.0, 5.0])];
        assert_eq!(impulse_decomposition(sig), result);
    }

    #[test]
    fn step_single() {
        let sig = AperiodicSignal::new(vec![10.0]);
        assert_eq!(step_decomposition(sig), vec![AperiodicSignal::new(vec![0.0])]);
    }

    #[test]
    fn step_multiple() {
        let sig = AperiodicSignal::new(vec![4.0, 2.0, 5.0]);
        let result = vec![
            AperiodicSignal::new(vec![0.0, 0.0, 0.0]),
            AperiodicSignal::new(vec![0.0, -2.0, -2.0]),
            AperiodicSignal::new(vec![0.0, 0.0, 3.0])];
        assert_eq!(step_decomposition(sig), result);
    }

    #[test]
    fn even_uneven() {
        let sig = AperiodicSignal::new(vec![4.0, 1.0, -3.0, -4.0, 10.0, 5.0, 7.0]);
        let even = AperiodicSignal::new(vec![4.0, 4.0, 1.0, 3.0, 3.0, 1.0, 4.0]);
        let odd = AperiodicSignal::new(vec![0.0, -3.0, -4.0, -7.0, 7.0, 4.0, 3.0]);
        assert_eq!(even_odd_decomposition(sig), vec![even, odd]);
    }

}
