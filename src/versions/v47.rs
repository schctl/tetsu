//! Event implementation for v47 of the protocol.
//! V47 covers server versions 1.8 - 1.8.9

use crate::errors::*;
use crate::event::*;
use crate::packet::*;

use uuid::Uuid;

mod internal {

    use crate::event::types::PlayerInfoUpdate;

    use super::*;

    use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
    use std::io;

    // Protocol specific types ---

    // Position ----

    #[derive(Debug, PartialEq)]
    pub struct PositionXYZ {
        pub x: i64,
        pub y: i64,
        pub z: i64,
    }

    impl From<Position> for PositionXYZ {
        #[inline]
        fn from(item: Position) -> Self {
            Self {
                x: item.x,
                y: item.y,
                z: item.z,
            }
        }
    }

    impl From<PositionXYZ> for Position {
        #[inline]
        fn from(item: PositionXYZ) -> Self {
            Self {
                x: item.x,
                y: item.y,
                z: item.z,
            }
        }
    }

    impl Readable for PositionXYZ {
        fn read_from<T: io::Read>(buf: &mut T) -> Result<Self, Error> {
            let val = buf.read_u64::<BigEndian>()?;

            let x = (val as i64) >> 38;
            let y = ((val as i64) >> 26) & 0xFFF;
            let z = ((val as i64) << 38) >> 38;

            Ok(Self {
                x: if x >= (2 << 24) { x - (2 << 25) } else { x },
                y: if y >= (2 << 10) { y - (2 << 11) } else { y },
                z: if z >= (2 << 24) { z - (2 << 25) } else { z },
            })
        }
    }

    impl Writable for PositionXYZ {
        #[inline]
        fn write_to<T: io::Write>(&self, buf: &mut T) -> Result<(), Error> {
            Ok(buf.write_i64::<BigEndian>(
                ((self.x & 0x3FFFFFF) << 38) | ((self.y & 0xFFF) << 26) | (self.z & 0x3FFFFFF),
            )?)
        }
    }

    // Statistics ----

    #[derive(Debug, PartialEq)]
    pub struct StatisticString {
        name: String,
        value: VarInt,
    }

    impl From<Statistic> for StatisticString {
        #[inline]
        fn from(item: Statistic) -> Self {
            Self {
                name: item.name,
                value: VarInt(item.value),
            }
        }
    }

    impl From<StatisticString> for Statistic {
        #[inline]
        fn from(item: StatisticString) -> Self {
            Self {
                name: item.name,
                value: item.value.0,
            }
        }
    }

