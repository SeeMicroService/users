use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt::Debug;
use std::io;
use tokio_postgres::types::private::BytesMut;
use tokio_postgres::types::{to_sql_checked, FromSql, IsNull, ToSql, Type};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Editor,
}

impl ToSql for Role {
    fn to_sql(&self, _ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn Error + Sync + Send>>
    where
        Self: Sized,
    {
        let value = match self {
            Self::Admin => "ADMIN",
            Self::Editor => "EDITOR",
        };

        out.extend_from_slice(value.as_bytes());
        Ok(IsNull::No)
    }

    fn accepts(ty: &Type) -> bool
    where
        Self: Sized,
    {
        ty == &Type::TEXT
    }

    to_sql_checked!();
}

impl<'a> FromSql<'a> for Role {
    fn from_sql(ty: &Type, raw: &'a [u8]) -> Result<Role, Box<dyn Error + Sync + Send>> {
        if *ty != Type::TEXT {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Cannot convert non-TEXT type to Role",
            )
            .into());
        }
        let value = std::str::from_utf8(raw)?;
        match value {
            "ADMIN" => Ok(Self::Admin),
            "EDITOR" => Ok(Self::Editor),
            _ => Err(format!("Unknown role: {}", value).into()),
        }
    }

    fn accepts(ty: &Type) -> bool {
        ty == &Type::TEXT
    }
}
