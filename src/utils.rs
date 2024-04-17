pub fn softmax(x: &[i32]) -> Vec<f64> {
    let sum_exp: f64 = x.iter().map(|i| (*i as f64).exp()).sum();
    let x_trans = x.iter().map(|i| (*i as f64).exp() / sum_exp).collect();
    x_trans
}
