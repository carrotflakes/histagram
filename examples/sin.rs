fn main() {
    let iter = (0..100000).map(|i| (i as f64 / 1000.0).sin());
    histagram::histagram(iter);
}
