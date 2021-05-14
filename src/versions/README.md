# Structure

## Implementing packet types

All protocol version must be able to convert native types to corresponding `event::types`.

- If all protocol versions share the same implementation for a type in `event::types`, define its implementations directly in `common.rs`.
- If an `event::types` type has different implementations accross different versions, define a private "intermediate type" for the type in each protocol implementation and implement `From<"intermediate_type">` for the `"event_type"`, and vice-versa.

## Packets implemented

### v47

#### ClientBound

- [x] Response
- [x] Pong

- [x] Disconnect
- [x] Encryption Request
- [x] Login Success
- [x] Set Compression

- [x] Keep Alive
- [x] Join Game
- [ ] Chat Message
- [ ] Time Update
- [ ] Entity Equipment
- [x] Spawn Position
- [ ] Update Health
- [ ] Respawn
- [x] Player Position And Look
- [x] Held Item Change
- [ ] Use Bed
- [ ] Animation
- [ ] Spawn Player
- [ ] Collect Item
- [ ] Spawn Object
- [ ] Spawn Mob
- [ ] Spawn Painting
- [ ] Spawn Experience Orb
- [ ] Entity Velocity
- [ ] Destroy Entities
- [ ] Entity
- [ ] Entity Relative Move
- [ ] Entity Look
- [ ] Entity Look And Relative Move
- [ ] Entity Teleport
- [ ] Entity Head Look
- [ ] Entity Status
- [ ] Attach Entity
- [ ] Entity Metadata
- [ ] Entity Effect
- [ ] Remove Entity Effect
- [ ] Set Experience
- [ ] Entity Properties
- [ ] Chunk Data
- [ ] Multi Block Change
- [ ] Block Change
- [ ] Block Action
- [ ] Block Break Animation
- [ ] Map Chunk Bulk
- [ ] Explosion
- [ ] Effect
- [ ] Sound Effect
- [ ] Particle
- [ ] Change Game State
- [ ] Spawn Global Entity
- [ ] Open Window
- [ ] Close Window
- [ ] Set Slot
- [ ] Window Items
- [ ] Window Property
- [ ] Confirm Transaction
- [ ] Update Sign
- [ ] Map
- [ ] Update Block Entity
- [ ] Open Sign Editor
- [x] Statistics
- [x] Player List Item
- [x] Player Abilities
- [ ] Tab-Complete
- [ ] Scoreboard Objective
- [ ] Update Score
- [ ] Display Scoreboard
- [ ] Teams
- [x] Plugin Message
- [ ] Disconnect
- [ ] Server Difficulty
- [ ] Combat Event
- [ ] Camera
- [ ] World Border
- [ ] Title
- [ ] Set Compression
- [ ] Player List Header And Footer
- [ ] Resource Pack Send
- [ ] Update Entity NBT

#### ServerBound

- [x] Handshake

- [x] Request
- [x] Ping

- [x] Login Start
- [x] Encryption Response

- [ ] Keep Alive
- [ ] Chat Message
- [ ] Use Entity
- [ ] Player
- [ ] Player Position
- [ ] Player Look
- [ ] Player Position And Look
- [ ] Player Digging
- [ ] Player Block Placement
- [ ] Held Item Change
- [ ] Animation
- [ ] Entity Action
- [ ] Steer Vehicle
- [ ] Close Window
- [ ] Click Window
- [ ] Confirm Transaction
- [ ] Creative Inventory Action
- [ ] Enchant Item
- [ ] Update Sign
- [ ] Player Abilities
- [ ] Tab-Complete
- [ ] Client Settings
- [ ] Client Status
- [ ] Plugin Message
- [ ] Spectate
- [ ] Resource Pack Status
