export function websocketConnect(socket, userID) {
  return { type: 'WEBSOCKET_CONNECT', socket, userID };
}

export function joinedRoom(key, host, users) {
  return { type: 'JOINED_ROOM', key, host, users };
}

export function gameStart(playerOrder, settings) {
  return { type: 'GAME_START', playerOrder, settings };
}

export function gameReceiveHand(hand) {
  return { type: 'GAME_RECEIVE_HAND', hand };
}

export function gameNextBidder(playerID) {
  return { type: 'GAME_NEXT_BIDDER', playerID };
}

export function playerJoined(username, userID) {
  return { type: 'PLAYER_JOIN', username, userID };
}

export function gamePlayerBid(playerID, bid) {
  if (bid) {
    return { type: 'GAME_PLAYER_BID', playerID, bid };
  }
  return { type: 'GAME_PLAYER_BID_NOTHING', playerID, bid: 'NO_BID' };
}
export function gameNoBids() {
  return { type: 'GAME_NO_BIDS' };
}

export function gameBiddingOver(bid, napoleonID) {
  return { type: 'GAME_BIDDING_OVER', bid, napoleonID };
}

export function gameAlliesChosen(trumpSuit, allies = []) {
  return { type: 'GAME_ALLIES_CHOSEN', trumpSuit, allies };
}

export function gameBecomeAlly() {
  return { type: 'GAME_BECOME_ALLY' };
}

export function gameNextPlayer(playerID, requiredSuit) {
  return { type: 'GAME_NEXT_PLAYER', playerID, requiredSuit };
}

export function gameCardPlayed(playerID, card) {
  return { type: 'GAME_CARD_PLAYED', playerID, card };
}

export function gameRoundOver(winnerPlayerID) {
  return { type: 'GAME_ROUND_OVER', winnerPlayerID };
}

export function gameOver(
  napoleonScoreDelta,
  playerScoreDelta,
  napoleonBet,
  combinedNapoleonScore,
  allies
) {
  return {
    type: 'GAME_OVER',
    napoleonScoreDelta,
    playerScoreDelta,
    napoleonBet,
    combinedNapoleonScore,
    allies,
  };
}