    impl Readable for StatisticString {
        #[inline]
        fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
            let name = String::read_from(buf)?;
            let value = VarInt::read_from(buf)?;
            Ok(Self { name, value })
        }
    }

    impl Writable for StatisticString {
        #[inline]
        fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
            self.name.write_to(buf)?;
            self.value.write_to(buf)
        }
    }

    // Player info ---

    #[derive(Debug, PartialEq, Clone)]
    pub struct InternalPlayerProperty {
        pub name: String,
        pub value: String,
        pub signature: GenericOption<String>,
    }

    impl From<PlayerProperty> for InternalPlayerProperty {
        fn from(item: PlayerProperty) -> Self {
            Self {
                name: item.name,
                value: item.value,
                signature: GenericOption(item.signature),
            }
        }
    }

    impl From<InternalPlayerProperty> for PlayerProperty {
        fn from(item: InternalPlayerProperty) -> Self {
            Self {
                name: item.name,
                value: item.value,
                signature: item.signature.0,
            }
        }
    }

    impl Readable for InternalPlayerProperty {
        #[inline]
        fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
            let name = String::read_from(buf)?;
            let value = String::read_from(buf)?;
            let signature = GenericOption::read_from(buf)?;

            Ok(Self {
                name,
                value,
                signature,
            })
        }
    }

    impl Writable for InternalPlayerProperty {
        #[inline]
        fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
            self.name.write_to(buf)?;
            self.value.write_to(buf)?;
            self.signature.write_to(buf)
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct InternalPlayerInfoAdd {
        pub name: String,
        pub properties: GenericArray<VarInt, InternalPlayerProperty>,
        pub gamemode: VarInt,
        pub ping: VarInt,
        pub display: GenericOption<Chat>,
    }

    impl From<PlayerInfoAdd> for InternalPlayerInfoAdd {
        fn from(item: PlayerInfoAdd) -> Self {
            let properties: Vec<InternalPlayerProperty> = item
                .properties
                .into_iter()
                .map(|p| -> InternalPlayerProperty { p.into() })
                .collect();
            Self {
                name: item.name,
                properties: GenericArray::from(properties),
                gamemode: VarInt(gamemode_to_byte(&item.gamemode) as i32),
                ping: VarInt(item.ping),
                display: GenericOption(item.display),
            }
        }
    }

    impl From<InternalPlayerInfoAdd> for PlayerInfoAdd {
        fn from(item: InternalPlayerInfoAdd) -> Self {
            let properties = item
                .properties
                .1
                .into_iter()
                .map(|p| -> PlayerProperty { p.into() })
                .collect();
            Self {
                name: item.name,
                properties,
                gamemode: byte_to_gamemode(item.gamemode.0 as u8),
                ping: item.ping.0,
                display: item.display.0,
            }
        }
    }

    impl Readable for InternalPlayerInfoAdd {
        #[inline]
        fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
            let name = String::read_from(buf)?;
            let properties = GenericArray::read_from(buf)?;
            let gamemode = VarInt::read_from(buf)?;
            let ping = VarInt::read_from(buf)?;
            let display = GenericOption::read_from(buf)?;

            Ok(Self {
                name,
                properties,
                gamemode,
                ping,
                display,
            })
        }
    }

    impl Writable for InternalPlayerInfoAdd {
        #[inline]
        fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
            self.name.write_to(buf)?;
            self.properties.write_to(buf)?;
            self.gamemode.write_to(buf)?;
            self.ping.write_to(buf)?;
            self.display.write_to(buf)
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct InternalPlayerGamemodeUpdate {
        gamemode: VarInt,
    }

    impl From<PlayerGamemodeUpdate> for InternalPlayerGamemodeUpdate {
        fn from(item: PlayerGamemodeUpdate) -> Self {
            Self {
                gamemode: VarInt(gamemode_to_byte(&item.gamemode) as i32),
            }
        }
    }

    impl From<InternalPlayerGamemodeUpdate> for PlayerGamemodeUpdate {
        fn from(item: InternalPlayerGamemodeUpdate) -> Self {
            Self {
                gamemode: byte_to_gamemode(item.gamemode.0 as u8),
            }
        }
    }

    impl Readable for InternalPlayerGamemodeUpdate {
        #[inline]
        fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
            let gamemode = VarInt::read_from(buf)?;

            Ok(Self { gamemode })
        }
    }

    impl Writable for InternalPlayerGamemodeUpdate {
        #[inline]
        fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
            self.gamemode.write_to(buf)
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct InternalPlayerLatencyUpdate {
        ping: VarInt,
    }

    impl From<PlayerLatencyUpdate> for InternalPlayerLatencyUpdate {
        fn from(item: PlayerLatencyUpdate) -> Self {
            Self {
                ping: VarInt(item.ping),
            }
        }
    }

    impl From<InternalPlayerLatencyUpdate> for PlayerLatencyUpdate {
        fn from(item: InternalPlayerLatencyUpdate) -> Self {
            Self { ping: item.ping.0 }
        }
    }

    impl Readable for InternalPlayerLatencyUpdate {
        #[inline]
        fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
            let ping = VarInt::read_from(buf)?;

            Ok(Self { ping })
        }
    }

    impl Writable for InternalPlayerLatencyUpdate {
        #[inline]
        fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
            self.ping.write_to(buf)
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct InternalPlayerDisplayNameUpdate {
        display: GenericOption<Chat>,
    }

    impl From<PlayerDisplayNameUpdate> for InternalPlayerDisplayNameUpdate {
        fn from(item: PlayerDisplayNameUpdate) -> Self {
            Self {
                display: GenericOption(item.display),
            }
        }
    }

    impl From<InternalPlayerDisplayNameUpdate> for PlayerDisplayNameUpdate {
        fn from(item: InternalPlayerDisplayNameUpdate) -> Self {
            Self {
                display: item.display.0,
            }
        }
    }

    impl Readable for InternalPlayerDisplayNameUpdate {
        #[inline]
        fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
            let display = GenericOption::read_from(buf)?;

            Ok(Self { display })
        }
    }

    impl Writable for InternalPlayerDisplayNameUpdate {
        #[inline]
        fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
            self.display.write_to(buf)
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct InternalRemovePlayer {}

    impl From<RemovePlayer> for InternalRemovePlayer {
        fn from(_: RemovePlayer) -> Self {
            Self {}
        }
    }

    impl From<InternalRemovePlayer> for RemovePlayer {
        fn from(_: InternalRemovePlayer) -> Self {
            Self {}
        }
    }

    impl Readable for InternalRemovePlayer {
        #[inline]
        fn read_from<T: io::Read>(_: &mut T) -> TetsuResult<Self> {
            Ok(Self {})
        }
    }

    impl Writable for InternalRemovePlayer {
        #[inline]
        fn write_to<T: io::Write>(&self, _: &mut T) -> TetsuResult<()> {
            Ok(())
        }
    }

    #[derive(Debug, PartialEq)]
    pub enum InternalPlayerInfoAction {
        Add(InternalPlayerInfoAdd),
        GamemodeUpdate(InternalPlayerGamemodeUpdate),
        LatencyUpdate(InternalPlayerLatencyUpdate),
        DisplayNameUpdate(InternalPlayerDisplayNameUpdate),
        Remove(InternalRemovePlayer),
    }

    impl From<PlayerInfoAction> for InternalPlayerInfoAction {
        fn from(item: PlayerInfoAction) -> Self {
            match item {
                PlayerInfoAction::Add(e) => Self::Add(e.into()),
                PlayerInfoAction::GamemodeUpdate(e) => Self::GamemodeUpdate(e.into()),
                PlayerInfoAction::LatencyUpdate(e) => Self::LatencyUpdate(e.into()),
                PlayerInfoAction::DisplayNameUpdate(e) => Self::DisplayNameUpdate(e.into()),
                PlayerInfoAction::Remove(e) => Self::Remove(e.into()),
            }
        }
    }

    impl From<InternalPlayerInfoAction> for PlayerInfoAction {
        fn from(item: InternalPlayerInfoAction) -> Self {
            match item {
                InternalPlayerInfoAction::Add(e) => Self::Add(e.into()),
                InternalPlayerInfoAction::GamemodeUpdate(e) => Self::GamemodeUpdate(e.into()),
                InternalPlayerInfoAction::LatencyUpdate(e) => Self::LatencyUpdate(e.into()),
                InternalPlayerInfoAction::DisplayNameUpdate(e) => Self::DisplayNameUpdate(e.into()),
                InternalPlayerInfoAction::Remove(e) => Self::Remove(e.into()),
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct InternalPlayerInfo {
        pub uuid: Uuid,
        pub action: InternalPlayerInfoAction,
    }

    #[derive(Debug, PartialEq)]
    pub struct InternalPlayerListUpdate {
        players: Vec<InternalPlayerInfo>,
    }

    impl From<PlayerInfoUpdate> for InternalPlayerListUpdate {
        fn from(item: PlayerInfoUpdate) -> Self {
            Self {
                players: item
                    .players
                    .into_iter()
                    .map(|p| -> InternalPlayerInfo {
                        InternalPlayerInfo {
                            uuid: p.uuid,
                            action: p.action.into(),
                        }
                    })
                    .collect(),
            }
        }
    }

    impl From<InternalPlayerListUpdate> for PlayerInfoUpdate {
        fn from(item: InternalPlayerListUpdate) -> Self {
            Self {
                players: item
                    .players
                    .into_iter()
                    .map(|p| -> PlayerListInfo {
                        PlayerListInfo {
                            uuid: p.uuid,
                            action: p.action.into(),
                        }
                    })
                    .collect(),
            }
        }
    }

    impl Readable for InternalPlayerListUpdate {
        #[inline]
        fn read_from<T: io::Read>(buf: &mut T) -> TetsuResult<Self> {
            let action = VarInt::read_from(buf)?.0;
            let player_len = VarInt::read_from(buf)?.0;

            let mut players = vec![];

            for _ in 0..player_len {
                let uuid = Uuid::read_from(buf)?;
                let p_action = match action {
                    0 => InternalPlayerInfoAction::Add(InternalPlayerInfoAdd::read_from(buf)?),
                    1 => InternalPlayerInfoAction::GamemodeUpdate(
                        InternalPlayerGamemodeUpdate::read_from(buf)?,
                    ),
                    2 => InternalPlayerInfoAction::LatencyUpdate(
                        InternalPlayerLatencyUpdate::read_from(buf)?,
                    ),
                    3 => InternalPlayerInfoAction::DisplayNameUpdate(
                        InternalPlayerDisplayNameUpdate::read_from(buf)?,
                    ),
                    4 => InternalPlayerInfoAction::Remove(InternalRemovePlayer::read_from(buf)?),
                    _ => panic!("Unknown")
                };
                players.push(InternalPlayerInfo {
                    uuid,
                    action: p_action,
                });
            }

            Ok(Self { players })
        }
    }

    impl Writable for InternalPlayerListUpdate {
        #[inline]
        fn write_to<T: io::Write>(&self, buf: &mut T) -> TetsuResult<()> {
            let player_len = self.players.len();

            match self.players[0].action {
                InternalPlayerInfoAction::Add(_) => VarInt(0).write_to(buf)?,
                InternalPlayerInfoAction::GamemodeUpdate(_) => VarInt(1).write_to(buf)?,
                InternalPlayerInfoAction::LatencyUpdate(_) => VarInt(2).write_to(buf)?,
                InternalPlayerInfoAction::DisplayNameUpdate(_) => VarInt(3).write_to(buf)?,
                InternalPlayerInfoAction::Remove(_) => VarInt(4).write_to(buf)?,
            };

            VarInt(player_len as i32).write_to(buf)?;

            for i in self.players.iter() {
                i.uuid.write_to(buf)?;
                match &i.action {
                    InternalPlayerInfoAction::Add(e) => e.write_to(buf)?,
                    InternalPlayerInfoAction::GamemodeUpdate(e) => e.write_to(buf)?,
                    InternalPlayerInfoAction::LatencyUpdate(e) => e.write_to(buf)?,
                    InternalPlayerInfoAction::DisplayNameUpdate(e) => e.write_to(buf)?,
                    InternalPlayerInfoAction::Remove(e) => e.write_to(buf)?,
                };
            }

            Ok(())
        }
    }

    // Conversions ---

    #[inline]
    pub fn byte_to_gamemode(byte: UnsignedByte) -> Gamemode {
        match byte {
            0 => Gamemode::Survival,
            1 => Gamemode::Creative,
            2 => Gamemode::Adventure,
            3 => Gamemode::Spectator,
            _ => panic!("Unknown packet"),
        }
    }

    #[inline]
    pub fn gamemode_to_byte(gamemode: &Gamemode) -> UnsignedByte {
        match gamemode {
            Gamemode::Survival => 0,
            Gamemode::Creative => 1,
            Gamemode::Adventure => 2,
            Gamemode::Spectator => 3,
        }
    }

    #[inline]
    pub fn byte_to_dimension(byte: Byte) -> Dimension {
        match byte {
            -1 => Dimension::Nether,
            0 => Dimension::Overworld,
            1 => Dimension::End,
            _ => panic!("Unknown packet"),
        }
    }

    #[inline]
    pub fn dimension_to_byte(dimension: &Dimension) -> Byte {
        match dimension {
            Dimension::Nether => -1,
            Dimension::Overworld => 0,
            Dimension::End => 1,
        }
    }

    #[inline]
    pub fn byte_to_difficulty(byte: UnsignedByte) -> Difficulty {
        match byte {
            0 => Difficulty::Peaceful,
            1 => Difficulty::Easy,
            2 => Difficulty::Normal,
            3 => Difficulty::Hard,
            _ => panic!("Unknown packet"),
        }
    }

    #[inline]
    pub fn difficulty_to_byte(difficulty: &Difficulty) -> UnsignedByte {
        match difficulty {
            Difficulty::Peaceful => 0,
            Difficulty::Easy => 1,
            Difficulty::Normal => 2,
            Difficulty::Hard => 3,
        }
    }
}

use internal::*;

// ---------------

protocol_impl! {

    inherit {
    }

    // Status ------------------

    (0x01) ServerBound Status StatusPingPacket: Ping {
        from_event {
            fn try_from(item: Ping) -> TetsuResult<StatusPingPacket> {
                Ok(StatusPingPacket {
                    payload: item.payload
                })
            }
        }
        to_event {
            fn try_from(item: StatusPingPacket) -> TetsuResult<Event> {
                Ok(Event::Ping(Ping {
                    payload: item.payload
                }))
            }
        }
        fields {
            payload: Long,
        }
    }

    (0x01) ClientBound Status StatusPongPacket: Pong {
        from_event {
            fn try_from(item: Pong) -> TetsuResult<StatusPongPacket> {
                Ok(StatusPongPacket {
                    payload: item.payload
                })
            }
        }
        to_event {
            fn try_from(item: StatusPongPacket) -> TetsuResult<Event> {
                Ok(Event::Pong(Pong {
                    payload: item.payload
                }))
            }
        }
        fields {
            payload: Long,
        }
    }

    (0x00) ServerBound Status StatusRequestPacket: StatusRequest {
        from_event {
            fn try_from(_: StatusRequest) -> TetsuResult<StatusRequestPacket> {
                Ok(StatusRequestPacket {})
            }
        }
        to_event {
            fn try_from(_: StatusRequestPacket) -> TetsuResult<Event> {
                Ok(Event::StatusRequest(StatusRequest {}))
            }
        }
        fields {

        }
    }

    (0x00) ClientBound Status StatusResponsePacket: StatusResponse {
        from_event {
            fn try_from(item: StatusResponse) -> TetsuResult<StatusResponsePacket> {
                Ok(StatusResponsePacket {
                    response: serde_json::to_string(&item.response).unwrap()
                })
            }
        }
        to_event {
            fn try_from(item: StatusResponsePacket) -> TetsuResult<Event> {
                Ok(Event::StatusResponse(StatusResponse {
                    response: serde_json::from_str(&item.response[..]).unwrap()
                }))
            }
        }
        fields {
            response: String,
        }
    }

    // Handshake ---------------

    (0x00) ServerBound Handshake HandshakePacket: Handshake {
        from_event {
            fn try_from(item: Handshake) -> TetsuResult<HandshakePacket> {
                Ok(HandshakePacket {
                    protocol_version: VarInt(47),
                    server_address: item.server_address,
                    server_port: item.server_port,
                    next_state: match item.next_state {
                        EventState::Status => VarInt(1),
                        EventState::Login => VarInt(2),
                        _ => return Err(Error::from(InvalidValue { expected: "Status or Login".to_owned() }))
                    }
                })
            }
        }
        to_event {
            fn try_from(item: HandshakePacket) -> TetsuResult<Event> {
                Ok(Event::Handshake(Handshake {
                    server_address: item.server_address,
                    server_port: item.server_port,
                    next_state: match item.next_state.0 {
                        1 => EventState::Status,
                        2 => EventState::Login,
                        _ => return Err(Error::from(InvalidValue { expected: "1 or 2".to_owned() }))
                    }
                }))
            }
        }
        fields {
            protocol_version: VarInt,
            server_address: String,
            server_port: UnsignedShort,
            next_state: VarInt,
        }
    }

    // Login -------------------

    (0x00) ServerBound Login LoginStartPacket: LoginStart {
        from_event {
            fn try_from(item: LoginStart) -> TetsuResult<LoginStartPacket> {
                Ok(LoginStartPacket {
                    name: item.name
                })
            }
        }
        to_event {
            fn try_from(item: LoginStartPacket) -> TetsuResult<Event> {
                Ok(Event::LoginStart(LoginStart {
                    name: item.name
                }))
            }
        }
        fields {
            name: String,
        }
    }

    (0x00) ClientBound Login DisconnectPacket: Disconnect {
        from_event {
            fn try_from(item: Disconnect) -> TetsuResult<DisconnectPacket> {
                Ok(DisconnectPacket {
                    reason: match item.reason.text {
                        Some(t) => t,
                        None => panic!("Unknown reason")
                    }
                })
            }
        }
        to_event {
            fn try_from(item: DisconnectPacket) -> TetsuResult<Event> {
                Ok(Event::Disconnect(Disconnect {
                    reason: Chat {
                        text: Some(item.reason),
                        translate: None,
                        bold: None,
                        italic: None,
                        underlined: None,
                        obfuscated: None,
                        strikethrough: None,
                        color: None,
                        click_event: None,
                        hover_event: None,
                        extra: None
                    }
                }))
            }
        }
        fields {
            reason: String,
        }
    }

    (0x01) ClientBound Login EncryptionRequestVarIntPacket: EncryptionRequest {
        from_event {
            fn try_from(item: EncryptionRequest) -> TetsuResult<EncryptionRequestVarIntPacket> {
               Ok( EncryptionRequestVarIntPacket {
                    server_id: item.server_id,
                    public_key: ByteArrayVarInt(item.public_key.len(), item.public_key),
                    verify_token: ByteArrayVarInt(item.verify_token.len(), item.verify_token)
                })
            }
        }
        to_event {
            fn try_from(item: EncryptionRequestVarIntPacket) -> TetsuResult<Event> {
                Ok(Event::EncryptionRequest(EncryptionRequest {
                    server_id: item.server_id,
                    public_key: item.public_key.1,
                    verify_token: item.verify_token.1
                }))
            }
        }
        fields {
            server_id: String,
            public_key: ByteArrayVarInt,
            verify_token: ByteArrayVarInt,
        }
    }

    (0x01) ServerBound Login EncryptionResponseVarIntPacket: EncryptionResponse {
        from_event {
            fn try_from(item: EncryptionResponse) -> TetsuResult<EncryptionResponseVarIntPacket> {
                Ok(EncryptionResponseVarIntPacket {
                    shared_secret: ByteArrayVarInt(item.shared_secret.len(), item.shared_secret),
                    verify_token: ByteArrayVarInt(item.verify_token.len(), item.verify_token)
                })
            }
        }
        to_event {
            fn try_from(item: EncryptionResponseVarIntPacket) -> TetsuResult<Event> {
                Ok(Event::EncryptionResponse(EncryptionResponse {
                    shared_secret: item.shared_secret.1,
                    verify_token: item.verify_token.1
                }))
            }
        }
        fields {
            shared_secret: ByteArrayVarInt,
            verify_token: ByteArrayVarInt,
        }
    }

    (0x02) ClientBound Login LoginSuccessPacket: LoginSuccess {
        from_event {
            fn try_from(item: LoginSuccess) -> TetsuResult<LoginSuccessPacket> {
                Ok(LoginSuccessPacket {
                    uuid: item.uuid.to_hyphenated().to_string(),
                    name: item.name
                })
            }
        }
        to_event {
            fn try_from(item: LoginSuccessPacket) -> TetsuResult<Event> {
                Ok(Event::LoginSuccess(LoginSuccess {
                    uuid: Uuid::parse_str(&item.uuid[..]).unwrap(),
                    name: item.name,
                }))
            }
        }
        fields {
            uuid: String,
            name: String,
        }
    }

    (0x03) ClientBound Login SetCompressionPacket: SetCompression {
        from_event {
            fn try_from(item: SetCompression) -> TetsuResult<SetCompressionPacket> {
                Ok(SetCompressionPacket {
                    threshold: VarInt(item.threshold)
                })
            }
        }
        to_event {
            fn try_from(item: SetCompressionPacket) -> TetsuResult<Event> {
                Ok(Event::SetCompression(SetCompression {
                    threshold: item.threshold.0
                }))
            }
        }
        fields {
            threshold: VarInt,
        }
    }

    // Play --------------------

    (0x00) ClientBound Play KeepAlivePacket: KeepAlive {
        from_event {
            fn try_from(item: KeepAlive) -> TetsuResult<KeepAlivePacket> {
                Ok(KeepAlivePacket {
                    id: VarInt(item.id as i32)
                })
            }
        }
        to_event {
            fn try_from(item: KeepAlivePacket) -> TetsuResult<Event> {
                Ok(Event::KeepAlive(KeepAlive {
                    id: item.id.0 as i64
                }))
            }
        }
        fields {
            id: VarInt,
        }
    }

    (0x01) ClientBound Play JoinGamePacket: JoinGame {
        from_event {
            fn try_from(item: JoinGame) -> TetsuResult<JoinGamePacket> {
                Ok(JoinGamePacket {
                    id: item.id,
                    gamemode: gamemode_to_byte(&item.gamemode) | (if item.is_hardcore { 0x80 } else { 0x00 }),
                    dimension: dimension_to_byte(&item.dimension),
                    difficulty: difficulty_to_byte(&item.difficulty),
                    max_players: item.max_players as u8,
                    level_type: item.world_type,
                    reduced_debug: item.reduced_debug
                })
            }
        }
        to_event {
            fn try_from(item: JoinGamePacket) -> TetsuResult<Event> {
                Ok(Event::JoinGame(JoinGame {
                    id: item.id,
                    gamemode: byte_to_gamemode(item.gamemode),
                    is_hardcore: item.gamemode & 0x80 == 0x80,
                    dimension: byte_to_dimension(item.dimension),
                    difficulty: byte_to_difficulty(item.difficulty),
                    max_players: item.max_players as u32,
                    world_type: item.level_type,
                    reduced_debug: item.reduced_debug
                }))
            }
        }
        fields {
            id: Int,
            gamemode: UnsignedByte,
            dimension: Byte,
            difficulty: UnsignedByte,
            max_players: UnsignedByte,
            level_type: String,
            reduced_debug: bool,
        }
    }

    (0x05) ClientBound Play SpawnPositionPacket: SpawnPosition {
        from_event {
            fn try_from(item: SpawnPosition) -> TetsuResult<SpawnPositionPacket> {
                Ok(SpawnPositionPacket {
                    location: item.location.into()
                })
            }
        }
        to_event {
            fn try_from(item: SpawnPositionPacket) -> TetsuResult<Event> {
                Ok(Event::SpawnPosition(SpawnPosition {
                    location: item.location.into()
                }))
            }
        }
        fields {
            location: PositionXYZ,
        }
    }

    (0x09) ClientBound Play HeldItemChangePacket: HeldItemChange {
        from_event {
            fn try_from(item: HeldItemChange) -> TetsuResult<HeldItemChangePacket> {
                Ok(HeldItemChangePacket {
                    slot: item.slot
                })
            }
        }
        to_event {
            fn try_from(item: HeldItemChangePacket) -> TetsuResult<Event> {
                Ok(Event::HeldItemChange(HeldItemChange {
                    slot: item.slot
                }))
            }
        }
        fields {
            slot: Byte,
        }
    }

    (0x37) ClientBound Play StatisticsPacket: Statistics {
        from_event {
            fn try_from(item: Statistics) -> TetsuResult<StatisticsPacket> {
                let values: Vec<StatisticString> = item.values.into_iter().map(|s| -> StatisticString { s.into() }).collect();
                Ok(StatisticsPacket {
                    values: GenericArray::from(values)
                })
            }
        }
        to_event {
            fn try_from(item: StatisticsPacket) -> TetsuResult<Event> {
                Ok(Event::Statistics(Statistics {
                    values: item.values.1.into_iter().map(|s| -> Statistic { s.into() }).collect()
                }))
            }
        }
        fields {
            values: GenericArray<VarInt, StatisticString>,
        }
    }

    (0x38) ClientBound Play PlayListItemPacket: PlayerInfoUpdate {
        from_event {
            fn try_from(item: PlayerInfoUpdate) -> TetsuResult<PlayListItemPacket> {
                Ok(Self {
                    players: InternalPlayerListUpdate::from(item)
                })
            }
        }
        to_event {
            fn try_from(item: PlayListItemPacket) -> TetsuResult<Event> {
                Ok(Event::PlayerInfoUpdate(PlayerInfoUpdate::from(item.players)))
            }
        }
        fields {
            players: InternalPlayerListUpdate,
        }
    }

    (0x39) ClientBound Play PlayerAbilityPacket: PlayerAbility {
        from_event {
            fn try_from(item: PlayerAbility) -> TetsuResult<PlayerAbilityPacket> {
                Ok(PlayerAbilityPacket {
                    flags: (if item.invulnerable { 0x01 } else { 0x00 })
                         | (if item.is_flying { 0x02 } else { 0x00 })
                         | (if item.allow_flying { 0x04 } else { 0x00 })
                         | (if item.creative_mode { 0x08 } else { 0x00 }),
                    flying_speed: item.flying_speed,
                    walking_speed: item.walking_speed
                })
            }
        }
        to_event {
            fn try_from(item: PlayerAbilityPacket) -> TetsuResult<Event> {
                Ok(Event::PlayerAbility(PlayerAbility {
                    invulnerable: item.flags & 0x01 == 0x01,
                    is_flying: item.flags & 0x02 == 0x02,
                    allow_flying: item.flags & 0x04 == 0x04,
                    creative_mode: item.flags & 0x08 == 0x08,
                    flying_speed: item.flying_speed,
                    walking_speed: item.walking_speed
                }))
            }
        }
        fields {
            flags: Byte,
            flying_speed: Float,
            walking_speed: Float,
        }
    }

    (0x3F) ClientBound Play PluginMessagePacket: PluginMessage {
        from_event {
            fn try_from(item: PluginMessage) -> TetsuResult<PluginMessagePacket> {
                Ok(PluginMessagePacket {
                    channel: item.channel,
                    data: item.data
                })
            }
        }
        to_event {
            fn try_from(item: PluginMessagePacket) -> TetsuResult<Event> {
                Ok(Event::PluginMessage(PluginMessage {
                    channel: item.channel,
                    data: item.data
                }))
            }
        }
        fields {
            channel: String,
            data: Vec<u8>,
        }
    }

    (0x41) ClientBound Play ServerDifficultyUpdatePacket: ServerDifficultyUpdate {
        from_event {
            fn try_from(item: ServerDifficultyUpdate) -> TetsuResult<ServerDifficultyUpdatePacket> {
                Ok(ServerDifficultyUpdatePacket {
                    difficulty: difficulty_to_byte(&item.difficulty)
                })
            }
        }
        to_event {
            fn try_from(item: ServerDifficultyUpdatePacket) -> TetsuResult<Event> {
                Ok(Event::ServerDifficultyUpdate(ServerDifficultyUpdate {
                    difficulty: byte_to_difficulty(item.difficulty),
                    difficulty_locked: false
                }))
            }
        }
        fields {
            difficulty: UnsignedByte,
        }
    }

}
