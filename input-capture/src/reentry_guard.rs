use std::time::{Duration, Instant};

/// Prevents an immediate re-entry into an input barrier after local capture
/// has been released. This is intentionally small: it gives the local cursor
/// one event cycle to settle without making an intentional return feel slow.
#[derive(Debug, Default)]
pub(crate) struct ReentryGuard {
    blocked_until: Option<Instant>,
}

impl ReentryGuard {
    pub(crate) fn block_for(&mut self, duration: Duration) {
        self.blocked_until = Some(Instant::now() + duration);
    }

    pub(crate) fn blocks(&mut self) -> bool {
        self.blocks_at(Instant::now())
    }

    fn blocks_at(&mut self, now: Instant) -> bool {
        match self.blocked_until {
            Some(until) if now < until => true,
            Some(_) => {
                self.blocked_until = None;
                false
            }
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_only_until_the_deadline() {
        let now = Instant::now();
        let mut guard = ReentryGuard {
            blocked_until: Some(now + Duration::from_millis(150)),
        };

        assert!(guard.blocks_at(now + Duration::from_millis(149)));
        assert!(!guard.blocks_at(now + Duration::from_millis(150)));
        assert!(!guard.blocks_at(now + Duration::from_millis(151)));

        guard.block_for(Duration::ZERO);
        assert!(!guard.blocks());
    }
}
