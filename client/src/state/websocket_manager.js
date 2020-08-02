import {
  websocketConnect,
  joinedRoom,
  playerJoined,
  gameStart,
  gameReceiveHand,
  gamePlayerBid,
  gameNoBids,
  gameBiddingOver,
  gameAlliesChosen,
  gameNextPlayer,
  gameNextBidder,
  gameCardPlayed,
  gameRoundOver,
  gameOver,
  gameBecomeAlly,
} from './action';

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
      const host = parts[1];

      const users = {};
      for (var i = 2; i + 1 < parts.length; i += 2) {
        users[parts[i + 1]] = { username: parts[i] };
      }

      this.store.dispatch(joinedRoom(key, host, users));
    } else if (msg[0] === 'j') {
      const parts = msg.split(',');
      const username = parts[0].slice(1);
      const userID = parts[1];

      this.store.dispatch(playerJoined(username, userID));
    } else if (msg[0] === 's') {
      const lines = msg.slice(1).split('\n');
      const playerOrder = lines[0].split(',');
      const settings = JSON.parse(lines[1]);
      this.store.dispatch(gameStart(playerOrder, settings));
    } else if (msg.slice(0, 2) === 'bn') {
      const playerID = msg.slice(2);
      this.store.dispatch(gameNextBidder(playerID));
    } else if (msg[0] === 'h') {
      const cards = msg.slice(1).split(',');
      const hand = [];
      for (var card of cards) {
        hand.push({ number: card[0], suit: card[1] });
      }
      this.store.dispatch(gameReceiveHand(hand));
    } else if (msg.slice(0, 2) === 'bp') {
      const playerBid = msg.slice(1).split(',');
      const playerID = playerBid[0];
      if (playerBid.length === 2) {
        const bid = parseInt(playerBid[1], 10);
        this.store.dispatch(gamePlayerBid(playerID, bid));
      } else {
        this.store.dispatch(gamePlayerBid(playerID));
      }
    } else if (msg.slice(0, 2) === 'nb') {
      this.store.dispatch(gameNoBids());
    } else if (msg.slice(0, 2) === 'bo') {
      const parts = msg.slice(2).split(',');
      const bid = parts[0];
      const napoleonID = parts[1];
      this.store.dispatch(gameBiddingOver(bid, napoleonID));
    } else if (msg.slice(0, 2) === 'ac') {
      const parts = msg.slice(2).split(',');
      const trumpSuit = parts[0];
      const allies = [];
      for (i = 1; i < parts.length; i++) {
        allies.push({ number: parts[i][0], suit: parts[i][1] });
      }
      this.store.dispatch(gameAlliesChosen(trumpSuit, allies));
    } else if (msg.slice(0, 2) === 'ab') {
      this.store.dispatch(gameBecomeAlly());
    } else if (msg[0] === 'n') {
      const parts = msg.slice(1).split(',');
      const playerID = parts[0];
      let requiredSuit = null;

      if (parts.length === 2) {
        requiredSuit = parts[1];
      }

      this.store.dispatch(gameNextPlayer(playerID, requiredSuit));
    } else if (msg[0] === 'p') {
      const parts = msg.slice(1).split(',');
      const playerID = parts[0];
      const card = { number: parts[1][0], suit: parts[1][1] };
      this.store.dispatch(gameCardPlayed(playerID, card));
    } else if (msg[0] === 'r') {
      const winnerPlayerID = msg.slice(1);
      this.store.dispatch(gameRoundOver(winnerPlayerID));
    } else if (msg[0] === 'g') {
      const parts = msg.slice(1).split(',');
      const [
        napoleonScoreDelta,
        playerScoreDelta,
        napoleonBid,
        combinedNapoleonScore,
      ] = [parts[0], parts[1], parts[2], parts[3]];
      const allies = [];
      if (parts.length > 4) {
        for (i = 4; i < parts.length; i++) {
          allies.push(parts[i]);
        }
      }
      this.store.dispatch(
        gameOver(
          napoleonScoreDelta,
          playerScoreDelta,
          napoleonBid,
          combinedNapoleonScore,
          allies
        )
      );
    } else {
      console.error(`Unknown websocket message '${msg}'`);
    }
  }

  createRoom(username) {
    this.socket.send('c' + username);
  }

  joinRoom(username, roomKey) {
    this.socket.send('j' + username + ',' + roomKey);
  }

  startGame() {
    this.socket.send('s');
  }

  bid(bid) {
    if (bid !== undefined) {
      this.socket.send('b' + bid);
    } else {
      this.socket.send('b');
    }
  }

  pickAllies(trumpSuit, allyCards) {
    const allies = allyCards.map(card => card.number + card.suit).join(',');
    this.socket.send('a' + trumpSuit + ',' + allies);
  }

  playCard(number, suit) {
    this.socket.send('p' + number + suit);
  }
}

