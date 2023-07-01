use rocket_governor::{RocketGovernable, Quota};

pub struct RateLimitGuard;

impl<'r> RocketGovernable<'r> for RateLimitGuard {
    fn quota(_method: rocket_governor::Method, _route_name: &str) -> Quota {
        Quota::per_minute(Self::nonzero(50))
    }
}