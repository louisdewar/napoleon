import React from 'react';

import Game from './Game';
import Lobby from './Lobby';

export default function Room({ userID, room, socket }) {
  if (room.game) {
    return <Game socket={socket} game={room.game} />;
  } else {
    return <Lobby socket={socket} room={room} userID={userID} />;
  }
}
