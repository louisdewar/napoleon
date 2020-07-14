import { createStore } from 'redux';

function pushItem(array, item) {
  return [...array, item];
}

function gameReducer(state = {}, action, rootState) {
  let newState = { ...state };

  switch (action.type) {
  case 'GAME_START':
    return {
      game_state: 'BIDDING',
      playerOrder: action.playerOrder,
      bids: {},
      cardsPlayed: {},
    }; // check "cardsPlayed" decisition
  case 'GAME_RECEIVE_HAND':
    newState.hand = action.cards;
    return newState;
  case 'GAME_NEXT_BIDDER':
    newState.bidder = action.playerID;
    return newState;
  case 'GAME_PLAYER_BID':
    newState.bids[action.playerID] = action.bid;
    return newState;
  case 'GAME_NO_BIDS':
    return { game_state: 'NO_BIDS' };
  case 'GAME_BIDDING_OVER':
    newState.napoleon = {
      napoleonBid: action.bid,
      napoleonID: action.napoleonID,
    };
    newState.game_state = 'BIDDING_OVER';
    return newState;
  case 'GAME_ALLIES_CHOSEN':
    newState.trumpSuit = action.trumpSuit;
    newState.allies = action.allies;
    return newState;
  case 'GAME_NEXT_PLAYER':
    newState.playerID = action.playerID;
    return newState;
  case 'GAME_CARD_PLAYED': {
    if (rootState.userID === action.playerID) {
      const newHand = [];
      for (var i = 0; i < newState.hand.length; i++) {
        if (
          newState.hand[i].number !== action.card.number ||
            newState.hand[i].suit !== action.card.suit
        ) {
          newHand.push({
            number: newState.hand[i].number,
            suit: newState.hand[i].suit,
          });
        }
      }
      newState.hand = newHand;
    }
    const newCardsPlayed = { ...newState.cardsPlayed };
    newCardsPlayed[action.playerID] = action.card;
    newState.cardsPlayed = newCardsPlayed;
    return newState;
  }
  case 'GAME_ROUND_OVER':
    newState.winner = action.winnerPlayerID;
    return newState;
  case 'GAME_OVER':
    newState.napoleonScoreDelta = action.napoleonScoreDelta;
    newState.playerScoreDelta = action.playerScoreDelta;
    newState.napoleonBet = action.napoleonBet;
    newState.combinedNapoleonScore = action.combinedNapoleonScore;
    newState.allies = action.allies;
    return newState;
  default:
    return state;
  }
}

function mainReducer(state = {}, action) {
  let newState = { ...state };
  switch (action.type) {
  case 'WEBSOCKET_CONNECT':
    return {
      connected: true,
      socket: action.socket,
      userID: action.userID,
    };
  case 'JOINED_ROOM':
    newState.room = {
      key: action.key,
      users: action.users,
      host: action.host,
    };
    return newState;
  case 'PLAYER_JOIN': {
    const newRoom = { ...newState.room };
    newState.room = newRoom;

    const newUsers = { ...newRoom.users };
    newUsers[action.userID] = {
      username: action.username,
    };
    newState.room.users = newUsers;
    return newState;
  }
  default:
    if (action.type.startsWith('GAME_')) {
      if (state.room) {
        newState.game = gameReducer(state.room.game, action);
      }
    }

    return state;
  }
}

export default createStore(mainReducer);
