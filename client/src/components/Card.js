import React from 'react';
import classnames from 'classnames';

import Heart from '../assets/images/heart.png';
import Diamond from '../assets/images/diamond.png';
import Club from '../assets/images/club.png';
import Spade from '../assets/images/spade.png';

import './Card.css';

export function suitToImage(suit) {
  switch (suit) {
  case 'H':
    return Heart;
  case 'D':
    return Diamond;
  case 'C':
    return Club;
  case 'S':
    return Spade;
  default:
    console.error(`Suit ${suit} was invalid!`);
    return null;
  }
}

export function MiniCard({ suit, number }) {
  number = number === 'T' ? '10' : number;
  const image = suitToImage(suit);

  return (
    <div
      className={classnames('miniCard', { red: suit === 'H' || suit === 'D' })}
    >
      <span className="number">{number}</span>
      <img className="suit" src={image} alt="" />
    </div>
  );
}

export default function Card({
  suit,
  number,
  disabled,
  onSelect,
  className,
  descriptionA,
  descriptionB,
}) {
  number = number === 'T' ? '10' : number;
  const image = suitToImage(suit);

  return (
    <div className="cardWrapper">
      <div
        className={classnames(
          'card',
          {
            red: suit === 'H' || suit === 'D',
            clickable: onSelect && !disabled,
            disabled,
            placeholder: !suit && !number,
          },
          className
        )}
        onClick={!disabled ? onSelect : undefined}
      >
        <span className="number">{number}</span>
        <img className="suitUpper" src={image} alt="" />
        <img className="suitLower" src={image} alt="" />
      </div>
      <label className="descriptionA">{descriptionA}</label>
      <label className="descriptionB">{descriptionB}</label>
    </div>
  );
}
