import React from 'react';
import classnames from 'classnames'; 

import Heart from '../assets/images/heart.png';
import Diamond from '../assets/images/diamond.png';
import Club from '../assets/images/club.png';
import Spade from '../assets/images/spade.png';

import './Card.css';

export default function Card({ suit, number, disabled, onSelect, className }) {
  number = number === 'T'? '10' : number;
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

  return (
    <div 
        className={classnames('card', { red: suit === 'H' || suit === 'D', clickable: onSelect && !disabled, disabled }, className)}
        onClick={!disabled? onSelect : undefined} >
      <span className="number">{number}</span>
      <img className="suitUpper" src={image} alt="" />
      <img className="suitLower" src={image} alt="" />
    </div>
  );
}