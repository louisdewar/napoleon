import React from 'react';

import './RoundInfo.css';

import Card, { MiniCard, suitToImage } from './Card';

export default function RoundInfo({
  allyCards,
  trumpSuit,
  napoleonUsername,
  napeolonBid,
}) {
  return (
    <table className="roundInfo">
      <thead>
        <tr>
          <th>Napoleon</th>
          <th>Bid</th>
          <th>Trump</th>
          {allyCards ? <th>Ally cards</th> : null}
        </tr>
      </thead>
      <tbody>
        <tr>
          <td>{napoleonUsername}</td>
          <td>{napeolonBid}</td>
          <td>
            <img
              className="trumpSuit"
              src={suitToImage(trumpSuit)}
              alt={trumpSuit}
            />
          </td>
          <td>
            {allyCards
              ? allyCards.map(card => (
                <MiniCard key={card.suit + card.number} suit={card.suit} number={card.number} />
              ))
              : null}
          </td>
        </tr>
      </tbody>
    </table>
  );
}