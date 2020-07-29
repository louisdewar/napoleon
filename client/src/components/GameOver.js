import React from 'react';

import './GameOver.css';

export default function GameOver({ game, users }) {
  let allies;
  if (game.allies && game.allies.length > 0) {
    allies = (
      <div className="allies">
        The napoleon's
        {game.allies.length === 1 ? ' ally was ' : ' allies were '}
        {game.allies.map(allyID => (
          <span className="ally">{users[allyID].username}</span>
        ))}
      </div>
    );
  }
  return (
    <div className="gameOver">
      <h1>Game over</h1>
      <p>{allies}</p>
      <p>
        The napoleon bid {game.napoleonBid} and the combined score was{' '}
        <span className="score">{game.combinedNapoleonScore}</span> which means that the napoleon and their
        allies
        <span className="score">{game.napoleonScoreDelta < 0
          ? ' lose ' + -1 * game.napoleonScoreDelta
          : ' gain ' + game.napoleonScoreDelta}</span>{' '}
        and the other players
        <span className="score">{game.playerScoreDelta < 0
          ? ' lose ' + -1 * game.playerScoreDelta
          : ' gain ' + game.playerScoreDelta}</span>
        .
      </p>
    </div>
  );
}
