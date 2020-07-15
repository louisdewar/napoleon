import React from 'react';

import Bidding from './Bidding';
import PostBidding from './PostBidding';
import Round from './Round';

export default function Game({ socket, game, userID, users }) {
  switch (game.gameState) {
  case 'START':
    return null;
  case 'BIDDING':
    return (
      <Bidding game={game} socket={socket} userID={userID} users={users} />
    );
  case 'POST_BIDDING':
    return (
      <PostBidding
        game={game}
        socket={socket}
        userID={userID}
        users={users}
      />
    );
  case 'ROUND':
    return (
      <Round game={game} socket={socket} userID={userID} users={users} />
    );
  default:
    console.error(`Invalid game state ${game.gameState}`);
    return null;
  }
}
