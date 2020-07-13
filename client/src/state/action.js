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