# Parabolica

Smart Contract racing game inspired by [0xMonaco](https://0xmonaco.ctf.paradigm.xyz/) deployed on Mikasa

## Automomous Selectors

***lap*** - 0x76cb0e8d
***should_run*** - 0xb4d74178
***should_kill*** - 0x44e091c7

## Race

Each race involves 3 players which are smart contracts deployed to Mikasa which we will all racers.

Racers will be called to take turns making moves on racetrack:

- Accelerate
  - increases car's speed by however many units it was called with
  - car's speed never decreases except when it is hit w/ a shell
- Fire a shell
  - shells allow car to set speed of car in front of them to 1
  - can only hit the car in front of the car taking it's turn
- Cut off
  - increases car's speed by however many units it was called with
  - car's speed never decreases except when it is hit w/ a shell

## Install

init mikasa node
`cargo run --release -- --dev`

deploy parabolica
`./scripts/deploy.sh`

register the `lap` function in the UI

query the race track
`./scripts/query.sh`
