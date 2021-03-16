//! <b style="font-variant:small-caps">crate_owners.csv</b>

use crate::crates::CrateId;
use crate::error::{err, Result};
use crate::load::FromRecord;
use crate::teams::TeamId;
use crate::users::UserId;
use chrono::NaiveDateTime;
use csv::StringRecord;
use serde::Deserialize;

#[derive(Copy, Clone, Debug)]
pub enum OwnerId {
    /// owner_kind=0
    User(UserId),
    /// owner_kind=1
    Team(TeamId),
}

#[derive(Debug)]
#[non_exhaustive]
pub struct Row {
    pub crate_id: CrateId,
    pub owner_id: OwnerId,
    pub created_at: NaiveDateTime,
    pub created_by: Option<UserId>,
}

impl FromRecord for Row {
    fn from_record(record: &StringRecord, headers: &StringRecord) -> Result<Self> {
        de(record, headers)
    }
}

fn de(record: &StringRecord, headers: &StringRecord) -> Result<Row> {
    #[derive(Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Record {
        crate_id: CrateId,
        #[serde(deserialize_with = "crate::datetime::de")]
        created_at: NaiveDateTime,
        created_by: Option<UserId>,
        owner_id: u32,
        owner_kind: u8,
    }

    let record: Record = record.deserialize(Some(headers)).map_err(err)?;

    let owner_id = match record.owner_kind {
        0 => OwnerId::User(UserId(record.owner_id)),
        1 => OwnerId::Team(TeamId(record.owner_id)),
        other => {
            return Err(err(format_args!(
                "unrecognized crate_owners.csv owner_kind: {}",
                other,
            )))
        }
    };

    Ok(Row {
        crate_id: record.crate_id,
        created_at: record.created_at,
        created_by: record.created_by,
        owner_id,
    })
}

impl PartialEq<UserId> for OwnerId {
    fn eq(&self, other: &UserId) -> bool {
        match self {
            OwnerId::User(user_id) => user_id == other,
            OwnerId::Team(_) => false,
        }
    }
}

impl PartialEq<TeamId> for OwnerId {
    fn eq(&self, other: &TeamId) -> bool {
        match self {
            OwnerId::User(_) => false,
            OwnerId::Team(team_id) => team_id == other,
        }
    }
}

impl PartialEq<OwnerId> for UserId {
    fn eq(&self, other: &OwnerId) -> bool {
        other == self
    }
}

impl PartialEq<OwnerId> for TeamId {
    fn eq(&self, other: &OwnerId) -> bool {
        other == self
    }
}
