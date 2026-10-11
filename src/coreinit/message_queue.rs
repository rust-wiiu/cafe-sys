//! Message queue

use super::thread_queue::ThreadQueue;
use crate::ffi;

/// OS_MESSAGE_QUEUE_TAG
///
/// Value of [MessageQueue::tag].
pub const MESSAGE_QUEUE_TAG: u32 = 0x6D536751;

/// OSMessageFlags
#[doc(alias = "OSMessageFlags")]
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageFlags(pub i32);

impl MessageFlags {
    pub const NONE: Self = Self(0);
    /// Waits for space or a message.
    pub const BLOCKING: Self = Self(1 << 0);
    /// Sends to the front of the queue.
    pub const HIGH_PRIORITY: Self = Self(1 << 1);
}

impl core::ops::BitOr for MessageFlags {
    type Output = Self;

    #[inline]
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

/// Value of [Message::args]`[0]` for messages of [get_system_message_queue].
#[repr(transparent)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SystemMessage(pub u32);

impl SystemMessage {
    pub const RELEASE_FOREGROUND: Self = Self(0xFACEBACC);
    pub const ACQUIRED_FOREGROUND: Self = Self(0xFACEF000);
    pub const EXIT: Self = Self(0xD1E0D1E0);
    pub const NETIO_CHANGE: Self = Self(0xAAC0FFEE);
    pub const HOME_BUTTON_DENIED: Self = Self(0xCCC0FFEE);
    pub const ROOT_RECYCLE: Self = Self(0xAAFECAAA);
}

/// OSMessage
#[doc(alias = "OSMessage")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Message {
    pub message: *mut ffi::c_void,
    pub args: [u32; 3],
}

impl Default for Message {
    fn default() -> Self {
        Self {
            message: core::ptr::null_mut(),
            args: [0; 3],
        }
    }
}

/// OSMessageQueue
#[doc(alias = "OSMessageQueue")]
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MessageQueue {
    /// [MESSAGE_QUEUE_TAG].
    pub tag: u32,
    pub name: *const ffi::c_char,
    _reserved: ffi::unknown<4>,
    pub send_queue: ThreadQueue,
    pub recv_queue: ThreadQueue,
    pub messages: *mut Message,
    pub size: i32,
    pub first: i32,
    pub used: i32,
}

unsafe extern "C" {
    /// OSInitMessageQueue
    ///
    /// `messages` holds `size` messages and must stay valid.
    #[doc(alias = "OSInitMessageQueue")]
    #[link_name = "OSInitMessageQueue"]
    pub unsafe fn init(queue: *mut MessageQueue, messages: *mut Message, size: i32);

    /// OSInitMessageQueueEx
    #[doc(alias = "OSInitMessageQueueEx")]
    #[link_name = "OSInitMessageQueueEx"]
    pub unsafe fn init_ex(
        queue: *mut MessageQueue,
        messages: *mut Message,
        size: i32,
        name: *const ffi::c_char,
    );

    /// OSSendMessage
    ///
    /// Returns false if the queue is full and [MessageFlags::BLOCKING] is not set.
    #[doc(alias = "OSSendMessage")]
    #[link_name = "OSSendMessage"]
    pub unsafe fn send(
        queue: *mut MessageQueue,
        message: *mut Message,
        flags: MessageFlags,
    ) -> ffi::c_bool;

    /// OSReceiveMessage
    ///
    /// Returns false if the queue is empty and [MessageFlags::BLOCKING] is not set.
    #[doc(alias = "OSReceiveMessage")]
    #[link_name = "OSReceiveMessage"]
    pub unsafe fn receive(
        queue: *mut MessageQueue,
        message: *mut Message,
        flags: MessageFlags,
    ) -> ffi::c_bool;

    /// OSPeekMessage
    ///
    /// Same as [receive] without removing the message or blocking.
    #[doc(alias = "OSPeekMessage")]
    #[link_name = "OSPeekMessage"]
    pub unsafe fn peek(queue: *mut MessageQueue, message: *mut Message) -> ffi::c_bool;

    /// OSGetSystemMessageQueue
    ///
    /// Receives [SystemMessage]s.
    #[doc(alias = "OSGetSystemMessageQueue")]
    #[link_name = "OSGetSystemMessageQueue"]
    pub unsafe fn get_system_message_queue() -> *mut MessageQueue;

    /// OSGetDefaultAppIOQueue
    #[doc(alias = "OSGetDefaultAppIOQueue")]
    #[link_name = "OSGetDefaultAppIOQueue"]
    pub unsafe fn get_default_app_io_queue() -> *mut MessageQueue;
}
