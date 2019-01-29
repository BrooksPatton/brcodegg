const gameInput = JSON.parse(process.argv[2]);
const { board, turn_number } = gameInput;
let location = { x: null, y: null };

if (turn_number === 0) {
  chooseRandomLocationOnBoard({ board, location });
}

console.log(JSON.stringify(location));

function chooseRandomLocationOnBoard({ location }) {
  location.x = 5;
  location.y = 5;
}
