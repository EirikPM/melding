CREATE TYPE kategori AS ENUM ('Dyrehelse', 'Dyrevelferd');
CREATE TABLE melding (
  id SERIAL PRIMARY KEY,
  am_funn VARCHAR(255),
  aktivitet_id VARCHAR(255),
  avsender_epost VARCHAR(255),
  begrunnelse VARCHAR(255),
  eftanummer VARCHAR(255),
  innsendt_av VARCHAR(255),
  journalpost_id VARCHAR(255),
  beskrivelse VARCHAR(255),
  kategorier kategori,
  pm_funn VARCHAR(255),
  saksnummer VARCHAR(255),
  sist_redigert_av VARCHAR(255),
  sist_redigert VARCHAR(255),
  tilsynsobjektinfo_id VARCHAR(255)
);
