import { createStore } from 'redux';

function gameReducer(state = {}, action, rootState) {
  let newState = { ...state };

  switch (action.type) {
  case 'GAME_START':
    return {
      gameState: 'START',
      playerOrder: action.playerOrder,
      settings: action.settings,
      bids: {},
      cardsPlayed: {},
    };
  case 'GAME_RECEIVE_HAND':
    newState.hand = action.hand;
    return newState;
  case 'GAME_NEXT_BIDDER':
    newState.gameState = 'BIDDING';
    newState.bidder = action.playerID;
    return newState;
  case 'GAME_PLAYER_BID':
    newState.bids[action.playerID] = action.bid;
    newState.lastBid = action.bid;
    return newState;
  case 'GAME_NO_BIDS':
    return { gameState: 'NO_BIDS' };
  case 'GAME_BIDDING_OVER':
    newState.napoleonID = action.napoleonID;
    newState.napoleonBid = action.bid;
    newState.gameState = 'POST_BIDDING';
    return newState;
  case 'GAME_ALLIES_CHOSEN':
    newState.trumpSuit = action.trumpSuit;
    newState.allies = action.allies;
    return newState;
  case 'GAME_NEXT_PLAYER':
    newState.gameState = 'ROUND';
    newState.playerID = action.playerID;
    newState.requiredSuit = action.requiredSuit;
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
  case 'GAME_BECOME_ALLY':
    newState.ally = true;
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
        newState.room = { ...newState.room };
        newState.room.game = gameReducer(state.room.game, action);
      }
    }

    return newState;
  }
}

export default createStore(mainReducer);
