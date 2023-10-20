use diesel::sql_types::Text;
use diesel::serialize::{ToSql, IsNull};
use diesel::deserialize::{FromSql, Result as FromSqlResult, FromSqlRow};
use diesel::pg::Pg;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone, Copy, FromSqlRow, Queryable)]
#[serde(rename_all = "snake_case")]
pub enum InstitutionType {
  Vocational,
  High,
  College,
  University,
  Technical,
  Other,
}

impl ToSql<Text, Pg> for InstitutionType {
  fn to_sql(&self) -> Result<IsNull, Box<dyn std::error::Error + Send + Sync>> {
    let value: &str = match self {
      InstitutionType::Vocational => "vocational",
      InstitutionType::High => "high",
      InstitutionType::College => "college",
      InstitutionType::University => "university",
      InstitutionType::Technical => "technical",
      InstitutionType::Other => "other",
    };
    ToSql::<Text, Pg>::to_sql(&value, Pg)
  }
}

impl FromSql<Text, Pg> for InstitutionType {
  fn from_sql(value: Option<&[u8]>) -> FromSqlResult<Self> {
    let value = String::from_utf8(value.unwrap().to_vec())
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
    match value.as_str() {
      "vocational" => Ok(InstitutionType::Vocational),
      "high" => Ok(InstitutionType::High),
      "college" => Ok(InstitutionType::College),
      "university" => Ok(InstitutionType::University),
      "technical" => Ok(InstitutionType::Technical),
      "other" => Ok(InstitutionType::Other),
      _ => Err("Invalid institution type value".into()),
    }
  }
}