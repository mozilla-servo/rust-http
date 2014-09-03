//! SSL client support.
//!
//! Which particular library is used depends upon the configuration used at
//! compile time; at present it can only be OpenSSL (`--cfg openssl`); without
//! that, you won't be able to use SSL (an attempt to make an HTTPS connection
//! will return an error).

#[cfg(not(nossl),not(target_os="android"))]
pub use self::openssl::NetworkStream;
#[cfg(nossl)]
#[cfg(target_os="android")]
pub use self::none::NetworkStream;

#[cfg(not(nossl),not(target_os="android"))]
mod openssl;
#[cfg(nossl)]
#[cfg(target_os="android")]
mod none;
