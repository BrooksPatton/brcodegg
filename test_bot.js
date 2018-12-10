const gameState = JSON.parse(process.argv[2])

const newLocation = {
    x: gameState.location.x,
    y: gameState.location.y
}

newLocation.x += Math.random() * gameState.radius;
newLocation.y += Math.random() * gameState.radius;

console.log(JSON.stringify(newLocation))