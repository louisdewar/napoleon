import React from 'react';
import ReactDOM from 'react-dom';
import App from './App';
import * as serviceWorker from './serviceWorker';

import { Provider } from 'react-redux';
import store from './state/store';

import WebsocketManager from './state/websocket_manager';

import './fonts.css';
import './index.css';

let wsProtocol = window.location.protocol === 'https:'? 'wss://' : 'ws://';

if (process.env.NODE_ENV === 'production') {
  new WebsocketManager(wsProtocol + window.location.hostname + '/' + (process.env.PUBLIC_URL || '') + '/ws/', store);
} else {
  new WebsocketManager(wsProtocol + 'localhost:3001/ws/', store);
}

ReactDOM.render(
  <Provider store={store}>
    <React.StrictMode>
      <App />
    </React.StrictMode>
  </Provider>,
  document.getElementById('root')
);

// If you want your app to work offline and load faster, you can change
// unregister() to register() below. Note this comes with some pitfalls.
// Learn more about service workers: https://bit.ly/CRA-PWA
serviceWorker.unregister();