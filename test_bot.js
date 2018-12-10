const gameState = JSON.parse(process.argv[2]);

const newLocation = {
  x: gameState.location.x,
  y: gameState.location.y
};

newLocation.x += 1;
newLocation.y += 1;

console.log(JSON.stringify(newLocation));
