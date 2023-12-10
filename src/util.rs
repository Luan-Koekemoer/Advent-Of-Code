pub mod extended_math {
    pub fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 {
            return a;
        }
        return gcd(b, a % b);
    }

    pub fn lcm(a: u64, b: u64) -> u64 {
        lcm_vec(vec![a, b])
    }

    pub fn lcm_vec(arr: Vec<u64>) -> u64 {
        let mut ans = arr[0];

        for i in 1..arr.len() {
            ans = (arr[i] * ans) / (gcd(arr[i], ans));
        }
        return ans;
    }
}

pub mod challenge_file_reader {
    use std::fs;
    use std::path::Path;

    pub fn read_file_to_list(path: &str) -> Vec<String> {
        if Path::new(path).exists() {
            return fs::read_to_string(path)
                .expect("Could not read file...")
                .lines()
                .map(String::from)
                .collect()
        }
        return vec![];
    }
}
