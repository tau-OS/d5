//! # DBus interface proxy for: `org.freedesktop.Notifications`
//!
//! This code was generated by `zbus-xmlgen` `3.1.0` from DBus introspection data.
//! Source: `Interface '/org/freedesktop/Notifications' from service 'org.freedesktop.Notifications' on session bus`.
//!
//! You may prefer to adapt it, instead of using it verbatim.
//!
//! More information can be found in the
//! [Writing a client proxy](https://dbus.pages.freedesktop.org/zbus/client.html)
//! section of the zbus documentation.
//!
//! This DBus object implements
//! [standard DBus interfaces](https://dbus.freedesktop.org/doc/dbus-specification.html),
//! (`org.freedesktop.DBus.*`) for which the following zbus proxies can be used:
//!
//! * [`zbus::fdo::PropertiesProxy`]
//! * [`zbus::fdo::IntrospectableProxy`]
//! * [`zbus::fdo::PeerProxy`]
//!
//! …consequently `zbus-xmlgen` did not generate code for the above interfaces.

use zbus::dbus_proxy;

#[dbus_proxy(
    interface = "org.freedesktop.Notifications",
    default_service = "org.freedesktop.Notifications",
    default_path = "/org/freedesktop/Notifications"
)]
trait Notifications {
    /// CloseNotification method
    fn close_notification(&self, arg_0: u32) -> zbus::Result<()>;

    /// GetCapabilities method
    fn get_capabilities(&self) -> zbus::Result<Vec<String>>;

    /// GetServerInformation method
    fn get_server_information(&self) -> zbus::Result<(String, String, String, String)>;

    /// Notify method
    fn notify(
        &self,
        arg_0: &str,
        arg_1: u32,
        arg_2: &str,
        arg_3: &str,
        arg_4: &str,
        arg_5: &[&str],
        arg_6: std::collections::HashMap<&str, zbus::zvariant::Value<'_>>,
        arg_7: i32,
    ) -> zbus::Result<u32>;

    /// ActionInvoked signal
    #[dbus_proxy(signal)]
    fn action_invoked(&self, arg_0: u32, arg_1: &str) -> zbus::Result<()>;

    /// NotificationClosed signal
    #[dbus_proxy(signal)]
    fn notification_closed(&self, arg_0: u32, arg_1: u32) -> zbus::Result<()>;
}
