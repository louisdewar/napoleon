export function websocketConnect(socket, userID) {
  return { type: 'WEBSOCKET_CONNECT', socket, userID };
}

export function joinedRoom(key, users) {
  return { type: 'JOINED_ROOM', key, users };
}

export function gameStart() {
  return { type: 'GAME_START' };
}

export function gameReceiveHand(hand) {
  return { type: 'GAME_RECEIVE_HAND', hand };
}

export function gameNextBidder(playerID) {
  return { type: 'GAME_NEXT_BIDDER', playerID };
}

export function playerJoined(username, userID){
  return { type: 'PLAYER_JOIN', username, userID };
}

export function gamePlayerBid(playerID, bid = 0) {
  if (bid) { // feel like these cases must be handled in store.js instead...
    return { type: 'GAME_PLAYER_BID', playerID, bid };
  }
  return { type: 'GAME_PLAYER_BID_NOTHING', playerID };
}

export function gameNoBids() {
  return { type: 'GAME_NO_BIDS' };
}

export function gameBiddingOver(bid, napoleonID) {
  return { type: 'GAME_BIDDING_OVER', bid, napoleonID }; 
}

export function gameAlliesChosen(trumpSuit, allies = []){ // an ally is a card
  return { type: 'GAME_ALLIES_CHOSEN', trumpSuit, allies };
}