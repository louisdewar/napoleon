import React from 'react';

import Game from './Game';
import Lobby from './Lobby';

export default function Room({ userID, room, socket }) {
  let content;
  if (room.game) {
    content = (
      <Game
        socket={socket}
        game={room.game}
        userID={userID}
        users={room.users}
      />
    );
  } else {
    content = <Lobby socket={socket} room={room} userID={userID} />;
  }

  return (
    <>
      {content}
      {/*
      <br />
       {JSON.stringify(room)} */}
    </>
  );
}
