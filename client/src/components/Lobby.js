import React from 'react';

export default function Lobby({ userID, room, socket }) {
  const host = room.host;

  let content;
  if (userID === host) {
    content = (
      <>
        <p>You are the host, you can start the game whenever you want.</p>
        <button onClick={() => socket.startGame()}>Start game</button>
      </>
    );
  } else {
    content = (
      <p>
        {room.users[host].username} is the current host, we are waiting for them
        to start the game
      </p>
    );
  }

  return (
    <div className="lobby">
      <h1>In lobby of room {room.key}</h1>
      {content}
    </div>
  );
}
