import React from 'react';

import Card from './Card';

import './Hand.css';

export default function Hand({ cards, onSelect, disabledCards = [] }) {
  return (
    <div className="hand">
      {cards.map((card, i) => (
        <Card
          suit={card.suit}
          number={card.number}
          key={card.number + card.suit}
          disabled={disabledCards.indexOf(i) !== -1}
          onSelect={onSelect? onSelect.bind(null, i) : undefined}
          className={card.className}
        />
      ))}
    </div>
  );
}
