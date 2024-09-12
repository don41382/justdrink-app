use std::time::Duration;

pub trait PrettyTime {
    fn to_pretty_time(&self) -> String;
}

impl PrettyTime for Duration {
    fn to_pretty_time(&self) -> String {
        let total_seconds = self.as_secs();

        if total_seconds < 60 {
            format!("{}s", total_seconds)
        } else {
            let minutes = total_seconds / 60;
            format!("{}m", minutes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pretty_time() {
        assert_eq!(Duration::from_secs(20).to_pretty_time(), "20s");
        assert_eq!(Duration::from_secs(59).to_pretty_time(), "59s");
        assert_eq!(Duration::from_secs(60).to_pretty_time(), "1m");
        assert_eq!(Duration::from_secs(90).to_pretty_time(), "1m");
        assert_eq!(Duration::from_secs(120).to_pretty_time(), "2m");
        assert_eq!(Duration::from_secs(3600).to_pretty_time(), "60m");
    }
}