 # Redis server

## Run

- installer redis-cli fra redis sin website

redis-cli kan bli lasted med node fra npm: "npm install -g redis-cli"
- installer rust fra rust sin (website)[https://www.rust-lang.org/tools/install]
- clone repoet: https://github.com/Esk3/rustis
- kjør: cargo run i repoet
- kjør: redis-cli

## Features

- [Key Value Store](#key-value-store)

- [Replication](#replication)

- [Redis Streams](#redis-streams)

- [Request Pipelining](#request-pipelining)

- [Transcations](#transactions)

### Key Value Store

SET key value

sets a key value pair

GET key

gets a key value pair or null if not set

### Replication

Kan bli kjørt som replica med argument-ed: replicaof port

Serveren vil da lage en tilkobling til porten på localhost og lytte etter oppdateringer til databasen

### Redis Streams

Redis streams er en append only log of data som det går an og subscribe til for å oppdateringer

Bruk: XADD stream key * field value...

for og legge til verdier i stremen

Bruk: XREAD eller XRANGE for å hente verider fra stremen

### Request Pipelining

Med pipelining samler du alle forespørslene og sender de på en gang uten å vente på svar først. Det samme skjer med svarene og.

Det gjør det ransker og sende flere forespørsler hvis tilkoblingen er treig og det sparer også på "read" og "write" system calls fordi det kan bli skrevet/lest på en gang

### Transactions

send MULTI kommando for og starte en transaction. alle følgende kommandoer vil bli lagret men ikke kjørt.

send EXEC kommando for og kjøre alle kommandoene på en gang.

## Kode overview

All koden ligger under src/

- i main.rs bruker jeg et populerte Command Line Arguement Parser bibliotek for å se om programmet skal være en leder eller en replica/følger og hvilken port den skal bruke.

Så bruker jeg "Builder pattern" for og sette opp serveren med de riktige instillingene

- i resp/value/ er serializing/deserializing

- i repository/ er koden om og lagre data. daten blir brukt mellom flere threads så den blir låst med en mutex for syncronasjon

- i event/ er event publisher og event subscriber types som blir brukt når en klient setter en verdi så publisher det et event som threadsne som er tilkoblet til følgere/replicas sender videre

- i connection/ er hvor meste av koden er, den har ansvar for og håntere inkomene tilkoblinger fra klienter og følgere/replicas og utgående tilkoblinger til leder/master

    den definerer også tilkoblingen



