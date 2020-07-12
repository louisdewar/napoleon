export function websocket_connect(socket, userID) {
  return { type: 'WEBSOCKET_CONNECT', socket, userID };
}

export function joined_room(key) {
  return { type: 'JOINED_ROOM', key };
}

export function game_start() {
  return { type: 'GAME_START' };
}

export function game_receive_hand(hand) {
  return { type: 'GAME_RECEIVE_HAND', hand };
}

export function game_next_bidder(playerID) {
  return { type: 'GAME_NEXT_BIDDER', playerID };
}
