# Solana ROSCA Project

So you don't get lost.. try reading the manual for once.
It’s a Rotating Savings and Credit Association (ROSCA) built on the Solana blockchain using Rust and TypeScript. Basically, it’s a way for up to 10 people to put in money each week, and one person gets the whole pot each time until everyone gets a turn. I’m learning as I go, so bear with me if it’s not perfect yet!

## What’s This Project About?
This project lets a group of people (max 10) save money together on Solana. One person sets up the group, others join, everyone pays a set amount each week, and one lucky person gets the whole pot each week. It keeps going until everyone has had their turn. I wrote the main program in Rust (`lib.rs`), added some tests in TypeScript (`solana-rosca.ts`), and made a client app (`client/index.ts`) to interact with it.

### Files in the Project
- **`programs/solana-rosca/src/lib.rs`**: The Rust code that runs on Solana. It has functions to create a group, join it, add money, and pick a winner each week.
- **`tests/solana-rosca.ts`**: Tests to make sure the group creation and joining work right.
- **`client/index.ts`**: A little TypeScript app to create a group on my local Solana validator.
- **`client/solana_rosca.json`**: The IDL file that tells the client how to talk to the program.

## How to Set It Up
I’m running this on WSL with a local Solana validator, so here’s how I got it working. You’ll need some tools first!

### Stuff You Need
- **Solana CLI**: I used version 2.2.17. Install it with:
  ```bash
  sh -c "$(curl -sSfL https://release.solana.com/v2.2.17/install)"
