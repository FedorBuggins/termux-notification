//! ## Todo
//!
//! - [] fix doc comments
//! - [] implement all options

#![allow(dead_code)]

/// Action to execute when pressing the notification
pub(crate) const ACTION: &str = "--action";

/// Do not alert when the notification is edited
pub(crate) const ALERT_ONCE: &str = "--alert-once";

/// Text to show on the first notification button
pub(crate) const BUTTON1: &str = "--button1";

/// Action to execute on the first notification button
pub(crate) const BUTTON1_ACTION: &str = "--button1-action";

/// Text to show on the second notification button
pub(crate) const BUTTON2: &str = "--button2";

/// Action to execute on the second notification button
pub(crate) const BUTTON2_ACTION: &str = "--button2-action";

/// Text to show on the third notification button
pub(crate) const BUTTON3: &str = "--button3";

/// Action to execute on the third notification button
pub(crate) const BUTTON3_ACTION: &str = "--button3-action";

/// Content to show in the notification.
pub(crate) const CONTENT: &str = "--content";

/// specifies the notification channel id this notification should be send on.
pub(crate) const CHANNEL: &str = "--channel";

/// Notification group (notifications with the same
pub(crate) const GROUP: &str = "--group";

/// Notification id (will overwrite any previous notification with the same id)
pub(crate) const ID: &str = "--id";

/// Set the icon that shows up in the status bar.
/// View available icons at `https://material.io/resources/icons/`
/// (default icon: `event_note`)
pub(crate) const ICON: &str = "--icon";

/// Absolute path to an image which will be shown in the
pub(crate) const IMAGE_PATH: &str = "--image-path";

/// Color of the blinking led as RRGGBB (default: none)
pub(crate) const LED_COLOR: &str = "--led-color";

/// Number of milliseconds for the LED to be off while
pub(crate) const LED_OFF: &str = "--led-off";

/// Number of milliseconds for the LED to be on while
pub(crate) const LED_ON: &str = "--led-on";

/// Action to execute when the the notification is cleared
pub(crate) const ON_DELETE: &str = "--on-delete";

/// Pin the notification
pub(crate) const ONGOING: &str = "--ongoing";

/// Notification priority (high/low/max/min/default)
pub(crate) const PRIORITY: &str = "--priority";

/// Play a sound with the notification
pub(crate) const SOUND: &str = "--sound";

/// Notification title to show
pub(crate) const TITLE: &str = "--title";

/// Vibrate pattern, comma separated as in 500,1000,200
pub(crate) const VIBRATE: &str = "--vibrate";

/// Notification style to use (default/media)
pub(crate) const TYPE: &str = "--type";

/// Action to execute on the media-next button
pub(crate) const MEDIA_NEXT: &str = "--media-next";

/// Action to execute on the media-pause button
pub(crate) const MEDIA_PAUSE: &str = "--media-pause";

/// Action to execute on the media-play button
pub(crate) const MEDIA_PLAY: &str = "--media-play";

/// Action to execute on the media-previous button
pub(crate) const MEDIA_PREVIOUS: &str = "--media-previous";
