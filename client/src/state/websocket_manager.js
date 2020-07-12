import { websocketConnect, joinedRoom } from './action';

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
    } else {
      console.error(`Unknown websocket message '${msg}'`);
    }
  }

  createRoom(username) {
    this.socket.send('c' + username);
  }
}
