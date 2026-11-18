//! What keeps a public server usable for everyone: how many runs one address
//! may create, how many connections it may hold, how many runs the server
//! keeps at most, and when a run nobody plays any more is removed.
//!
//! A server made with [`Limits::default`] has none of these, as the one
//! inside the desktop app; `soulsync-server` reads them from the environment.

use axum::http::HeaderMap;
use std::collections::{HashMap, VecDeque};
use std::net::{IpAddr, SocketAddr};
use std::sync::{Arc, Mutex};

const HOUR_MS: u64 = 60 * 60 * 1000;
const DAY_MS: u64 = 24 * HOUR_MS;

#[derive(Debug, Default)]
pub struct Limits {
    /// Runs one address may create per hour; 0 is no limit.
    pub runs_per_hour: usize,
    /// Connections one address may hold at once; 0 is no limit.
    pub connections: usize,
    /// Runs the server keeps at most; 0 is no limit.
    pub max_runs: usize,
    /// Days after which a run nobody changed is removed; 0 is never.
    pub idle_days: u64,
    /// Run codes that do not exist one address may ask for per hour before
    /// it is refused every run for a while; 0 is no limit. Codes are short
    /// enough to type, so without this they could be tried one by one.
    pub misses_per_hour: usize,
    /// Whether the address comes from `X-Forwarded-For`, as behind the
    /// bundled Caddy. Only set when nobody reaches the server directly, or
    /// anyone could claim any address.
    pub trust_proxy: bool,
    pub(crate) created: Mutex<HashMap<IpAddr, VecDeque<u64>>>,
    pub(crate) open: Mutex<HashMap<IpAddr, usize>>,
    pub(crate) misses: Mutex<HashMap<IpAddr, VecDeque<u64>>>,
}

impl Limits {
    /// Limits for a public server, each changeable by an environment variable:
    /// `SOULSYNC_RUNS_PER_HOUR` (10), `SOULSYNC_CONNECTIONS` (20),
    /// `SOULSYNC_MAX_RUNS` (2000), `SOULSYNC_IDLE_DAYS` (60),
    /// `SOULSYNC_MISSES_PER_HOUR` (200) and `SOULSYNC_TRUST_PROXY` (off; `1`
    /// behind a reverse proxy).
    pub fn from_env() -> Self {
        let num = |key: &str, default: u64| {
            std::env::var(key)
                .ok()
                .and_then(|v| v.trim().parse().ok())
                .unwrap_or(default)
        };
        Limits {
            runs_per_hour: num("SOULSYNC_RUNS_PER_HOUR", 10) as usize,
            connections: num("SOULSYNC_CONNECTIONS", 20) as usize,
            max_runs: num("SOULSYNC_MAX_RUNS", 2000) as usize,
            idle_days: num("SOULSYNC_IDLE_DAYS", 60),
            misses_per_hour: num("SOULSYNC_MISSES_PER_HOUR", 200) as usize,
            trust_proxy: num("SOULSYNC_TRUST_PROXY", 0) == 1,
            ..Limits::default()
        }
    }

    /// The address a request comes from, if known. An IPv6 address stands
    /// for its whole /64 network, which usually belongs to one household or
    /// one server and would otherwise give it billions of addresses.
    pub fn client(&self, peer: Option<SocketAddr>, headers: &HeaderMap) -> Option<IpAddr> {
        self.address(peer, headers).map(network)
    }

    fn address(&self, peer: Option<SocketAddr>, headers: &HeaderMap) -> Option<IpAddr> {
        if self.trust_proxy {
            // The proxy appends the address it saw; whatever is before it
            // the client may have written itself.
            let forwarded = headers
                .get_all("x-forwarded-for")
                .iter()
                .filter_map(|v| v.to_str().ok())
                .flat_map(|v| v.split(','))
                .next_back()
                .and_then(|a| a.trim().parse().ok());
            if forwarded.is_some() {
                return forwarded;
            }
        }
        peer.map(|p| p.ip())
    }

    /// Whether `ip` asked for too many unknown run codes in the last hour.
    pub fn too_many_misses(&self, ip: Option<IpAddr>, now: u64) -> bool {
        let (Some(ip), n) = (ip, self.misses_per_hour) else {
            return false;
        };
        n > 0 && recent(&mut self.misses.lock().unwrap(), ip, now) >= n
    }

    /// Notes that `ip` asked for a run code that does not exist.
    pub fn missed(&self, ip: Option<IpAddr>, now: u64) {
        if let Some(ip) = ip.filter(|_| self.misses_per_hour > 0) {
            let mut misses = self.misses.lock().unwrap();
            recent(&mut misses, ip, now);
            misses.entry(ip).or_default().push_back(now);
        }
    }

    /// Notes a new run by `ip`, unless it made too many in the last hour.
    pub fn may_create(&self, ip: Option<IpAddr>, now: u64) -> bool {
        let (Some(ip), n) = (ip, self.runs_per_hour) else {
            return true;
        };
        if n == 0 {
            return true;
        }
        let mut created = self.created.lock().unwrap();
        if recent(&mut created, ip, now) >= n {
            return false;
        }
        created.entry(ip).or_default().push_back(now);
        true
    }

