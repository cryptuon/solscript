---
title: "Voting Contract"
description: "A decentralized voting system with proposal creation, vote casting, and result tallying. Demonstrates governance patterns."
tags: ["voting", "governance", "DAO"]
difficulty: intermediate
order: 6
sourceFile: "voting/voting.sol"
---

## Overview

A governance voting contract where users can create proposals, vote for or against them, and tally results. Demonstrates the foundation of on-chain governance.

Key patterns:
- **Proposal management**: Create and track proposals with metadata
- **Vote casting**: One vote per address per proposal
- **Result tallying**: Automatic counting of for/against votes
- **Time-based voting periods**: Proposals have deadlines
