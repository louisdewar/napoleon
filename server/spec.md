# Napoleon websocket protocol

## General

card = <number><suit>

T is the number 10
number = 2-9|T|J|Q|K|A
suit = H|D|C|S

## Server Commands

These commands are handled in `session.rs`.

### Joined Room
`e{key}(,{username},{session_id})+`

### NextBidder
`bn{player_id}`

### PlayerBid
`bp{player_id}(,{bid})?`

### PlayerJoined
`j{username},{player_id}`

### NoBids
`bn`

### BiddingOver
`bo{bid},{napoleon_id}`

### AlliesChosen
`ac{trump_suit}(,{ally})*`

### BecomeAlly
`ab`

### NextPlayer
`n{player_id}`

### GameStarted
`s{player_id}(,{player_id})*`

### PlayerHand
`h({card_number}{card_suit}(,{card_number}{card_suit})*)?`

### CardPlayed
`p{player_id},{card_number}{card_suit}`

### RoundOver
`r{winner}`

### GameOver
`g{napoleon_score_delta},{player_score_delta},{napoleon_bet},{combined_napoleon_score}(,{ally})*`


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