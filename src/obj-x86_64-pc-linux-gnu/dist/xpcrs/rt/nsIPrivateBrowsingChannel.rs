//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIPrivateBrowsingChannel.idl
//


/// `interface nsIPrivateBrowsingChannel : nsISupports`
///

/// ```text
/// /**
///  * This interface is implemented by channels which support overriding the
///  * privacy state of the channel.
///  *
///  * This interface must be used only from the XPCOM main thread.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPrivateBrowsingChannel {
    vtable: *const nsIPrivateBrowsingChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPrivateBrowsingChannel.
unsafe impl XpCom for nsIPrivateBrowsingChannel {
    const IID: nsIID = nsID(0xdf702bb0, 0x55b8, 0x11e2,
        [0xbc, 0xfd, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPrivateBrowsingChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPrivateBrowsingChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPrivateBrowsingChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIPrivateBrowsingChannel`.
    fn coerce_from(v: &nsIPrivateBrowsingChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPrivateBrowsingChannelCoerce for nsIPrivateBrowsingChannel {
    #[inline]
    fn coerce_from(v: &nsIPrivateBrowsingChannel) -> &Self {
        v
    }
}

impl nsIPrivateBrowsingChannel {
    /// Cast this `nsIPrivateBrowsingChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPrivateBrowsingChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPrivateBrowsingChannel {
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
impl<T: nsISupportsCoerce> nsIPrivateBrowsingChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPrivateBrowsingChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPrivateBrowsingChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPrivateBrowsingChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setPrivate (in boolean aPrivate); */
    pub SetPrivate: unsafe extern "system" fn (this: *const nsIPrivateBrowsingChannel, aPrivate: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean isChannelPrivate; */
    pub GetIsChannelPrivate: unsafe extern "system" fn (this: *const nsIPrivateBrowsingChannel, aIsChannelPrivate: *mut bool) -> ::nserror::nsresult,

    /* [noscript] boolean isPrivateModeOverriden (out boolean aValue); */
    pub IsPrivateModeOverriden: unsafe extern "system" fn (this: *const nsIPrivateBrowsingChannel, aValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPrivateBrowsingChannel {

    /// ```text
    /// /**
    ///      * Determine whether the channel is tied to a private browsing window.
    ///      *
    ///      * This value can be set only before the channel is opened.  Setting it
    ///      * after that does not have any effect.  This value overrides the privacy
    ///      * state of the channel, which means that if you call this method, then
    ///      * the loadGroup and load context will no longer be consulted when we
    ///      * need to know the private mode status for a channel.
    ///      *
    ///      * Note that this value is only meant to be used when the channel's privacy
    ///      * status cannot be obtained from the loadGroup or load context (for
        ///      * example, when the channel is not associated with any loadGroup or load
        ///      * context.)  Setting this value directly should be avoided if possible.
    ///      *
    ///      * Implementations must enforce the ordering semantics of this function by
    ///      * raising errors if setPrivate is called on a channel which has a loadGroup
    ///      * and/or callbacks that implement nsILoadContext, or if the loadGroup
    ///      * or notificationCallbacks are set after setPrivate has been called.
    ///      *
    ///      * @param aPrivate whether the channel should be opened in private mode.
    ///      */
    /// ```
    ///

    /// `void setPrivate (in boolean aPrivate);`
    #[inline]
    pub unsafe fn SetPrivate(&self, aPrivate: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPrivate)(self, aPrivate)
    }


    /// ```text
    /// /**
    ///      * States whether the channel is in private browsing mode. This may either
    ///      * happen because the channel is opened from a private mode context or
    ///      * when the mode is explicitly set with ::setPrivate().
    ///      *
    ///      * This attribute is equivalent to NS_UsePrivateBrowsing(), but scriptable.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean isChannelPrivate;`
    #[inline]
    pub unsafe fn GetIsChannelPrivate(&self, aIsChannelPrivate: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsChannelPrivate)(self, aIsChannelPrivate)
    }



    /// `[noscript] boolean isPrivateModeOverriden (out boolean aValue);`
    #[inline]
    pub unsafe fn IsPrivateModeOverriden(&self, aValue: *mut bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsPrivateModeOverriden)(self, aValue, _retval)
    }


}


