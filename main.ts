// // Bootstrap
// import 'https://stackpath.bootstrapcdn.com/bootstrap/4.5.2/js/bootstrap.min.js';

// // Material UI
// import 'https://unpkg.com/@material-ui/core@latest/umd/material-ui.min.js';

// Obtain Chainlink VRF Random Number
    // logic goes here
    // isPrime(num-generated)
    // if (!isPrime) {findNextPrime(num-generated)}
    // 

/**
   * @notice Cycles through consecutively occurring integers after N until a prime integer is reached
   * @param numGenerated - a large random integer generated by ChainlinkVRF
   * @returns N - the next prime integer occurring after numGenerated
*/
function findNextPrime(numGenerated) {
    let N = numGenerated + 1;
    while (!isPrime(N)) {
        console.log("N tested for primality: ", N);
        N++;
    }
    console.log("This N is Prime: ", N);
    return N;
  }


/**
   * @notice Checks if the number entered is prime
   * @param number the number to be checked
   * @returns (boolean) - true if prime, false if not
*/
// function isPrime(number) {
//     // checks if number is divisible by any integer (greater than 1) up to the sqrt of number
//     for (let i = 2; i <= Math.sqrt(number); i++) {
//       if (number % i === 0) {
//         return false;   // not prime
//       }
//     }
//     return true;        // prime
//   }

function isPrime(num) {
    if (num == 2 || num == 3)
      return true;
    if (num <= 1 || num % 2 == 0 || num % 3 == 0)
      return false;  
    for (let i = 5; i * i <= num ; i+=6)
      if (num % i == 0 || num % (i + 2) == 0)
        return false;
    return true;
}

  findNextPrime(943761546512977987);