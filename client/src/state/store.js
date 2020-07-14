import { createStore } from 'redux';

function pushItem(array, item) {
  return [...array, item];
}

function gameReducer(state = {}, action) {
  let newState = { ...state };

  switch(action.type) {
  case 'GAME_START':
    return { game_state: 'bidding' };
  case 'GAME_RECEIVE_HAND':
    newState.hand = action.cards;
    return newState;
  case 'GAME_NEXT_BIDDER':
    newState.bidder = action.playerID;
    return newState;
  case 'GAME_PLAYER_BID':
    newState.bid = action.bid; // we don't really need to know the id of the player who bid, do we?.
    return newState;
  case 'GAME_NO_BIDS':
    return { game_state: 'noBids' };
  case 'GAME_BIDDING_OVER':
    newState.napoleon = {napoleonBid: action.bid, napoleonID: action.napoleonID};
    newState.game_state = 'biddingOver';
    return newState;
  case 'GAME_ALLIES_CHOSEN':
    newState.trumpSuit = action.trumpSuit;
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
      userID: action.userID
    };
  case 'JOINED_ROOM':
    newState.room = { key: action.key, users: action.users };
    return newState;
  case 'PLAYER_JOIN': {
    const newRoom = { ...newState.room };
    newState.room = newRoom;

    const newUsers = { ...newRoom.users };
    newUsers[action.userID] = {
      username: action.username
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
