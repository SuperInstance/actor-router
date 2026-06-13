# Actor Router

**Actor Router** is a Rust library implementing message routing strategies across groups of actors — round-robin, random, broadcast, and consistent-hash — with pluggable selection policies for different communication patterns.

## Why It Matters

In a multi-actor system, the routing strategy determines the communication topology. Different workloads demand different topologies: request-reply workloads need round-robin for even distribution; fan-out computations need broadcast; stateful sessions need consistent-hash to route related messages to the same actor. The Router pattern, codified in Akka and Erlang/OTP, abstracts the selection logic from the application code, allowing routing strategy changes without modifying message handlers. This is essential for systems that must dynamically adapt their communication patterns — for example, switching from round-robin to broadcast during consensus rounds.

## How It Works

The router maintains a strategy enum and a capacity counter. Each `route()` call returns the indices to dispatch to:

**Round-Robin:** Returns a single index, advancing a modular counter. O(1) per call. Guarantees perfectly even distribution (±1 across N calls).

**Broadcast:** Returns all indices `[0..capacity)`. O(N) to construct, O(N) to process. Used for control messages, heartbeats, and consensus proposals where every actor must receive the same message.

**Consistent Hash:** Routes based on a hash of the message key. The implementation uses a simplified fallback to round-robin, but the full algorithm places actors on a virtual ring (hash ring) with V replicas each:

```
ring = sorted([(hash(actor_i, replica_j), i) for all i, j])
route(key) → ring[bisect(ring, hash(key)) % len(ring)].actor_index
```

When an actor joins or leaves, only K/V keys need remapping (where V = virtual nodes per actor, typically 150). This minimizes disruption compared to hash-table rehashing which remaps everything.

**Random:** Theoretically uniform, but suffers from short-run clustering. Suitable only when approximate fairness is acceptable.

| Strategy | Distribution | Key Benefit |
|----------|-------------|-------------|
| Round-robin | Perfect ±1 | Simplicity |
| Broadcast | All actors | Fan-out |
| Consistent-hash | Key-affinity | Session locality |
| Random | Statistical | Stateless |

## Quick Start

```rust
fn main() {
    let mut router = Router::new(RoutingStrategy::RoundRobin, 3);
    assert_eq!(router.route(), vec![0]);
    assert_eq!(router.route(), vec![1]);
    assert_eq!(router.route(), vec![2]);
    assert_eq!(router.route(), vec![0]); // wraps

    let mut bcast = Router::new(RoutingStrategy::Broadcast, 3);
    assert_eq!(bcast.route(), vec![0, 1, 2]);
}
```

## API

| Type/Method | Description |
|-------------|-------------|
| `RoutingStrategy` | Enum: RoundRobin, Random, Broadcast, ConsistentHash |
| `Router::new` | Construct with strategy and actor count |
| `Router::route` | Returns `Vec<usize>` of target indices |

## Architecture Notes

The Router implements the **message dispatch topology** in the SuperInstance actor framework. Within γ + η = C, the routing strategy controls how conservation-law monitoring signals propagate: broadcast for species census (all nodes must report), round-robin for task dispatch (even γ-layer load), and consistent-hash for agent-session affinity in η-layer conversations.

See [ARCHITECTURE.md](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

**Mathematical foundation of consistent hashing:** The hash ring places nodes at positions determined by hash(node_id + replica_index). For V = 150 virtual nodes per physical node, the standard deviation of per-node load is σ ≈ (N/K) × √(K/V) where K = total keys and N = nodes. With V = 150, the load imbalance stays below 10% for K/N > 1000 — virtually perfect distribution.

**Consensus integration:** The Broadcast strategy is used during consensus rounds where all actors must evaluate the same proposal. This is a one-to-many communication pattern with O(N) fan-out, used in Paxos-derivative protocols where all participants must vote on each proposal.

## References

1. Karger, D. et al. (1997). "Consistent Hashing and Random Trees: Distributed Caching Protocols for Relieving Hot Spots on the World Wide Web." *STOC*.
2. Haller, P. & Odersky, M. (2009). "Scala Actors: Unifying Thread-Based and Event-Based Programming." *Theoretical Computer Science*.

## License

MIT
