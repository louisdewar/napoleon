import { websocketConnect, joinedRoom, playerJoined, gameStart, gameReceiveHand } from './action';

export default class WebsocketManager {
  constructor(host, store) {
    this.socket = new WebSocket(host);
    this.socket.onmessage = this.onMessage.bind(this);
    this.store = store;
  }

  onMessage(event) {
    const msg = event.data;

    if (msg[0] === 'c') {
      const id = msg.slice(1);
      this.store.dispatch(websocketConnect(this, id));
    } else if (msg[0] === 'e') {
      const parts = msg.split(',');
      const key = parts[0].slice(1);

      const users = { };
      for (var i = 1; i < parts.length - 1; i += 2) {
        users[parts[i + 1]] = { username: parts[i] };
      }

      this.store.dispatch(joinedRoom(key, users));
    } else if (msg[0] === 'j') {
      const parts = msg.split(',');
      const username = parts[0].slice(1);
      const userID = parts[1];

      this.store.dispatch(playerJoined(username, userID));

    } else if (msg[0] === 's') {
      this.store.dispatch(gameStart());
    } else if (msg.slice(0, 2) === 'bn') {
      const playerID = msg.slice(2);
      this.store.dispatch(playerID);
    } else if (msg[0] === 'h') {
      //`h({card_number}{card_suit}(,{card_number}{card_suit})*)?`
      const cards = msg.slice(1).split(',');
      const hand = [];
      for (var card of cards) {
        hand.push({ number: card[0], suit: card[1] });
      }
      this.store.dispatch(gameReceiveHand(hand));
    } else {
      console.error(`Unknown websocket message '${msg}'`);
    }
  }

  createRoom(username) {
    this.socket.send('c' + username);
  }
}
