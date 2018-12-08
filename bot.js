const gameState = process.argv[2]

const state = JSON.parse(gameState)

const currentLocation = {
    x: state.current_location_x,
    y: state.current_location_y
}

currentLocation.x++;
currentLocation.y++;

console.log(JSON.stringify(currentLocation))