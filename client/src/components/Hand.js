import React from 'react';

import { numbersMap, suitsMap } from '../util';

function Card({ number, suit, disabled, onSelect }) {
  return (
    <div
      className="card"
      disabled={disabled}
      onClick={() => onSelect(number, suit)}
    >
      {numbersMap[number]} of {suitsMap[suit]}
      {disabled ? ' (disabled)' : ''}
    </div>
  );
}

export default function Hand({ cards, onSelect, disabledCards = [] }) {
  console.log(disabledCards, cards);
  return (
    <div className="hand">
      {cards.map((card, i) => (
        <Card
          key={card.number + card.suit}
          number={card.number}
          suit={card.suit}
          disabled={disabledCards.indexOf(i) !== -1}
          onSelect={onSelect || (() => {})}
        />
      ))}
    </div>
  );
}
