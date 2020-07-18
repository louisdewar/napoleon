import React, { useMemo } from 'react';

import Hand from './Hand';

export default function Round({ userID, game, users, socket }) {
  const disabledCards = useMemo(() => {
    let disabled = [];
    for (let i = 0; i < game.hand.length; i++) {
      if (game.hand[i].suit !== game.requiredSuit) {
        disabled.push(i);
      }
    }

    console.log('calculating disabled cards', disabled, game.requiredSuit);

    if (disabled.length === game.hand.length) {
      // Any card is allowed since current player doesn't have the required suit
      return [];
    } else {
      return disabled;
    }
  }, [game.hand, game.requiredSuit]);

  const playedCards = (
    <>
      <p>The cards played so far:</p>
      <Hand cards={Object.values(game.cardsPlayed)} />
    </>
  );

  if (userID === game.playerID) {
    const onClick = (number, suit) => socket.playCard(number, suit);
    console.log(disabledCards);
    return (
      <>
        <p>Choose a card to play:</p>
        <Hand
          cards={game.hand}
          disabledCards={disabledCards}
          onSelect={onClick}
        />
        {playedCards}
      </>
    );
  } else {
    return (
      <>
        <p>{users[game.playerID].username} is currently picking a card</p>
        {playedCards}
      </>
    );
  }
}
