# Napoleon websocket protocol

## General

card = <number><suit>

T is the number 10
number = 2-9|T|J|Q|K|A
suit = H|D|C|S

## Server Commands

## Client Commands
These commands are generally handled in `message_handling.rs` (`j`, `c` are handled by `game_server`).

### Join room
`j{username},{room key}`

### Create room
`c{username}`


### Start game
`s`

### Bid
`b{optional bid (nothing to indicate no bid)}`

### Pick allies
`a{trump suit letter <suit>}(,{ally card <card>})*`

### Play card
`p{card <card>}`
