import React, { useState, useCallback } from 'react';
import { useSelector } from 'react-redux';
import './Landing.css';

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

  let room = username? "room show" : "room hidden";
  
  return (
    <div className="landing-wrapper">
      <div className="landing">
        <h2>Enter Username:</h2>
        <input
          placeholder="Username"
          type="text"
          value={username}
          onChange={event =>  setUsername(event.target.value)}
          maxLength="15"
        />
        <div className={room}>
          <hr></hr>
          <h2>Join Room:</h2> 
          <form className="join-room"onSubmit={joinRoom}>
            <input
              placeholder="Room Key"
              type="text"
              value={roomCode}
              onChange={event => setRoomCode(event.target.value)}
              maxLength="5"
            />
            <input type="submit" value="Join" />
          </form>

          <h3 className="or-hr">or</h3>
          <form className="create-room" onSubmit={createRoom}>
            <input 
              type="submit" 
              value="Create Room"
            />
          </form>  
        </div>
      </div>
    </div>
  );
}