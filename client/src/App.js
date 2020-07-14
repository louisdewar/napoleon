import React from 'react';
import { useSelector } from 'react-redux';
import Landing from 'components/Landing';

function App() {
  const room = useSelector(room);
  if (room) {
    return (<h1>You are in room {room.key}</h1>);
  } else{
    return (<Landing />);
  }
}

export default App;