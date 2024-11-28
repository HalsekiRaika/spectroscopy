use std::cmp::Ordering;
use time::OffsetDateTime;

/// Basic format of the data to be saved.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Payload {
    /// Aggregate entity identifier
    pub id: String,
    /// Unique sequence value at a specific Entity
    pub sequence_id: i64,
    /// Unique id for each data format used in [`ResolveMapping`](spectroscopy_core::mapping::ResolveMapping)
    pub registry_key: String,
    /// Data body in binary format
    pub bytes: Vec<u8>,
    /// Time the Event was generated
    pub created_at: OffsetDateTime
}

impl Eq for Payload {}

impl PartialEq<Self> for Payload {
    fn eq(&self, other: &Self) -> bool {
        self.sequence_id.eq(&other.sequence_id)
        && self.id.eq(&other.id)
        && self.created_at.eq(&other.created_at)
    }
}

impl PartialOrd<Self> for Payload {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Payload {
    fn cmp(&self, other: &Self) -> Ordering {
        self.sequence_id.cmp(&other.sequence_id)
            .then_with(|| self.created_at.cmp(&other.created_at))
            .then_with(|| self.id.cmp(&other.id))
    }
}
