import { websocketConnect, joinedRoom, playerJoined, gameStart, gameReceiveHand, gamePlayerBid, gameNoBids, gameBiddingOver, gameAlliesChosen } from './action';

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
      const cards = msg.slice(1).split(',');
      const hand = [];
      for (var card of cards) {
        hand.push({ number: card[0], suit: card[1] });
      }
      this.store.dispatch(gameReceiveHand(hand));
    } else if (msg.slice(0, 2) === 'bp'){
      const playerBid = msg.slice(1).split(',');
      const playerID = playerBid[0];
      if (playerBid.length === 2){
        const bid = playerBid[1];
        this.store.dispatch(gamePlayerBid(playerID, bid));
      } else{
        this.store.dispatch(gamePlayerBid(playerID));
      }
    } else if (msg.slice(0, 2) === 'nb') {
      this.store.dispatch(gameNoBids());
    } else if (msg.slice(0,2) === 'bo') {
      const parts = msg.slice(2).split(',');
      const bid = parts[0];
      const napoleonID = parts[1];
      this.store.dispatch(gameBiddingOver(bid, napoleonID));
    } else if (msg.slice(0, 2) === 'ac'){
      const parts = msg.slice(2).split(',');
      const trumpSuit = parts[0];
      const allies = [];
      for (i = 1; i < parts.length; i++){
        allies.push(parts[i]);
      }
      this.store.dispatch(gameAlliesChosen(trumpSuit, allies));
    } else {
      console.error(`Unknown websocket message '${msg}'`);
    }
  }

  createRoom(username) {
    this.socket.send('c' + username);
  }
}
