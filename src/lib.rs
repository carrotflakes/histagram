#[derive(Clone)]
pub struct Result {
    min: f64,
    max: f64,
    lower: f64,
    upper: f64,
    buckets: Vec<usize>,
}

impl Result {
    pub fn shrink(&self) -> Result {
        if self.buckets.is_empty() {
            return self.clone();
        }
        let i = self.buckets.iter().position(|&x| x != 0).unwrap(); // TODO
        let j = self.buckets.len() - self.buckets.iter().rev().position(|&x| x != 0).unwrap(); // TODO
        Result {
            min: self.min,
            max: self.max,
            lower: self.lower + (self.upper - self.lower) * (i as f64 / self.buckets.len() as f64),
            upper: self.lower + (self.upper - self.lower) * (j as f64 / self.buckets.len() as f64),
            buckets: self.buckets.iter().take(j).skip(i).copied().collect(),
        }
    }
}

pub fn make_histogram(mut iter: impl Iterator<Item = f64>) -> Result {
    let desire_size = 60;
    if let Some(first) = iter.next() {
        let mut first_count = 1;
        let second = {
            loop {
                if let Some(x) = iter.next() {
                    if x != first {
                        break x;
                    }
                } else {
                    return Result {
                        min: first,
                        max: first,
                        lower: first,
                        upper: first,
                        buckets: vec![first_count],
                    };
                }
                first_count += 1;
            }
        };

        let mut min = first.min(second);
        let mut max = first.max(second);
        let mut lower = min;
        let mut upper = max;
        let mut buckets = vec![0; desire_size];
        buckets[0] = if lower == first { first_count } else { 1 };
        buckets[desire_size - 1] = if upper == first { first_count } else { 1 };

        for x in iter {
            min = min.min(x);
            max = max.max(x);
            while x < lower {
                // extend lower bound
                for i in 0..desire_size / 2 {
                    buckets[desire_size - i - 1] =
                        buckets[desire_size - i * 2 - 1] + buckets[desire_size - i * 2 - 2];
                }
                buckets[0..desire_size / 2].fill(0);
                lower -= upper - lower;
            }
            while upper <= x {
                // extend upper bound
                for i in 0..desire_size / 2 {
                    buckets[i] = buckets[i * 2] + buckets[i * 2 + 1];
                }
                buckets[desire_size / 2..].fill(0);
                upper += upper - lower;
            }
            buckets[((x - lower) * desire_size as f64 / (upper - lower)).floor() as usize] += 1;
        }
        Result {
            min,
            max,
            lower,
            upper,
            buckets,
        }
    } else {
        Result {
            min: f64::NAN,
            max: f64::NAN,
            lower: f64::NAN,
            upper: f64::NAN,
            buckets: vec![],
        }
    }
}

pub fn histagram(iter: impl Iterator<Item = f64>) {
    let Result {
        lower,
        upper,
        buckets,
        ..
    } = make_histogram(iter).shrink();

    if buckets.is_empty() {
        println!("No data");
    } else {
        let scale = 10;
        let max = buckets.iter().copied().max().unwrap();
        for i in 0..scale {
            print!("{:>6}|", max * (scale - i - 1) / (scale - 1));
            for x in buckets.iter() {
                print!(
                    "{}",
                    if scale * x / max > scale - i - 1 {
                        "*"
                    } else {
                        " "
                    }
                );
            }
            println!("|");
        }
        let lower_str = format!("{:.6}", lower);
        let upper_str = if lower != upper {
            format!("{:.6}", upper)
        } else {
            "".to_string()
        };
        println!(
            "{} {}{}{}",
            " ".repeat(6),
            lower_str,
            " ".repeat(
                buckets
                    .len()
                    .saturating_sub(lower_str.len() + upper_str.len())
            ),
            upper_str
        );
    }
}
