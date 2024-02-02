use diesel_derive_enum::DbEnum;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[derive(Debug, DbEnum)]
#[ExistingTypePath = "crate::db::org::org::sql_types::InstitutionType"]
pub enum InstitutionType {
  Vocational,
  High,
  College,
  University,
  Technical,
  Other,
}

#[derive(Serialize, Deserialize, Clone)]
#[derive(Debug, DbEnum)]
#[ExistingTypePath = "crate::db::org::org::sql_types::OrgType"]
pub enum OrgType {
  Org,
  Ist
}

#[derive(Serialize, Deserialize, Clone)]
#[derive(Debug, DbEnum)]
#[ExistingTypePath = "crate::db::project::project::sql_types::ProposalType"]
pub enum ProposalType {
  Proposal,
  Revised,
  Supplemental,
  Continuation,
  Notice,
  Solicited,
  Other
}

#[derive(Clone, Serialize, Deserialize, Debug, DbEnum)]
#[ExistingTypePath = "crate::db::platform::platform::sql_types::RoleType"]
pub enum RoleType {
  Owner,
  Admin,
  Staff,
  Period
}
