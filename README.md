# Bitcoin Dojo

A from-scratch Bitcoin implementation. No shortcuts, no libraries doing the hard parts for me — I'm building the cryptographic primitives, transactions, blocks, and p2p networking by hand.

Two goals:

1. **Learn Bitcoin the hard way.** I don't really understand something until I've built it myself. So I'm building it.
2. **Get good at Rust.** I genuinely enjoy the language, and the only way to get fluent is to write a lot of it.

I'm following the [BitcoinDojo](https://bitcoindojo.dev/) format because it gives me a clear path: implement a primitive, run it against a pile of tests, unlock the next lesson. The gamified "unlock the next thing" loop sounds gimmicky, but it works — I do this in my spare time, and making it fun is what keeps me from procrastinating. :-)

## A note on AI and agentic tools

At my day job I lean hard on agentic coding tools. They make me faster, and writing code is now almost "free" — the real work is knowing *what* to build. That's where I spend my time, and I think it's what companies expect from senior people today.

But I can work that way *because* I have the background to back it up: years of Java, Go, and Python, and enough production incidents to feel the pain when things break. That foundation is what lets me hand the AI good specs and clear intent so it gets things right instead of drifting or hallucinating. LLMs are statistical machines — they'll always predict *something* next, even when it's nonsense. You need to know enough to catch it.

**For this project I do the exact opposite.** AI autocomplete and code suggestions are off. I write everything by hand, boilerplate included. The point here isn't shipping fast, it's learning — and learning as a human means friction: making mistakes, repeating things, getting it wrong before getting it right.

I do still *chat* with AI, though. For understanding a concept, poking at a corner case, or getting an ELI5, it's fantastic — a private tutor that adapts to my level. I learn top-down, framework first, and AI is great at meeting me there. It was a lifesaver while I was working through the cryptographic math (field elements and friends).

## About me

I'm a software and data engineer with 15+ years building scalable systems and large-scale data applications, across healthcare, satellite imagery, cloud infrastructure, and fintech/banking.

In 2021 I switched to blockchain, working professionally at an Ethereum-based company. But the more chains I explored, the more Bitcoin stood out — not just for its economics and philosophy, but because it's *simple*. It doesn't try to be a thousand things. It's money. Programmable money.

I've been learning and building in the Bitcoin ecosystem since Chaincode Labs' **Bitcoin and Lightning Protocol Development** program (2025 cohort). Four intense months — I learned more than I could digest. Since then I've been going deep, step by step: a 9-month Rust course, *Mastering Bitcoin*, and a steady diet of BIPs, books, and tutorials.
