import React, { useCallback } from 'react';
import classnames from 'classnames';
import { numbers, suits } from '../util';

import Heart from '../assets/images/heart.png';
import Diamond from '../assets/images/diamond.png';
import Club from '../assets/images/club.png';
import Spade from '../assets/images/spade.png';
import leftArrow from '../assets/images/leftArrow.svg';
import rightArrow from '../assets/images/rightArrow.svg';

import './CardPick.css';

export default function CardPicker({ setSuit, setNumber, suit, number }) {
  number = number === 'T' ? '10' : number;
  let image;
  switch (suit) {
  case 'H':
    image = Heart;
    break;

  case 'D':
    image = Diamond;
    break;

  case 'C':
    image = Club;
    break;

  case 'S':
    image = Spade;
    break;

  default:
    console.error(`Suit ${suit} was invalid!`);
  }

  const changeSuit = useCallback(
    direction => {
      if (direction === 'l') {
        setSuit(
          oldSuit =>
            suits[(suits.indexOf(oldSuit) + suits.length - 1) % suits.length]
        );
      } else {
        setSuit(oldSuit => suits[(suits.indexOf(oldSuit) + 1) % suits.length]);
      }
    },
    [setSuit]
  );

  const changeNumber = useCallback(
    direction => {
      if (direction === 'l') {
        setNumber(
          oldNum =>
            numbers[
              (numbers.indexOf(oldNum) + numbers.length - 1) % numbers.length
            ]
        );
      } else {
        setNumber(
          oldNum => numbers[(numbers.indexOf(oldNum) + 1) % numbers.length]
        );
      }
    },
    [setNumber]
  );

  return (
    <div
      className={classnames('cardpick', { red: suit === 'H' || suit === 'D' })}
    >
      <span className="number">{number}</span>
      <img className="suit" src={image} alt="" />
      <span className="arrow-l-suit" onClick={changeSuit.bind(null, 'l')}>
        <img src={leftArrow} width="35px" alt="" />
      </span>
      <span className="arrow-r-suit" onClick={changeSuit.bind(null, 'r')}>
        <img src={rightArrow} width="35px" alt="" />
      </span>
      <span className="arrow-l-number" onClick={changeNumber.bind(null, 'l')}>
        <img src={leftArrow} width="35px" alt="" />
      </span>
      <span className="arrow-r-number" onClick={changeNumber.bind(null, 'r')}>
        <img src={rightArrow} width="35px" alt="" />
      </span>
    </div>
  );
}

