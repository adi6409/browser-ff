//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIConsoleMessage.idl
//


/// `interface nsIConsoleMessage : nsISupports`
///

/// ```text
/// /**
///  * This is intended as a base interface; implementations may want to
///  * provide an object that can be qi'ed to provide more specific
///  * message information.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIConsoleMessage {
    vtable: *const nsIConsoleMessageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIConsoleMessage.
unsafe impl XpCom for nsIConsoleMessage {
    const IID: nsIID = nsID(0x3aba9617, 0x10e2, 0x4839,
        [0x83, 0xae, 0x2e, 0x6f, 0xc4, 0xdf, 0x42, 0x8b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIConsoleMessage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIConsoleMessage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIConsoleMessageCoerce {
    /// Cheaply cast a value of this type from a `nsIConsoleMessage`.
    fn coerce_from(v: &nsIConsoleMessage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIConsoleMessageCoerce for nsIConsoleMessage {
    #[inline]
    fn coerce_from(v: &nsIConsoleMessage) -> &Self {
        v
    }
}

impl nsIConsoleMessage {
    /// Cast this `nsIConsoleMessage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIConsoleMessageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIConsoleMessage {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIConsoleMessageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIConsoleMessage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIConsoleMessage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIConsoleMessageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute uint32_t logLevel; */
    pub GetLogLevel: unsafe extern "system" fn (this: *const nsIConsoleMessage, aLogLevel: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute long long timeStamp; */
    pub GetTimeStamp: unsafe extern "system" fn (this: *const nsIConsoleMessage, aTimeStamp: *mut i64) -> ::nserror::nsresult,

    /* [binaryname(MessageMoz)] readonly attribute AString message; */
    pub GetMessageMoz: unsafe extern "system" fn (this: *const nsIConsoleMessage, aMessage: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AUTF8String toString (); */
    pub ToString: unsafe extern "system" fn (this: *const nsIConsoleMessage, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIConsoleMessage {
    /// ```text
    /// /** Log level constants. */
    /// ```
    ///

    pub const debug: i64 = 0;


    pub const info: i64 = 1;


    pub const warn: i64 = 2;


    pub const error: i64 = 3;

    /// ```text
    /// /**
    ///      * The log level of this message.
    ///      */
    /// ```
    ///

    /// `readonly attribute uint32_t logLevel;`
    #[inline]
    pub unsafe fn GetLogLevel(&self, aLogLevel: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetLogLevel)(self, aLogLevel)
    }


    /// ```text
    /// /**
    ///      * The time (in milliseconds from the Epoch) that the message instance
    ///      * was initialised.
    ///      * The timestamp is initialized as JS_now/1000 so that it can be
    ///      * compared to Date.now in Javascript.
    ///      */
    /// ```
    ///

    /// `readonly attribute long long timeStamp;`
    #[inline]
    pub unsafe fn GetTimeStamp(&self, aTimeStamp: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetTimeStamp)(self, aTimeStamp)
    }



    /// `[binaryname(MessageMoz)] readonly attribute AString message;`
    #[inline]
    pub unsafe fn GetMessageMoz(&self, aMessage: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetMessageMoz)(self, aMessage)
    }



    /// `AUTF8String toString ();`
    #[inline]
    pub unsafe fn ToString(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ToString)(self, _retval)
    }


}


