# Catalyst

<p align="center">
  <img src="./assets/Catalyst.png" alt="Catalyst" width="50%">
</p>

## Overview

The Catalyst smart contract is a Rust-based contract designed to work as a fast bridging solution between blockchains outside of the Cosmos ecosystem, but utilizing Osmosis as the intermediary.

At a high level, this contract acts as a pseudo order book, where bridge transactions from an external blockchain (such as the Bitcoin network) are listed prior to receiving sufficient confirmations. If a bridge transaction is posted to this contract, a market maker is able to fill the order by sending the appropriate amount of tokens to the contract, which in turn immediately get forwarded to the desired end location. As a reward for taking on the risk of block reorgs, the market maker is able to claim a portion of the amount being bridged.

If the transaction stays posted to Catalyst beyond the time the Osmosis protocol deems as safe (usually 6 confirmations on the Bitcoin network), the transaction is removed from the order book and continues on it's normal flow.
