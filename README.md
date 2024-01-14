# REhoST

This challenge was created for the [potluck ctf](https://potluckctf.com) 2023.

The intention of `REhoST` was to create a challengen, which requires rehosting
of firmware to talk to a REST endpoint.

## Description

Our development team created a secure way of booting firmware by authenticating against a different chip. We have the firmware, however we don't have the secure chip at hand :c Can you help us by emulating the secure processor so that we can boot our firmware?

## Category

rev, misc

## Distributed files

Only the binaries `peripheral` and `firmware` were provided to the teams

## Info

During the CTF a bug has been reported, which was misleading for the players.
It has been already fixed in this repo, but the old version remains in the
comments.

## Running the challenge

Start the docker container. This will expose port `5000`, which is the REST
endpoint.

## Flag

`potluck{R3h05TinG_4s_4_S3rv1c3_1s_7h3_Fu7ur3!!!}`
