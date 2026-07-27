//! Seats and partnerships.
//!
//! Implements `S = Z/4Z` with fixed opposite-seat partnerships
//! (Math §2.3; Rules R-SEAT-01, R-TEAM-01).

/// A seat in clockwise order, `Z/4Z` (Math §2.3).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Seat(u8);

impl Seat {
    /// Validating constructor for values `0..=3`.
    pub const fn new(value: u8) -> Option<Seat> {
        if value <= 3 {
            Some(Seat(value))
        } else {
            None
        }
    }

    /// The four seats in clockwise order.
    pub const ALL: [Seat; 4] = [Seat(0), Seat(1), Seat(2), Seat(3)];

    /// Storage index `0..4`.
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// The clockwise successor `s + 1 mod 4` (Math §2.3).
    pub const fn next(self) -> Seat {
        Seat((self.0 + 1) % 4)
    }

    /// Seat offset by `k` clockwise steps.
    pub const fn offset(self, k: u8) -> Seat {
        Seat((self.0 + (k % 4)) % 4)
    }

    /// The partner seat (opposite; R-TEAM-01).
    pub const fn partner(self) -> Seat {
        Seat((self.0 + 2) % 4)
    }

    /// The fixed partnership `θ(s) = s mod 2` (Math §2.3).
    pub const fn team(self) -> Team {
        Team(self.0 % 2)
    }
}

/// A partnership, `Z/2Z` (Math §2.3; Rules R-TEAM-01).
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Team(u8);

impl Team {
    /// Validating constructor for values `0..=1`.
    pub const fn new(value: u8) -> Option<Team> {
        if value <= 1 {
            Some(Team(value))
        } else {
            None
        }
    }

    /// The two partnerships.
    pub const ALL: [Team; 2] = [Team(0), Team(1)];

    /// Storage index `0..2`.
    pub const fn index(self) -> usize {
        self.0 as usize
    }

    /// The opposing partnership.
    pub const fn opponent(self) -> Team {
        Team(1 - self.0)
    }
}
