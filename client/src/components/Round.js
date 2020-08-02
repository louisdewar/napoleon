import React, { useMemo } from 'react';

import Hand from './Hand';
import RoundInfo from './RoundInfo';

import './Round.css';

export default function Round({ userID, game, users, socket }) {
  const disabledCards = useMemo(() => {
    let disabled = [];
    for (let i = 0; i < game.hand.length; i++) {
      if (game.hand[i].suit !== game.requiredSuit) {
        disabled.push(i);
      }
    }

    if (disabled.length === game.hand.length) {
      // Any card is allowed since current player doesn't have the required suit
      return [];
    } else {
      return disabled;
    }
  }, [game.hand, game.requiredSuit]);

  const cardsPlayed = game.cardsPlayed.map(card => {
    return {
      suit: card.suit,
      number: card.number,
      className: card.playerID === game.winner ? 'winner' : '',
      descriptionA: `${users[card.playerID].username} (${game.trickCount[
        card.playerID
      ] || 0})`,
      descriptionB: game.napoleonID === card.playerID ? 'Napoleon' : '',
      playerID: card.playerID,
    };
  });

  const roundInfo = (
    <RoundInfo
      napoleonUsername={users[game.napoleonID].username}
      napeolonBid={game.napoleonBid}
      trumpSuit={game.trumpSuit}
      allyCards={game.allies}
    />
  );

  const playingArea = (
    <>
      <p>Playing area:</p>
      <Hand cards={cardsPlayed} />
    </>
  );

  const winnerCardsPlayed = (
    <>
      <p>The cards played in this round:</p>
      <Hand cards={cardsPlayed} />
    </>
  );

  let allyMessage = game.ally ? <h2 className="allyMessage">You are a secret ally of the Napoleon</h2> : null


  if (userID === game.playerID) {
    const onClick = id =>
      socket.playCard(game.hand[id].number, game.hand[id].suit);

    if (game.winner) {
      return (
        <>
          {roundInfo}
          {allyMessage}
          <p>Your hand:</p>
          <Hand cards={game.hand} />
          <p>{users[game.winner].username} won the round!</p>
          {winnerCardsPlayed}
        </>
      );
    }

    return (
      <>
        {roundInfo}
        {allyMessage}
        <p>Choose a card to play:</p>
        <Hand
          cards={game.hand}
          disabledCards={disabledCards}
          onSelect={onClick}
        />
        {playingArea}
      </>
    );
  } else {
    return (
      <>
        {roundInfo}
        {allyMessage}
        <p>Your hand:</p>
        <Hand cards={game.hand} />
        <p>{users[game.playerID].username} is currently picking a card</p>
        {playingArea}
      </>
    );
  }
}

