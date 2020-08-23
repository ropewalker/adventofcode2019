fn main() {
    let mut count = 0;

    for a in 2..=7 {
        for b in a..=9 {
            for c in b..=9 {
                for d in c..=9 {
                    for e in d..=9 {
                        for f in e..=9 {
                            let number =
                                100_000 * a + 10_000 * b + 1_000 * c + 100 * d + 10 * e + f;

                            if number < 245_182 {
                                continue;
                            } else if number > 790_572 {
                                break;
                            } else if (a == b) && (a != c)
                                || (b == c) && (b != d) && (b != a)
                                || (c == d) && (c != b) && (c != e)
                                || (d == e) && (d != c) && (d != f)
                                || (e == f) && (d != e)
                            {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("{}", count);
}
