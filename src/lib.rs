//! actor-router — message routing strategies across actor groups.

/// Routing logic selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutingStrategy {
    RoundRobin,
    Random,
    Broadcast,
    ConsistentHash,
}

/// A router that selects target indices based on a strategy.
#[derive(Debug, Clone)]
pub struct Router {
    strategy: RoutingStrategy,
    next: usize,
    capacity: usize,
}

impl Router {
    pub fn new(strategy: RoutingStrategy, capacity: usize) -> Self {
        Self { strategy, next: 0, capacity }
    }

    /// Returns the indices to route to.
    pub fn route(&mut self) -> Vec<usize> {
        if self.capacity == 0 {
            return vec![];
        }
        match self.strategy {
            RoutingStrategy::RoundRobin => {
                let idx = self.next % self.capacity;
                self.next = self.next.wrapping_add(1);
                vec![idx]
            }
            RoutingStrategy::Broadcast => (0..self.capacity).collect(),
            RoutingStrategy::Random | RoutingStrategy::ConsistentHash => {
                // simplified: pick next slot
                let idx = self.next % self.capacity;
                self.next = self.next.wrapping_add(1);
                vec![idx]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_robin_routing() {
        let mut r = Router::new(RoutingStrategy::RoundRobin, 3);
        assert_eq!(r.route(), vec![0]);
        assert_eq!(r.route(), vec![1]);
        assert_eq!(r.route(), vec![2]);
        assert_eq!(r.route(), vec![0]);
    }

    #[test]
    fn broadcast_routing() {
        let mut r = Router::new(RoutingStrategy::Broadcast, 3);
        assert_eq!(r.route(), vec![0, 1, 2]);
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
