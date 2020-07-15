import React, { useState, useCallback, useMemo } from 'react';
import { numbers, suits, numbersMap, suitsMap } from '../util';
import Hand from './Hand';

function CardPicker({ id, onSelection }) {
  const [selectedNumber, setSelectedNumber] = useState(null);
  const [selectedSuit, setSelectedSuit] = useState(null);

  const onNumberChange = useCallback(
    e => {
      if (selectedSuit !== null) {
        onSelection(id, e.target.value, selectedSuit);
      }

      setSelectedNumber(e.target.value);
    },
    [id, setSelectedNumber, selectedSuit, onSelection]
  );
  const onSuitChange = useCallback(
    e => {
      if (selectedNumber !== null) {
        onSelection(id, selectedNumber, e.target.value);
      }

      setSelectedSuit(e.target.value);
    },
    [id, setSelectedSuit, selectedNumber, onSelection]
  );

  return (
    <>
      {numbers.map(number => (
        <React.Fragment key={number}>
          <input
            type="radio"
            name="number"
            id={'number_' + number}
            value={number}
            onChange={onNumberChange}
            checked={selectedNumber === number}
          />
          <label htmlFor={'number_' + number}>{numbersMap[number]}</label>
        </React.Fragment>
      ))}
      <br />
      {suits.map(suit => (
        <React.Fragment key={suit}>
          <input
            type="radio"
            name="suit"
            id={'suit_' + suit}
            value={suit}
            onChange={onSuitChange}
            checked={selectedSuit === suit}
          />
          <label htmlFor={'suit_' + suit}>{suitsMap[suit]}</label>
        </React.Fragment>
      ))}
    </>
  );
}

export default function PostBidding({ game, userID, socket, users }) {
  const [allyCards, setAllyCards] = useState(
    new Array(game.settings.ally_cards).fill(null)
  );

  const [trumpSuit, setTrumpSuit] = useState(null);

  const onCardSelection = useCallback(
    (id, number, suit) => {
      setAllyCards(cards => {
        const newCards = [...cards];
        newCards[id] = { number, suit };
        return newCards;
      });
    },
    [setAllyCards]
  );

  const onSubmit = useCallback(
    e => {
      e.preventDefault();

      // At least one ally card is not yet selected
      if (allyCards.indexOf(null) !== -1) {
        console.error('At least one ally card wasn\'t selected');
        return;
      }

      if (trumpSuit === null) {
        console.error('Trump suit wasn\'t selected');
        return;
      }

      socket.pickAllies(trumpSuit, allyCards);
    },
    [allyCards, trumpSuit, socket]
  );

  const allowedTrumpSuits = useMemo(() => {
    let allowed = [];

    for (var card of game.hand) {
      if (allowed.indexOf(card.suit) === -1) {
        allowed.push(card.suit);
      }
    }

    return allowed;
  }, [game.hand]);

  if (game.napoleonID === userID) {
    return (
      <form onSubmit={onSubmit}>
        <p>Please choose {game.settings.ally_cards} cards as your allies</p>
        {[...Array(game.settings.ally_cards)].map((_, i) => (
          <div key={i}>
            <p>Choose ally {i + 1}</p>
            <CardPicker onSelection={onCardSelection} id={i} />
          </div>
        ))}

        <p>Please choose your trump suit:</p>
        {allowedTrumpSuits.map(suit => (
          <React.Fragment key={suit}>
            <input
              type="radio"
              name="trumpsuit"
              id={'trumpsuit_' + suit}
              value={suit}
              onChange={e => setTrumpSuit(e.target.value)}
              checked={trumpSuit === suit}
            />
            <label htmlFor={'trumpsuit_' + suit}>{suitsMap[suit]}</label>
          </React.Fragment>
        ))}

        <br />

        <input type="submit" value="Submit" />

        <Hand cards={game.hand} />
      </form>
    );
  } else {
    return (
      <>
        <p>
          {users[game.napoleonID].username} is currently picking their allies
          and trump suit...
        </p>
        <Hand cards={game.hand} />
      </>
    );
  }
}
