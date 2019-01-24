const gameState = JSON.parse(process.argv[2]);

const newLocation = {
  x: gameState.location.x,
  y: gameState.location.y
};

console.log(JSON.stringify(newLocation));
