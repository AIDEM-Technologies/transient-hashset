#[cfg(test)]
mod tests {
    use tokio::{time::{Duration, sleep}};
    use itertools::Itertools;
    use crate::{Perfect, TransientHashSet, Naive};

    #[tokio::test]
    async fn perfect_test() {
        let perfect: Perfect<u8, u8> = Perfect::new(Duration::from_secs(1));
        assert!(!perfect.contains(0,0).await);
        assert!(!perfect.contains(1,0).await);
        assert!(!perfect.contains(0,1).await);
        assert!(!perfect.contains(1,1).await);
        assert!(perfect.contains(0,0).await);
        sleep(Duration::from_secs(2)).await;
        assert!(!perfect.contains(0,0).await);
    }

    #[tokio::test]
    async fn naive_test() {
        let naive: Naive<u8, u8> = Naive::new(Duration::from_secs(1));
        assert!(!naive.contains(0,0).await);
        assert!(!naive.contains(1,0).await);
        assert!(!naive.contains(0,1).await);
        assert!(!naive.contains(1,1).await);
        assert!(naive.contains(0,0).await);
        sleep(Duration::from_secs(2)).await;
        assert!(!naive.contains(0,0).await);
    }

    #[tokio::test]
    async fn evmap_test() {
        let evmap: TransientHashSet<u8, u8> = TransientHashSet::new(Duration::from_secs(1));
        assert!(!evmap.contains(0,0).await);
        assert!(!evmap.contains(1,0).await);
        assert!(!evmap.contains(0,1).await);
        assert!(!evmap.contains(1,1).await);
        assert!(evmap.contains(0,0).await);
        sleep(Duration::from_secs(2)).await;
        assert!(!evmap.contains(0,0).await);
    }

    #[tokio::test]
    async fn soundness_test() {
        let charset = "1234567890qwertyuioplkjhgfdsazxcvbnm";
    
        let different_key_values = (0..100_000).map(|_| (random_string::generate(2, charset), random_string::generate(2, charset))).collect_vec();
        let evmap: TransientHashSet<String, String> = TransientHashSet::new(Duration::from_secs(5));
        let perfect: Perfect<String, String> = Perfect::new(Duration::from_secs(5));
        for (first, second) in different_key_values.into_iter() {
            assert_eq!(perfect.contains(first.clone(), second.clone()).await, evmap.contains(first, second).await);
        }

        let different_key_values = (0..100_000).map(|_| (random_string::generate(2, charset), random_string::generate(2, charset))).collect_vec();
        let naive: Naive<String, String> = Naive::new(Duration::from_secs(5));
        let perfect: Perfect<String, String> = Perfect::new(Duration::from_secs(5));
        for (first, second) in different_key_values.into_iter() {
            assert_eq!(perfect.contains(first.clone(), second.clone()).await, naive.contains(first, second).await);
        }

        let different_key_values = (0..100_000).map(|_| (random_string::generate(2, charset), random_string::generate(2, charset))).collect_vec();
        let naive: Naive<String, String> = Naive::new(Duration::from_secs(5));
        let evmap: TransientHashSet<String, String> = TransientHashSet::new(Duration::from_secs(5));
        for (first, second) in different_key_values.into_iter() {
            assert_eq!(evmap.contains(first.clone(), second.clone()).await, naive.contains(first, second).await);
        }
    }
}