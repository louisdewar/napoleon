import { createStore } from 'redux';

function initCardsPlayed(playerOrder, startPlayer) {
  const startIndex = playerOrder.indexOf(startPlayer);
  const playedCards = [];

  for (let i = 0; i < playerOrder.length; i++) {
    playedCards.push({
      playerID: playerOrder[(startIndex + i) % playerOrder.length],
    });
  }

  return playedCards;
}

function gameReducer(state = {}, action, rootState) {
  let newState = { ...state };

  switch (action.type) {
  case 'GAME_START':
    return {
      gameState: 'START',
      playerOrder: action.playerOrder,
      settings: action.settings,
      bids: {},
      cardsPlayed: initCardsPlayed(action.playerOrder, action.playerOrder[0]),
      trickCount: {},
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
    newState.cardsPlayed = null;
    return newState;
  case 'GAME_NEXT_PLAYER':
    newState.gameState = 'ROUND';
    newState.playerID = action.playerID;
    newState.requiredSuit = action.requiredSuit;

    if (!newState.cardsPlayed || newState.winner) {
      newState.cardsPlayed = initCardsPlayed(
        newState.playerOrder,
        newState.playerID
      );
    }

    if (newState.winner) {
      newState.winner = null;
    }

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

    const newCardsPlayed = [...newState.cardsPlayed];

    for (let i = 0; i < newCardsPlayed.length; i++) {
      if (newCardsPlayed[i].playerID === action.playerID) {
        newCardsPlayed[i] = {
          suit: action.card.suit,
          number: action.card.number,
          playerID: action.playerID,
        };
      }
    }

    newState.cardsPlayed = newCardsPlayed;
    return newState;
  }
  case 'GAME_ROUND_OVER':
    newState.winner = action.winnerPlayerID;
    newState.trickCount = { ...newState.trickCount };
    const winner = newState.trickCount[newState.winner] || 0;
    newState.trickCount[newState.winner] = winner + 1;
    return newState;
  case 'GAME_OVER':
    return {
      gameState: 'GAME_OVER',
      napoleonScoreDelta: action.napoleonScoreDelta,
      playerScoreDelta: action.playerScoreDelta,
      napoleonBid: action.napoleonBid,
      combinedNapoleonScore: action.combinedNapoleonScore,
      allies: action.allies,
    };
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
        newState.room.game = gameReducer(state.room.game, action, newState);
      }
    }

    return newState;
  }
}

export default createStore(
  mainReducer,
  window.__REDUX_DEVTOOLS_EXTENSION__ && window.__REDUX_DEVTOOLS_EXTENSION__()
);
