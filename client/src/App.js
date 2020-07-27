import React from 'react';
import { useSelector } from 'react-redux';
import Landing from './components/Landing';
import Room from './components/Room';

function App() {
  const room = useSelector(state => state.room);
  const userID = useSelector(state => state.userID);
  const socket = useSelector(state => state.socket);
  if (room) {
    return <Room room={room} userID={userID} socket={socket} />;
  } else {
    return <Landing />;
  }
}

export default App;