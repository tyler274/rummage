# Authentication in MTG Commander Game Engine

This document outlines the authentication mechanisms used in the MTG Commander game engine's multiplayer implementation.

## Overview

Authentication is a critical component of the networking system, ensuring that only authorized users can connect to and participate in games. The system uses bevy_replicon along with custom authentication logic to establish secure connections.

## Authentication Flow

1. **User Login**: Players begin by providing credentials or connecting via a trusted third-party service
2. **Credential Validation**: Server validates credentials against stored records or third-party responses
3. **Session Token Generation**: Upon successful validation, a unique session token is generated
4. **Client Authentication**: The client uses this token for all subsequent communications
5. **Session Management**: Server tracks active sessions and handles expiration/revocation

## Implementation Details

### Server-Side Authentication

```rust
// Authentication handler on server
fn handle_authentication(
    mut commands: Commands,
    mut server: ResMut<ReplicionServer>,
    mut auth_events: EventReader<AuthenticationEvent>,
) {
    for event in auth_events.read() {
        // Validate credentials
        if validate_credentials(&event.credentials) {
            // Generate session token
            let session_token = generate_session_token();
            
            // Store session information
            commands.spawn((
                SessionData {
                    user_id: event.user_id,
                    token: session_token.clone(),
                    expiration: Utc::now() + Duration::hours(24),
                },
                Replicated,
            ));
            
            // Send authentication response
            server.send_message(event.client_id, AuthSuccess { token: session_token });
        } else {
            // Failed authentication
            server.send_message(event.client_id, AuthFailure { reason: "Invalid credentials".to_string() });
        }
    }
}
```

### Client-Side Authentication

```rust
// Client authentication system
fn authenticate_client(
    mut client: ResMut<RepliconClient>,
    mut auth_state: ResMut<AuthenticationState>,
    keyboard: Res<Input<KeyCode>>,
    mut ui_state: ResMut<UiState>,
) {
    // Handle authentication flow based on state
    match *auth_state {
        AuthenticationState::NotAuthenticated => {
            if ui_state.credentials_ready {
                // Send credentials to server
                client.send_message(AuthRequest {
                    username: ui_state.username.clone(),
                    password: ui_state.password.clone(),
                });
                *auth_state = AuthenticationState::Authenticating;
            }
        },
        AuthenticationState::Authenticating => {
            // Wait for server response (handled in another system)
        },
        AuthenticationState::Authenticated => {
            // Authentication complete, proceed to lobby or game
        }
    }
}
```

## Security Considerations

### Token Security

- Session tokens are cryptographically secure random values
- Tokens are transmitted securely and stored with appropriate protections
- Token lifetimes are limited and can be revoked if suspicious activity is detected

### Password Security

- Passwords are never stored in plaintext
- Argon2id hashing with appropriate parameters is used for password storage
- Password policy enforces minimum complexity requirements

### Protection Against Common Attacks

- Rate limiting is implemented to prevent brute force attempts
- Protection against replay attacks using token rotation and nonces
- Secure communication channel to prevent man-in-the-middle attacks

## Future Enhancements

- OAuth integration for third-party authentication
- Two-factor authentication for additional security
- Hardware token support for high-security environments
- Biometric authentication for supported platforms

## Testing the Authentication System

Comprehensive testing is critical for authentication systems. See the [Security Testing Strategy](../testing/security/strategy.md) for details on the testing approach for authentication.

---

This documentation will be updated as the authentication system evolves.
