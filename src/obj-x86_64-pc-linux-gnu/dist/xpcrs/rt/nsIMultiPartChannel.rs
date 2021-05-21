//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIMultiPartChannel.idl
//


/// `interface nsIMultiPartChannel : nsISupports`
///

/// ```text
/// /**
///  * An interface to access the the base channel
///  * associated with a MultiPartChannel.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMultiPartChannel {
    vtable: *const nsIMultiPartChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMultiPartChannel.
unsafe impl XpCom for nsIMultiPartChannel {
    const IID: nsIID = nsID(0x4fefb490, 0x5567, 0x11e5,
        [0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMultiPartChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMultiPartChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMultiPartChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIMultiPartChannel`.
    fn coerce_from(v: &nsIMultiPartChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMultiPartChannelCoerce for nsIMultiPartChannel {
    #[inline]
    fn coerce_from(v: &nsIMultiPartChannel) -> &Self {
        v
    }
}

impl nsIMultiPartChannel {
    /// Cast this `nsIMultiPartChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMultiPartChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMultiPartChannel {
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
impl<T: nsISupportsCoerce> nsIMultiPartChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMultiPartChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMultiPartChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMultiPartChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIChannel baseChannel; */
    pub GetBaseChannel: unsafe extern "system" fn (this: *const nsIMultiPartChannel, aBaseChannel: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* readonly attribute uint32_t partID; */
    pub GetPartID: unsafe extern "system" fn (this: *const nsIMultiPartChannel, aPartID: *mut uint32_t) -> ::nserror::nsresult,

    /* readonly attribute boolean isLastPart; */
    pub GetIsLastPart: unsafe extern "system" fn (this: *const nsIMultiPartChannel, aIsLastPart: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMultiPartChannel {

    /// ```text
    /// /**
    ///      * readonly attribute to access the underlying channel
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIChannel baseChannel;`
    #[inline]
    pub unsafe fn GetBaseChannel(&self, aBaseChannel: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetBaseChannel)(self, aBaseChannel)
    }


    /// ```text
    /// /**
    ///      * Attribute guaranteed to be different for different parts of
    ///      * the same multipart document.
    ///      */
    /// ```
    ///

    /// `readonly attribute uint32_t partID;`
    #[inline]
    pub unsafe fn GetPartID(&self, aPartID: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetPartID)(self, aPartID)
    }


    /// ```text
    /// /**
    ///      * Set to true when onStopRequest is received from the base channel.
    ///      * The listener can check this from its onStopRequest to determine
    ///      * whether more data can be expected.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isLastPart;`
    #[inline]
    pub unsafe fn GetIsLastPart(&self, aIsLastPart: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsLastPart)(self, aIsLastPart)
    }


}


/// `interface nsIMultiPartChannelListener : nsISupports`
///

/// ```text
/// /**
///  * An interface that listeners can implement to receive a notification
///  * when the last part of the multi-part channel has finished, and the
///  * final OnStopRequest has been sent.
///   */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMultiPartChannelListener {
    vtable: *const nsIMultiPartChannelListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMultiPartChannelListener.
unsafe impl XpCom for nsIMultiPartChannelListener {
    const IID: nsIID = nsID(0xb084959a, 0x4fb9, 0x41a5,
        [0x88, 0xa0, 0xd0, 0xf0, 0x45, 0xce, 0x75, 0xcf]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMultiPartChannelListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMultiPartChannelListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMultiPartChannelListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIMultiPartChannelListener`.
    fn coerce_from(v: &nsIMultiPartChannelListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMultiPartChannelListenerCoerce for nsIMultiPartChannelListener {
    #[inline]
    fn coerce_from(v: &nsIMultiPartChannelListener) -> &Self {
        v
    }
}

impl nsIMultiPartChannelListener {
    /// Cast this `nsIMultiPartChannelListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMultiPartChannelListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMultiPartChannelListener {
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
impl<T: nsISupportsCoerce> nsIMultiPartChannelListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMultiPartChannelListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMultiPartChannelListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMultiPartChannelListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onAfterLastPart (in nsresult status); */
    pub OnAfterLastPart: unsafe extern "system" fn (this: *const nsIMultiPartChannelListener, status: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMultiPartChannelListener {

    /// ```text
    /// /**
    ///      * Sent when all parts have finished and sent OnStopRequest.
    ///      */
    /// ```
    ///

    /// `void onAfterLastPart (in nsresult status);`
    #[inline]
    pub unsafe fn OnAfterLastPart(&self, status: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).OnAfterLastPart)(self, status)
    }


}


