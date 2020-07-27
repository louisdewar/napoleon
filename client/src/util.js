import Heart from './assets/images/heart.png';
import Diamond from './assets/images/diamond.png';
import Club from './assets/images/club.png';
import Spade from './assets/images/spade.png';

export const numbers = [
  '2',
  '3',
  '4',
  '5',
  '6',
  '7',
  '8',
  '9',
  'T',
  'J',
  'Q',
  'K',
  'A',
];

export const suits = ['H', 'D', 'C', 'S'];

export const numbersMap = {
  '2': '2',
  '3': '3',
  '4': '4',
  '5': '5',
  '6': '6',
  '7': '7',
  '8': '8',
  '9': '9',
  T: '10',
  J: 'Jack',
  Q: 'Queen',
  K: 'King',
  A: 'Ace',
};

export const suitsMap = {
  H: 'Hearts',
  D: 'Diamonds',
  C: 'Clubs',
  S: 'Spades',
};

export const suitsMapImage = {
  'Hearts': Heart,
  'Diamonds': Diamond,
  'Clubs': Club,
  'Spades': Spade,
}