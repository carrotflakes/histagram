pub fn histagram(mut iter: impl Iterator<Item = f64>) {
    let desire_size = 30;
    if let Some(first) = iter.next() {
        let mut first_count = 1;
        let second = {
            loop {
                if let Some(x) = iter.next() {
                    if x != first {
                        break x;
                    }
                } else {
                    println!("All {}", first);
                    return;
                }
                first_count += 1;
            }
        };

        let mut min = first.min(second);
        let mut max = first.max(second);
        let mut buckets = vec![0; desire_size];
        buckets[0] = first_count;
        buckets[desire_size - 1] = 1;
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
        println!("{} {} {:?}", min, max, buckets);
    } else {
        println!("No data");
    }
}
