const start = new Date().getTime();

let primes = 0;

let x = 1;

while (true) {
  const now = new Date().getTime();
  if (now - start >= 10000) {
    console.log(
      `Found ${primes} primes numbers on ${x} in ${(now - start) / 1000}s`
    );
    break;
  }

  let isPrime = true;
  let y = 3;
  while (isPrime && y <= Math.sqrt(x)) {
    if (x % y === 0) {
      isPrime = false;
    }
    y += 2;
  }
  if (isPrime) {
    primes++;
  }
  x += 2;
}
