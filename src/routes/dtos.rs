use serde::{Deserialize, Serialize};

use crate::domain::mattilsyn_melding::Kategori;

#[derive(Serialize, Deserialize)]
pub struct OpprettMattilsynMelding {
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
    pub tilsynsobjektinfo_id: Option<String>,
}
