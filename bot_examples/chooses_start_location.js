const gameInput = JSON.parse(process.argv[2]);
const { board, turn_number } = gameInput;
let {location} = gameInput;

if (turn_number === 0) {
  chooseRandomLocationOnBoard({ board, location });
} else {
  moveRandomly({board, location});
}

console.log(JSON.stringify(location));

function chooseRandomLocationOnBoard({ location }) {
  location.x = 5;
  location.y = 5;
}

function moveRandomly({location}) {
  const random = Math.random();

  if (random < 0.25) {
    location.x -= 1;
  } else if (random < 0.5) {
    location.y -= 1;
  } else if (random < 0.75) {
    location.x += 1;
  } else {
    location.y += 1;
  }
}