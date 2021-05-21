//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/ftp/nsIFTPChannel.idl
//


/// `interface nsIFTPChannel : nsISupports`
///

/// ```text
/// /**
///  * This interface may be used to determine if a channel is a FTP channel.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFTPChannel {
    vtable: *const nsIFTPChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFTPChannel.
unsafe impl XpCom for nsIFTPChannel {
    const IID: nsIID = nsID(0x07f0d5cd, 0x1fd5, 0x4aa3,
        [0xb6, 0xfc, 0x66, 0x5b, 0xdc, 0x5d, 0xbf, 0x9f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFTPChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFTPChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFTPChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIFTPChannel`.
    fn coerce_from(v: &nsIFTPChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFTPChannelCoerce for nsIFTPChannel {
    #[inline]
    fn coerce_from(v: &nsIFTPChannel) -> &Self {
        v
    }
}

impl nsIFTPChannel {
    /// Cast this `nsIFTPChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFTPChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFTPChannel {
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
impl<T: nsISupportsCoerce> nsIFTPChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFTPChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFTPChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFTPChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute PRTime lastModifiedTime; */
    pub GetLastModifiedTime: unsafe extern "system" fn (this: *const nsIFTPChannel, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult,

    /* attribute PRTime lastModifiedTime; */
    pub SetLastModifiedTime: unsafe extern "system" fn (this: *const nsIFTPChannel, aLastModifiedTime: PRTime) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFTPChannel {


    /// `attribute PRTime lastModifiedTime;`
    #[inline]
    pub unsafe fn GetLastModifiedTime(&self, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModifiedTime)(self, aLastModifiedTime)
    }



    /// `attribute PRTime lastModifiedTime;`
    #[inline]
    pub unsafe fn SetLastModifiedTime(&self, aLastModifiedTime: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).SetLastModifiedTime)(self, aLastModifiedTime)
    }


}


/// `interface nsIFTPEventSink : nsISupports`
///

/// ```text
/// /**
///  * This interface may be defined as a notification callback on the FTP
///  * channel.  It allows a consumer to receive a log of the FTP control
///  * connection conversation.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFTPEventSink {
    vtable: *const nsIFTPEventSinkVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFTPEventSink.
unsafe impl XpCom for nsIFTPEventSink {
    const IID: nsIID = nsID(0x455d4234, 0x0330, 0x43d2,
        [0xbb, 0xfb, 0x99, 0xaf, 0xbe, 0xcb, 0xfe, 0xb0]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFTPEventSink {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFTPEventSink.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFTPEventSinkCoerce {
    /// Cheaply cast a value of this type from a `nsIFTPEventSink`.
    fn coerce_from(v: &nsIFTPEventSink) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFTPEventSinkCoerce for nsIFTPEventSink {
    #[inline]
    fn coerce_from(v: &nsIFTPEventSink) -> &Self {
        v
    }
}

impl nsIFTPEventSink {
    /// Cast this `nsIFTPEventSink` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFTPEventSinkCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFTPEventSink {
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
impl<T: nsISupportsCoerce> nsIFTPEventSinkCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFTPEventSink) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFTPEventSink
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFTPEventSinkVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void OnFTPControlLog (in boolean server, in string msg); */
    pub OnFTPControlLog: unsafe extern "system" fn (this: *const nsIFTPEventSink, server: bool, msg: *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFTPEventSink {

    /// ```text
    /// /**
    ///      * XXX document this method!  (see bug 328915)
    ///      */
    /// ```
    ///

    /// `void OnFTPControlLog (in boolean server, in string msg);`
    #[inline]
    pub unsafe fn OnFTPControlLog(&self, server: bool, msg: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).OnFTPControlLog)(self, server, msg)
    }


}


