import { websocket_connect, joined_room } from './action';

export default class WebsocketManager {
  constructor(host, store) {
    this.socket = new WebSocket(host);
    this.socket.onmessage = this.on_message.bind(this);
    this.store = store;
  }

  on_message(event) {
    const msg = event.data;

    if (msg[0] === 'c') {
      const id = msg.slice(1);
      this.store.dispatch(websocket_connect(this, id));
    } else if (msg[0] === 'e') {
      const key = msg.slice(1);
      this.store.dispatch(joined_room(key));
    } else {
      console.error(`Unknown websocket message '${msg}'`);
    }
  }

  createRoom(username) {
    this.socket.send('c' + username);
  }
}
