# brcodegg
This is the primary repo for my battle royale coding game

## Player Stories

### Doing

### Done

### Backlog

#### Spike Research

* [x] Architecture
  * [x] Authentication (oauth with github)
  * [x] Separate API and web server or not? (together)
* [x] Technology to use for Front end
  * [x] React (React as I want to improve my skills with it)
  * [x] Vanilla.js
  * [ ] Replay technology 
    * [ ] Divs and vanilla
    * [ ] Canvas
    * [ ] P5js
* [-] Technology to use for the Api
  * [ ] Actix-web
  * [ ] Database to use
  * [x] Oauth with Github and a rust server
* [ ] How to be GDPR compliance
* [ ] Create a bot
  * [ ] Python
  * [ ] Node.js
* [ ] Calling multiple docker containers from within a rust app

#### General Development

* [ ] As a developer, I would like to see a contribution guide so that I know how to help
  * [ ] Near the top of the README
  * [ ] Include a contributors section
  * [ ] Create tags for different kinds of issues
* [ ] As a devoloper, I would like to see a Code of Conduct so that I know how to conduct myself in this community
  * [ ] Find one from a organization that I like
  * [ ] Make it not sound legal

#### Front End

* [ ] As a possible player, I want to see what game I'm on
  * [ ] Top nav-bar
  * [ ] BRcodeGG logo
  * [ ] Description of the game
  * [ ] Video introduction of the game
* [ ] As a player, I want to be able to log in
  * [ ] Can log in using github
  * [ ] can log out using github
  * [ ] delete the account
  * [ ] download all data I have on the player
* [ ] As a player, I want to submit a bot to the game
  * [ ] See a list of bots already submitted
  * [ ] Add a bot
  * [ ] Remove a bot
* [ ] As a player, I want to see how my bots are doing
  * [ ] bot profile page
  * [ ] bot statistics
  * [ ] replays with the bot in question highlighted
  * [ ] Link to the code
* [ ] As a player, I want to watch matches
  * [ ] List of battles that include the players bots
  * [ ] Clicking on battle brings player to a view battle page
  * [ ] Filter to show all battles regardless of players bots in the game
* [ ] As a player, I want to share awesome battles with the world
  * [ ] unique link for every battle
  * [ ] Share on twitter
  * [ ] Share on Reddit
* [ ] As a player, I want to see the bot api documentation so that I know how to write a bot
  * [ ] page for the documentation
  * [ ] API endpoints for the bot
  * [ ] Example code using each of the endpoints

#### Game Engine

* [ ] As a bot, I want to know the state of the game when its my turn
  * [ ] Input via standard in
  * [ ] dimensions of the arena
  * [ ] locations of other bots
  * [ ] my location in the arena
  * [ ] drop locations
  * [ ] bullets
  * [ ] my inventory
  * [ ] speed
  * [ ] can I fire a bullet?
  * [ ] success and error codes for previously taken actions
* [ ] As a bot, I want to tell the game what my actions are every turn
  * [ ] output move action
  * [ ] output where to fire my bullet
  * [ ] As a bot, I want to live in a container so to that I can save my state
  * [ ] As a bot, I want my container to be locked down so that I cannot break out
* [ ] As a player, I want to know how my bot did
  * [ ] Record the results to the database
  * [ ] Save the game replay to the database

#### Future (not doing)