    /// Takes a connection for `ip`, unless it holds too many. The connection
    /// counts until the returned guard is dropped.
    pub fn connect(self: &Arc<Self>, ip: Option<IpAddr>) -> Option<Connection> {
        let Some(ip) = ip.filter(|_| self.connections > 0) else {
            return Some(Connection {
                limits: self.clone(),
                ip: None,
            });
        };
        let mut open = self.open.lock().unwrap();
        let n = open.entry(ip).or_default();
        if *n >= self.connections {
            return None;
        }
        *n += 1;
        Some(Connection {
            limits: self.clone(),
            ip: Some(ip),
        })
    }

    /// Whether a run last changed at `touched` has been idle too long.
    pub fn is_idle(&self, touched: u64, now: u64) -> bool {
        self.idle_days > 0 && touched + self.idle_days * DAY_MS < now
    }
}

/// Forgets times older than an hour and returns how many `ip` has left.
fn recent(times: &mut HashMap<IpAddr, VecDeque<u64>>, ip: IpAddr, now: u64) -> usize {
    times.retain(|_, t| {
        while t.front().is_some_and(|&t| t + HOUR_MS <= now) {
            t.pop_front();
        }
        !t.is_empty()
    });
    times.get(&ip).map_or(0, VecDeque::len)
}

/// An IPv4 address as it is; an IPv6 address as its /64 network.
fn network(ip: IpAddr) -> IpAddr {
    match ip {
        IpAddr::V4(_) => ip,
        IpAddr::V6(v6) => match v6.to_ipv4_mapped() {
            Some(v4) => IpAddr::V4(v4),
            None => {
                let s = v6.segments();
                IpAddr::from([s[0], s[1], s[2], s[3], 0, 0, 0, 0])
            }
        },
    }
}

/// A connection counted against its address.
pub struct Connection {
    limits: Arc<Limits>,
    ip: Option<IpAddr>,
}

impl Drop for Connection {
    fn drop(&mut self) {
        let Some(ip) = self.ip else { return };
        let mut open = self.limits.open.lock().unwrap();
        if let Some(n) = open.get_mut(&ip) {
            *n -= 1;
            if *n == 0 {
                open.remove(&ip);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ip(last: u8) -> Option<IpAddr> {
        Some(IpAddr::from([203, 0, 113, last]))
    }

    #[test]
    fn counts_new_runs_per_address_and_hour() {
        let l = Limits {
            runs_per_hour: 2,
            ..Limits::default()
        };
        assert!(l.may_create(ip(1), 0));
        assert!(l.may_create(ip(1), 1));
        assert!(!l.may_create(ip(1), 2));
        assert!(l.may_create(ip(2), 2));
        assert!(l.may_create(None, 2));
        assert!(l.may_create(ip(1), HOUR_MS + 1));
        assert!(Limits::default().may_create(ip(1), 0));
    }

    #[test]
    fn counts_open_connections() {
        let l = Arc::new(Limits {
            connections: 2,
            ..Limits::default()
        });
        let a = l.connect(ip(1)).unwrap();
        let _b = l.connect(ip(1)).unwrap();
        assert!(l.connect(ip(1)).is_none());
        assert!(l.connect(ip(2)).is_some());
        drop(a);
        assert!(l.connect(ip(1)).is_some());
    }

    #[test]
    fn trusts_the_proxy_only_when_told() {
        let peer: SocketAddr = "10.0.0.2:5000".parse().unwrap();
        let mut h = HeaderMap::new();
        h.insert("x-forwarded-for", "1.2.3.4, 198.51.100.7".parse().unwrap());
        let open = Limits::default();
        assert_eq!(open.client(Some(peer), &h), Some(peer.ip()));
        let behind = Limits {
            trust_proxy: true,
            ..Limits::default()
        };
        assert_eq!(behind.client(Some(peer), &h), "198.51.100.7".parse().ok());
        assert_eq!(
            behind.client(Some(peer), &HeaderMap::new()),
            Some(peer.ip())
        );
    }

    #[test]
    fn an_ipv6_network_counts_as_one_address() {
        let l = Limits::default();
        let at = |a: &str| {
            l.client(
                Some(SocketAddr::new(a.parse().unwrap(), 1)),
                &HeaderMap::new(),
            )
        };
        assert_eq!(at("2001:db8:1:2::1"), at("2001:db8:1:2:ffff::9"));
        assert_ne!(at("2001:db8:1:2::1"), at("2001:db8:1:3::1"));
        assert_eq!(at("::ffff:203.0.113.5"), at("203.0.113.5"));
    }

    #[test]
    fn too_many_unknown_codes_lock_an_address_out_for_an_hour() {
        let l = Limits {
            misses_per_hour: 2,
            ..Limits::default()
        };
        assert!(!l.too_many_misses(ip(1), 0));
        l.missed(ip(1), 0);
        l.missed(ip(1), 1);
        assert!(l.too_many_misses(ip(1), 2));
        assert!(!l.too_many_misses(ip(2), 2));
        assert!(!l.too_many_misses(ip(1), HOUR_MS + 1));
        Limits::default().missed(ip(1), 0);
        assert!(!Limits::default().too_many_misses(ip(1), 0));
    }

    #[test]
    fn a_run_is_idle_after_the_set_days() {
        let l = Limits {
            idle_days: 2,
            ..Limits::default()
        };
        assert!(!l.is_idle(0, 2 * DAY_MS));
        assert!(l.is_idle(0, 2 * DAY_MS + 1));
        assert!(!Limits::default().is_idle(0, u64::MAX / 2));
    }
}
