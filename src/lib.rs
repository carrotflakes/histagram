pub fn make_histogram(mut iter: impl Iterator<Item = f64>) -> (f64, f64, Vec<usize>) {
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
                    return (first, first, vec![first_count]);
                }
                first_count += 1;
            }
        };

        let mut min = first.min(second);
        let mut max = first.max(second);
        let mut buckets = vec![0; desire_size];
        buckets[0] = if min == first { first_count } else { 1 };
        buckets[desire_size - 1] = if max == first { first_count } else { 1 };

        for x in iter {
            while x < min {
                // extend lower bound
                for i in 0..desire_size / 2 {
                    buckets[desire_size - i - 1] =
                        buckets[desire_size - i * 2 - 1] + buckets[desire_size - i * 2 - 2];
                }
                buckets[0..desire_size / 2].fill(0);
                min -= max - min;
            }
            while max <= x {
                // extend upper bound
                for i in 0..desire_size / 2 {
                    buckets[i] = buckets[i * 2] + buckets[i * 2 + 1];
                }
                buckets[desire_size / 2..].fill(0);
                max += max - min;
            }
            buckets[((x - min) * desire_size as f64 / (max - min)).floor() as usize] += 1;
        }
        (min, max, buckets)
    } else {
        (f64::NAN, f64::NAN, vec![])
    }
}

pub fn histagram(iter: impl Iterator<Item = f64>) {
    let (lower, uppper, data) = make_histogram(iter);
    if data.is_empty() {
        println!("No data");
    } else {
        println!("{} {} {:?}", lower, uppper, data);
        let scale = 10;
        let max = data.iter().copied().max().unwrap();
        for i in 0..scale {
            print!("{:>6}|", max * (scale - i - 1) / (scale - 1));
            for x in data.iter() {
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
        println!("{} {:<8}{}{:>8}", " ".repeat(6), lower, " ".repeat(data.len().saturating_sub(16)) , uppper);
    }
}
