const gameInput = JSON.parse(process.argv[2]);
const { id, board, turn_number } = gameInput;
let location = { x: null, y: null };

if (turn_number === 0) {
  chooseRandomLocationOnBoard({ board, location });
}

console.log(JSON.stringify(location));

function chooseRandomLocationOnBoard({ board, location }) {
  const { width, height } = board;
  const x = Math.floor(Math.random() * width);
  const y = Math.floor(Math.random() * height);

  location.x = x;
  location.y = y;
}
