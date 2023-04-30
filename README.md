# vrf-ssl
Verifiably Random SSL Cert Issuance using Chainlink's VRF



Overall Plan
    -Simple MUI UI front-end framework for Alice, Bob, and Eve
    -Implement Basic React framework
    -Diffie-Hellman Formula Working w/ basic randomness for selection of N and g
    -Set up Chainlink VRF, get a request working
    -replace large prime number (N) randomness select with VRF
    -replace generator of N (g) randomness selection with VRF (may require legwork in validating )
        -NOTE: The formula for producing a random modular generator (g) of a large prime number (N) is g = a^x mod N, where a is a random number from 1 to N-1 and x is a random number from 0 to N-2.
    -Re-design UI using cool React features
    -Add SSL cert display window
