import React, { useState, useCallback, useEffect } from 'react';

import Hand from './Hand';
import './Bidding.css';

export default function Bidding({ game, userID, users, socket }) {
  const minimumBid = game.lastBid !== undefined? game.lastBid + 1 : 0;
  const maximumBid = game.hand.length;

  const [bidAmount, setBidAmount] = useState(0);

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

  useEffect(() => {
    if (minimumBid > bidAmount){
      setBidAmount(minimumBid);
    }
  }, [minimumBid, bidAmount, setBidAmount]);

  if (game.bidder === userID) {
    return (
      <div className="bidding">
        <form onSubmit={submitBid}>
          <p className="info-p">
            Choose a number to bid, it must between {minimumBid} and{' '}
            {maximumBid} inclusive:
          </p>
          <input
            className='bid-choice'
            type="number"
            value={bidAmount}
            onChange={e => setBidAmount(e.target.value)}
            min={minimumBid}
            max={maximumBid}
            step="1"
          />
          <input type="submit" value="Submit bid" />
        </form>
        <h3 className="or-hr">or</h3>
        <div className="skip">
          <form onSubmit={skipBid}>
            <input type="submit" value="Skip my bid" />
          </form>
        </div>
        <Hand cards={game.hand} />

      </div>
    );
  } else {
    return (
      <div className="bidding-wrapper">
        <div className="bidding">
          <p className="info-p">
            {users[game.bidder].username} is bidding, please wait for them to
            decide...
          </p>
          <Hand cards={game.hand} />
        </div>
      </div>
    );
  }
}
