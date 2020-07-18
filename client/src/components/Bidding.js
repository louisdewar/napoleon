import React, { useState, useCallback } from 'react';

import Hand from './Hand';

export default function Bidding({ game, userID, users, socket }) {
  const [bidAmount, setBidAmount] = useState(0);
  const minimumBid = game.lastBid || 0;
  const maximumBid = game.hand.length;

  const submitBid = useCallback(
    e => {
      e.preventDefault();

      if (bidAmount < minimumBid || bidAmount > maximumBid) {
        return;
      }

      socket.bid(bidAmount);
    },
    [socket, bidAmount, minimumBid, maximumBid]
  );
  const skipBid = useCallback(
    e => {
      e.preventDefault();

      socket.bid();
    },
    [socket]
  );

  if (game.bidder === userID) {
    return (
      <>
        <form onSubmit={submitBid}>
          <p>
            Choose a number to bid, it must between {minimumBid} and{' '}
            {maximumBid} inclusive:
          </p>
          <input
            type="number"
            value={bidAmount}
            onChange={e => setBidAmount(e.target.value)}
            min={minimumBid}
            max={maximumBid}
            step="1"
          />
          <input type="submit" value="Submit bid" />
        </form>
        <form onSubmit={skipBid}>
          <input type="submit" value="Skip my bid" />
        </form>
        <Hand cards={game.hand} />
      </>
    );
  } else {
    return (
      <>
        <p>
          {users[game.bidder].username} is bidding please wait for them to
          decide
        </p>
        <Hand cards={game.hand} />
      </>
    );
  }
}
