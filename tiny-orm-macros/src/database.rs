use quote::format_ident;
use syn::Ident;

#[allow(dead_code)]
#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DbType {
    #[default]
    Postgres,
    MySQL,
    Sqlite,
}

impl DbType {
    pub fn parse(identifier: &str) -> Self {
        match identifier {
            "postgres" => Self::Postgres,
            "mysql" => Self::MySQL,
            "sqlite" => Self::Sqlite,
            _ => panic!("unknown db type: {identifier}"),
        }
    }

    pub fn to_ident(&self) -> Ident {
        match self {
            DbType::Postgres => format_ident!("PgExecutor"),
            DbType::MySQL => format_ident!("MySqlExecutor"),
            DbType::Sqlite => format_ident!("SqliteExecutor"),
        }
    }
}
