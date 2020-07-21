import React from 'react';

import './Lobby.css';

export default function Lobby({ userID, room, socket }) {
  const host = room.host;

  let content;
  if (userID === host) {
    content = (
      <>
        <p className="host-p">You are the host, you can start the game whenever you want.</p>
        <button className="start-btn" onClick={() => socket.startGame()}>Start game</button>
      </>
    );
  } else {
    content = (
      <p className="user-p">
        {room.users[host].username} is the current host, we are waiting for them
        to start the game
      </p>
    );
  }

  return (
    <div className="lobby-wrapper">
      <div className="lobby">
        <h1>In lobby of room <span className="room-key">{room.key}</span></h1>
        {content}
      </div>
    </div>
  );
}
