import React, { useState, useCallback, useMemo } from 'react';
import { suitsMap, suitsMapImage } from '../util';
import Hand from './Hand';
import CardPick from './CardPick'; 

import './PostBidding.css';


export default function PostBidding({ game, userID, socket, users }) {
  const [allyCards, setAllyCards] = useState(
    new Array(game.settings.ally_cards).fill({ number: 'A', suit: 'H' })
  );

  const [trumpSuit, setTrumpSuit] = useState(null);

  const setAllyCardSuit = useCallback(
    (id, suit) => {
      setAllyCards(cards => {
        const newCards = [...cards];
        newCards[id] = {number: newCards[id].number, suit: suit(cards[id].suit)};
        return newCards;
      })
    }, [setAllyCards]);

  const setAllyCardNumber = useCallback(
    (id, number) => {
      setAllyCards(cards => {
        const newCards = [...cards];
        newCards[id] = {number: number(cards[id].number), suit: newCards[id].suit};
        return newCards;
      })
    }, [])

  const onSubmit = useCallback(
    e => {
      e.preventDefault();

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
      <div className="postbidding-wrapper">
        <div className="postbidding">
          <form onSubmit={onSubmit}>
            <p>Please choose {game.settings.ally_cards} cards as your allies</p>
            {allyCards.map((card, i) => (
              <div key={i}>
                <p>Choose ally {i + 1}</p>
                <CardPick 
                  suit={card.suit}
                  number={card.number}
                  setSuit={setAllyCardSuit.bind(null, i)}
                  setNumber={setAllyCardNumber.bind(null, i)} />
              </div>
            ))}

            <p>Please choose your trump suit:</p>
            {allowedTrumpSuits.map(suit => (
              <div className="trump-suit-wrapper" key={suit}>
                <img
                  className={trumpSuit === suit? "active" : null}
                  src={suitsMapImage[suitsMap[suit]]}
                  onClick={e => setTrumpSuit(suit)}
                  alt=""
                  />
              </div>
            ))}

            <div className="submit-div">
              <input type="submit" value="Submit" />
            </div>
            <Hand cards={game.hand} />

          </form>
        </div>
      </div>
    );
  } else {
    return (
      <div className="postbidding-wrapper">
        <div className="postbidding">  
          <p>
            {users[game.napoleonID].username} is currently picking their allies
            and trump suit...
          </p>
          <Hand classname="hand" cards={game.hand} />
        </div>
      </div>
    );
  }
}
