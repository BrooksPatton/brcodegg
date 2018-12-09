const gameState = JSON.parse(process.argv[2])

const newLocation = {
    x: gameState.current_location_x,
    y: gameState.current_location_y
}

newLocation.x++;
newLocation.y++;

console.log(JSON.stringify(newLocation))