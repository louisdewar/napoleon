import React from 'react';

import './RoundInfo.css';

import { MiniCard, suitToImage } from './Card';

export default function RoundInfo({
  allyCards,
  trumpSuit,
  napoleonUsername,
  napeolonBid,
}) {
  return (
    <table className="roundInfo">
      <tr>
        <th>Napoleon</th>
        <th>Bid</th>
        <th>Trump</th>
        {allyCards ? <th>Ally cards</th> : null}
      </tr>
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
              <MiniCard suit={card.suit} number={card.number} />
            ))
            : null}
        </td>
      </tr>
    </table>
  );
}
