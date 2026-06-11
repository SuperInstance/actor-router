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
