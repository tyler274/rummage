# Chat System Network Protocol

This document details the network protocol used by Rummage's in-game chat system for transmitting text and voice data between players.

## Table of Contents

1. [Overview](#overview)
2. [Network Architecture](#network-architecture)
3. [Text Chat Protocol](#text-chat-protocol)
4. [Voice Chat Protocol](#voice-chat-protocol)
5. [Security Considerations](#security-considerations)
6. [Bandwidth Optimization](#bandwidth-optimization)
7. [Implementation Details](#implementation-details)
8. [Error Handling](#error-handling)

## Overview

The chat network protocol is designed to efficiently transmit both text and voice data between players in a Commander game. It prioritizes low latency for real-time communication while maintaining bandwidth efficiency and reliability.

Key design principles:

- **Efficiency**: Minimize bandwidth usage without sacrificing quality
- **Reliability**: Ensure delivery of text messages while optimizing voice transmission
- **Security**: Protect player communication from tampering and eavesdropping
- **Scalability**: Support multiple players with varying network conditions
- **Extensibility**: Allow for future protocol extensions and enhancements

## Network Architecture

### Communication Model

The chat system uses a hybrid networking model:

- **Peer-to-Peer Mode**: Direct connections between players for lower latency (default for voice)
- **Client-Server Mode**: Messages routed through a central server (default for text)
- **Relay Mode**: Fallback when direct connections aren't possible due to NAT/firewalls

### Connection Flow

1. **Initialization**: When joining a game, players establish communication channels
2. **Channel Negotiation**: Determine optimal connection type for each player pair
3. **Session Establishment**: Create encrypted sessions for text and voice
4. **Heartbeat Monitoring**: Regular connectivity checks to detect disconnections
5. **Graceful Termination**: Proper session closure when leaving a game

### Protocol Layers

The network protocol is organized in layers:

```
┌─────────────────────────────────────┐
│ Application Layer (Chat UI/Logic)   │
├─────────────────────────────────────┤
│ Message Layer (Serialization/Types) │
├─────────────────────────────────────┤
│ Session Layer (Encryption/Auth)     │
├─────────────────────────────────────┤
│ Transport Layer (UDP/TCP)           │
└─────────────────────────────────────┘
```

- **Transport Layer**: Uses TCP for text messages and UDP for voice data
- **Session Layer**: Handles encryption, authentication, and session management
- **Message Layer**: Serializes/deserializes messages and handles message types
- **Application Layer**: Processes messages for display and user interaction

## Text Chat Protocol

### Message Format

Text messages use a binary format with the following structure:

```
┌────────┬────────┬───────────┬──────────┬─────────┬────────────┐
│ Header │ Length │ Timestamp │ Metadata │ Payload │ Checksum   │
│ (8B)   │ (4B)   │ (8B)      │ (Variable)│ (Variable)│ (4B)      │
└────────┴────────┴───────────┴──────────┴─────────┴────────────┘
```

#### Header (8 bytes)

```
┌───────────────┬─────────────┬───────────┬───────────┬───────────┐
│ Protocol Ver. │ Message Type│ Channel ID│ Sender ID │ Flags     │
│ (1B)          │ (1B)        │ (2B)      │ (2B)      │ (2B)      │
└───────────────┴─────────────┴───────────┴───────────┴───────────┘
```

- **Protocol Version**: Current protocol version (currently 1)
- **Message Type**: Type of message (standard, system, command, etc.)
- **Channel ID**: Channel identifier for routing (global, team, private, etc.)
- **Sender ID**: Unique ID of the sender
- **Flags**: Special message flags (e.g., encrypted, acknowledged, etc.)

#### Length (4 bytes)

Total message length in bytes, including header and checksum.

#### Timestamp (8 bytes)

Unix timestamp with millisecond precision for message ordering and latency calculation.

#### Metadata (Variable)

Optional metadata fields depending on message type:

- For standard messages: formatting options, reply references, etc.
- For system messages: event types, severity levels, etc.
- For command messages: command identifiers, argument count, etc.

#### Payload (Variable)

The actual message content, UTF-8 encoded text. For binary data (e.g., emotes, images), a specialized binary format is used with appropriate type headers.

#### Checksum (4 bytes)

CRC32 checksum for message integrity verification.

### Message Types

The protocol supports various message types:

| Type ID | Name | Description |
|---------|------|-------------|
| 0x01 | STANDARD | Regular user message |
| 0x02 | SYSTEM | System message or notification |
| 0x03 | COMMAND | Chat command |
| 0x04 | EMOTE | Emote action |
| 0x05 | REACTION | Message reaction |
| 0x06 | STATUS | User status change |
| 0x07 | READ_RECEIPT | Message read confirmation |
| 0x08 | TYPING | Typing indicator |
| 0x09 | BINARY | Binary data (images, etc.) |
| 0x0A | CONTROL | Protocol control message |
| 0x0B | VOICE_CONTROL | Voice chat control |

### Flow Control

The text chat protocol implements flow control mechanisms:

- **Rate Limiting**: Maximum messages per second (configurable per channel)
- **Message Ordering**: Sequence numbers to handle out-of-order delivery
- **Message Acknowledgment**: Optional ACKs for important messages
- **Retry Logic**: Automatic retransmission of unacknowledged messages
- **Flood Protection**: Dynamic rate limiting based on channel activity

### Serialization

Message serialization follows these steps:

1. Create message object with all required fields
2. Serialize message to binary format with appropriate headers
3. Apply compression if message exceeds threshold size
4. Encrypt message content if required
5. Calculate and append checksum
6. Transmit over appropriate transport (TCP or WebSocket)

## Voice Chat Protocol

### Packet Format

Voice data uses a compact binary format optimized for real-time transmission:

```
┌────────┬────────┬───────────┬──────────┬─────────┬────────────┐
│ Header │ Sequence│ Timestamp │ Channel  │ Payload │ Checksum   │
│ (4B)   │ (2B)    │ (4B)      │ (1B)     │ (Variable)│ (2B)      │
└────────┴────────┴───────────┴──────────┴─────────┴────────────┘
```

#### Header (4 bytes)

```
┌───────────────┬─────────────┬───────────┬───────────┐
│ Protocol Ver. │ Coding Type │ Player ID │ Flags     │
│ (1B)          │ (1B)        │ (1B)      │ (1B)      │
└───────────────┴─────────────┴───────────┴───────────┘
```

- **Protocol Version**: Current voice protocol version
- **Coding Type**: Audio codec and parameters
- **Player ID**: Unique ID of the speaking player
- **Flags**: Special flags (e.g., priority speaker, muted, etc.)

#### Sequence Number (2 bytes)

Sequential packet counter for detecting packet loss and handling jitter.

#### Timestamp (4 bytes)

Relative timestamp in milliseconds for synchronization and jitter buffer management.

#### Channel (1 byte)

Voice channel identifier (global, team, private, etc.).

#### Payload (Variable)

Compressed audio data using the specified codec. Typical payload sizes:

- Opus codec at 20ms frames: ~80-120 bytes per frame
- Range: 10-120 bytes depending on codec and settings

#### Checksum (2 bytes)

CRC16 checksum for basic packet integrity verification.

### Audio Encoding

The voice chat protocol supports multiple audio codecs:

| ID | Codec | Bit Rate | Sample Rate | Frame Size |
|----|-------|----------|-------------|------------|
| 0x01 | Opus | 16-64 kbps | 48 kHz | 20ms |
| 0x02 | Opus | 8-24 kbps | 24 kHz | 20ms |
| 0x03 | Opus | 6-12 kbps | 16 kHz | 20ms |
| 0x04 | Speex | 8-16 kbps | 16 kHz | 20ms |
| 0x05 | Celt | 16-32 kbps | 32 kHz | 20ms |

Codec selection is automatic based on:
- Available bandwidth
- CPU capabilities
- User quality preferences
- Network conditions

### Network Optimization

The voice protocol includes several optimizations:

- **Jitter Buffer**: Dynamic buffer to handle network timing variations
- **Packet Loss Concealment**: Interpolation of missing audio frames
- **FEC (Forward Error Correction)**: Optional redundancy for high-quality mode
- **Dynamic Bitrate Adjustment**: Adapts to changing network conditions
- **Silence Suppression**: Reduced data during silence periods
- **Prioritization**: QoS markings for voice packets

### Voice Activity Detection

The protocol uses voice activity detection to minimize bandwidth:

1. Client-side detection determines when player is speaking
2. Only active voice is transmitted
3. Comfort noise is generated during silence
4. Brief transmission continues after speech ends to avoid clipping

## Security Considerations

### Encryption

All chat communication is encrypted using:

- **Transport Security**: TLS 1.3 for WebSocket/TCP connections
- **Content Encryption**: AES-256-GCM for message payloads
- **Key Exchange**: ECDHE for perfect forward secrecy
- **Authentication**: HMAC-SHA256 for message authentication

### Authentication

Messages are authenticated using:

- **Session Keys**: Generated during game join process
- **Message Signing**: HMAC signature for each message
- **Player Identity**: Verified through game server authentication
- **Anti-Spoofing**: Measures to prevent player impersonation

### Privacy Controls

The protocol implements privacy features:

- **Muting**: Client-side and server-side muting options
- **Blocking**: Prevent communication from specific players
- **Reporting**: Ability to report abusive messages with context
- **Data Minimization**: Limited metadata collection
- **Retention Policy**: Temporary storage of message history

## Bandwidth Optimization

### Text Chat Optimization

Text messages use several bandwidth optimization techniques:

- **Message Batching**: Combine multiple messages when possible
- **Compression**: zlib compression for larger messages
- **Differential Updates**: Send only changes for edited messages
- **Efficient Encoding**: Binary format instead of JSON/XML
- **Incremental History**: Download message history in chunks

### Voice Chat Optimization

Voice transmission is optimized for bandwidth efficiency:

- **Codec Selection**: Choose appropriate codec based on conditions
- **Bitrate Adaptation**: Adjust quality based on available bandwidth
- **Packet Coalescing**: Combine small packets when possible
- **Selective Forwarding**: Only send voice to players who need it
- **Bandwidth Limiter**: Cap total voice bandwidth usage
- **Mixed-mode**: Option to route voice through server when P2P is inefficient

### Bandwidth Usage

Typical bandwidth usage per player:

| Communication Type | Direction | Bandwidth (Average) | Bandwidth (Peak) |
|-------------------|-----------|---------------------|------------------|
| Text Chat | Upload | 0.1-0.5 KB/s | 2-5 KB/s |
| Text Chat | Download | 0.2-1.0 KB/s | 5-10 KB/s |
| Voice Chat | Upload | 5-15 KB/s | 20 KB/s |
| Voice Chat | Download | 5-15 KB/s per active speaker | 20 KB/s per speaker |

## Implementation Details

### Protocol Handlers

The protocol is implemented using Bevy's ECS architecture:

```rust
/// System to handle incoming chat network packets
fn handle_chat_network_packets(
    mut network_receiver: EventReader<NetworkPacketReceived>,
    mut message_events: EventWriter<ChatMessageEvent>,
    session_manager: Res<ChatSessionManager>,
) {
    for packet in network_receiver.read() {
        if packet.protocol_id != CHAT_PROTOCOL_ID {
            continue;
        }
        
        // Verify packet integrity
        if !verify_packet_checksum(&packet.data) {
            // Log error and discard packet
            continue;
        }
        
        // Decrypt message
        let decrypted_data = match decrypt_message(
            &packet.data,
            &session_manager.get_session_key(packet.sender_id),
        ) {
            Ok(data) => data,
            Err(e) => {
                // Log decryption error
                continue;
            }
        };
        
        // Deserialize message
        let message = match deserialize_chat_message(&decrypted_data) {
            Ok(msg) => msg,
            Err(e) => {
                // Log deserialization error
                continue;
            }
        };
        
        // Process message
        message_events.send(ChatMessageEvent {
            message,
            recipients: get_recipients_from_channel(message.channel),
        });
    }
}

/// System to send chat messages over the network
fn send_chat_network_packets(
    mut message_events: EventReader<OutgoingChatMessageEvent>,
    mut network_sender: EventWriter<NetworkPacketSend>,
    session_manager: Res<ChatSessionManager>,
) {
    for event in message_events.read() {
        // Serialize message
        let serialized_data = match serialize_chat_message(&event.message) {
            Ok(data) => data,
            Err(e) => {
                // Log serialization error
                continue;
            }
        };
        
        // Encrypt message for each recipient
        for recipient_id in get_recipient_ids(&event.recipients) {
            let session_key = session_manager.get_session_key(recipient_id);
            
            let encrypted_data = match encrypt_message(&serialized_data, &session_key) {
                Ok(data) => data,
                Err(e) => {
                    // Log encryption error
                    continue;
                }
            };
            
            // Add checksum
            let final_data = add_checksum(&encrypted_data);
            
            // Send packet
            network_sender.send(NetworkPacketSend {
                protocol_id: CHAT_PROTOCOL_ID,
                recipient_id,
                data: final_data,
                reliability: PacketReliability::Reliable,
                channel: NetworkChannel::Chat,
            });
        }
    }
}
```

### Voice Processing Pipeline

The voice chat processing pipeline:

```rust
/// System to process and send voice data
fn process_and_send_voice(
    mut audio_input: ResMut<AudioInputBuffer>,
    mut voice_events: EventWriter<OutgoingVoicePacketEvent>,
    voice_config: Res<VoiceChatConfig>,
    voice_activity: Res<VoiceActivityDetector>,
    encoder: Res<AudioEncoder>,
    local_player: Res<LocalPlayer>,
    time: Res<Time>,
) {
    // Check if player is muted or voice is disabled
    if !voice_config.enabled || voice_config.is_muted {
        return;
    }
    
    // Get audio samples from input buffer
    let raw_samples = audio_input.get_samples(FRAME_SIZE);
    
    // Check for voice activity
    let is_voice_active = voice_config.activation_mode == VoiceActivationMode::AlwaysOn ||
                         (voice_config.activation_mode == VoiceActivationMode::VoiceActivated && 
                          voice_activity.detect_voice(&raw_samples));
    
    if is_voice_active {
        // Apply preprocessing (noise suppression, etc.)
        let processed_samples = preprocess_audio(
            &raw_samples,
            voice_config.noise_suppression_level,
            voice_config.auto_gain_control,
        );
        
        // Encode audio with selected codec
        let encoded_data = encoder.encode(&processed_samples);
        
        // Determine sequence number
        let sequence = next_voice_sequence_number();
        
        // Create voice packet
        let voice_packet = VoicePacket {
            player_id: local_player.id,
            sequence,
            timestamp: time.elapsed_seconds() * 1000.0,
            channel: voice_config.active_channel,
            data: encoded_data,
        };
        
        // Send to network system
        voice_events.send(OutgoingVoicePacketEvent {
            packet: voice_packet,
        });
    }
}
```

### Protocol Constants

Key protocol constants:

```rust
// Protocol identifiers
const CHAT_PROTOCOL_ID: u8 = 0x01;
const VOICE_PROTOCOL_ID: u8 = 0x02;

// Protocol versions
const TEXT_PROTOCOL_VERSION: u8 = 0x01;
const VOICE_PROTOCOL_VERSION: u8 = 0x01;

// Maximum message sizes
const MAX_TEXT_MESSAGE_SIZE: usize = 2048;
const MAX_VOICE_PACKET_SIZE: usize = 512;

// Timing constants
const TEXT_RETRY_INTERVAL_MS: u32 = 500;
const MAX_TEXT_RETRIES: u32 = 5;
const VOICE_FRAME_DURATION_MS: u32 = 20;
const MAX_JITTER_BUFFER_MS: u32 = 200;

// Rate limiting
const MAX_MESSAGES_PER_SECOND: u32 = 5;
const MAX_VOICE_BANDWIDTH_KBPS: u32 = 30;
```

## Error Handling

### Network Errors

The protocol handles various network errors:

- **Connection Loss**: Automatic reconnection with exponential backoff
- **Packet Loss**: Retransmission for text, concealment for voice
- **Latency Spikes**: Jitter buffer for voice, acknowledge timeouts for text
- **Fragmentation**: Message reassembly for large text messages
- **MTU Limits**: Automatic fragmentation for oversized packets

### Protocol Errors

Handling of protocol-level errors:

- **Version Mismatch**: Negotiation of compatible protocol version
- **Malformed Messages**: Proper error logging and discarding
- **Checksum Failures**: Retransmission requests for critical messages
- **Decryption Failures**: Session renegotiation if persistent
- **Sequence Gaps**: Reordering or retransmission as appropriate

### Error Reporting

Error metrics collected for monitoring:

- **Packet Loss Rate**: Percentage of packets not received
- **Retry Rate**: Frequency of message retransmissions
- **Decryption Failures**: Count of failed decryption attempts
- **Latency**: Round-trip time for acknowledgments
- **Jitter**: Variance in packet arrival times

These metrics are used to dynamically adjust protocol parameters for optimal performance. 