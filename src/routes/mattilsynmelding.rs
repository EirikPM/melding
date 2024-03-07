use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;

use crate::domain::mattilsyn_melding::MattilsynMelding;
use crate::routes::dtos::OpprettMattilsynMelding;

pub async fn opprett_mattilsynmelding(
    State(db): State<Arc<sqlx::PgPool>>,
    Json(payload): Json<OpprettMattilsynMelding>,
) -> (StatusCode, Json<MattilsynMelding>) {
    let ny_melding = MattilsynMelding {
        sist_redigert_av: "Eirik".to_string(),
        sist_redigert: Utc::now().to_string(),
        id: None,
        am_funn: payload.am_funn,
        aktivitet_id: payload.aktivitet_id,
        avsender_epost: payload.avsender_epost,
        begrunnelse: payload.begrunnelse,
        eftanummer: payload.eftanummer,
        innsendt_av: payload.innsendt_av,
        journalpost_id: payload.journalpost_id,
        kategorier: payload.kategorier,
        pm_funn: payload.pm_funn,
        saksnummer: payload.saksnummer,
        tilsynsobjektinfo_id: payload.tilsynsobjektinfo_id,
    };

    let melding = sqlx::query_as::<_, MattilsynMelding>(
        "INSERT INTO melding (innsendt_av, beskrivelse, kategorier, sist_redigert, sist_redigert_av) VALUES ($1, $2, $3, $4, $5) RETURNING *")
        .bind(ny_melding.innsendt_av.unwrap())
        .bind(ny_melding.begrunnelse.unwrap())
        .bind(ny_melding.kategorier.unwrap())
        .bind(Utc::now().to_string())
        .bind(ny_melding.sist_redigert_av)
        .fetch_one(&*db)
        .await
        .expect("Failed to insert ny melding to db");

    (StatusCode::CREATED, Json(melding))
}

pub async fn get_meldinger(
    State(db): State<Arc<sqlx::PgPool>>,
) -> (StatusCode, Json<Vec<MattilsynMelding>>) {
    let meldinger = sqlx::query_as::<_, MattilsynMelding>("SELECT * FROM melding")
        .fetch_all(&*db)
        .await
        .unwrap();

    (StatusCode::OK, Json(meldinger))
}

pub async fn slett_melding(State(db): State<Arc<sqlx::PgPool>>, Path(id): Path<i32>) -> StatusCode {
    let _ = sqlx::query("DELETE FROM melding WHERE id = $1")
        .bind(id)
        .execute(&*db)
        .await
        .map_err(|_| StatusCode::NOT_FOUND);

    StatusCode::NO_CONTENT
}
