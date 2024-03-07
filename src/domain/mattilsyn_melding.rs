use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Clone, Serialize, Deserialize, FromRow)]
// #[sqlx(transparent, no_pg_array)]
pub struct MattilsynMelding {
    pub id: Option<i32>,
    pub am_funn: Option<String>,
    pub aktivitet_id: Option<String>,
    pub avsender_epost: Option<String>,
    pub begrunnelse: Option<String>,
    pub eftanummer: Option<String>,
    pub innsendt_av: Option<String>,
    pub journalpost_id: Option<String>,
    pub kategorier: Option<Kategori>,
    pub pm_funn: Option<String>,
    pub saksnummer: Option<String>,
    pub sist_redigert_av: String,
    pub sist_redigert: String,
    pub tilsynsobjektinfo_id: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "kategori")]
pub enum Kategori {
    Dyrevelferd,
    Dyrehelse,
}
