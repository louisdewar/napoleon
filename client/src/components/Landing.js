import React, { useState, useCallback } from 'react';
import { useSelector } from 'react-redux';

export default function Landing() {
  const [roomCode, setRoomCode] = useState('');
  const [username, setUsername] = useState('');

  const websocket = useSelector(state => state.socket);
  const joinRoom = useCallback(
    e => {
      e.preventDefault();
      websocket.joinRoom(username, roomCode);
    },
    [websocket, roomCode, username]
  );

  const createRoom = useCallback(
    e => {
      e.preventDefault();
      websocket.createRoom(username);
    },
    [websocket, username]
  );

  return (
    <div className="landing">
      <input
        placeholder="Username"
        type="text"
        value={username}
        onChange={event => setUsername(event.target.value)}
      />
      <form onSubmit={joinRoom}>
        <input
          placeholder="Room Key"
          type="text"
          value={roomCode}
          onChange={event => setRoomCode(event.target.value)}
        />
        <input type="submit" value="Join Room" />
      </form>

      <form onSubmit={createRoom}>
        <input type="submit" value="Create Room" />
      </form>
    </div>
  );
}
