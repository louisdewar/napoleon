import React from 'react';
import { useCallback } from 'react';
import { useSelector } from 'react-redux';

function App() {
  const is_connected = useSelector(state => state.connected);
  const user_id = useSelector(state => state.userID);
  const room = useSelector(state => state.room);
  const websocket = useSelector(state => state.socket);

  const create_room = useCallback(() => {
    websocket.createRoom('TEMP_USER_' + user_id);
  }, [websocket, user_id]);

  if (is_connected) {
    if (!room) {
      return (
        <div className="App">
          Connected our id is {user_id}.
          <button onClick={create_room}>Create room</button>
        </div>
      );
    } else {
      return (
        <div className="App">
          Connected our id is {user_id} and we are in room {room.key}.
          There are {Object.keys(room.users).length} users connected.
        </div>
      );
    }
  } else {
    return (
      <div className="App">
        Not connected.
      </div>
    );
  }
}

export default App;
