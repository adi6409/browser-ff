//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIRedirectChannelRegistrar.idl
//


/// `interface nsIRedirectChannelRegistrar : nsISupports`
///

/// ```text
/// /**
///  * Used on the chrome process as a service to join channel implementation
///  * and parent IPC protocol side under a unique id.  Provides this way a generic
///  * communication while redirecting to various protocols.
///  *
///  * See also nsIChildChannel and nsIParentChannel.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRedirectChannelRegistrar {
    vtable: *const nsIRedirectChannelRegistrarVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRedirectChannelRegistrar.
unsafe impl XpCom for nsIRedirectChannelRegistrar {
    const IID: nsIID = nsID(0xefa36ea2, 0x5b07, 0x46fc,
        [0x95, 0x34, 0xa5, 0xac, 0xb8, 0xb7, 0x7b, 0x72]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRedirectChannelRegistrar {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRedirectChannelRegistrar.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRedirectChannelRegistrarCoerce {
    /// Cheaply cast a value of this type from a `nsIRedirectChannelRegistrar`.
    fn coerce_from(v: &nsIRedirectChannelRegistrar) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRedirectChannelRegistrarCoerce for nsIRedirectChannelRegistrar {
    #[inline]
    fn coerce_from(v: &nsIRedirectChannelRegistrar) -> &Self {
        v
    }
}

impl nsIRedirectChannelRegistrar {
    /// Cast this `nsIRedirectChannelRegistrar` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRedirectChannelRegistrarCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRedirectChannelRegistrar {
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
impl<T: nsISupportsCoerce> nsIRedirectChannelRegistrarCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRedirectChannelRegistrar) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRedirectChannelRegistrar
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRedirectChannelRegistrarVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void registerChannel (in nsIChannel channel, in uint64_t id); */
    pub RegisterChannel: unsafe extern "system" fn (this: *const nsIRedirectChannelRegistrar, channel: *const nsIChannel, id: uint64_t) -> ::nserror::nsresult,

    /* nsIChannel linkChannels (in uint64_t id, in nsIParentChannel channel); */
    pub LinkChannels: unsafe extern "system" fn (this: *const nsIRedirectChannelRegistrar, id: uint64_t, channel: *const nsIParentChannel, _retval: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* nsIChannel getRegisteredChannel (in uint64_t id); */
    pub GetRegisteredChannel: unsafe extern "system" fn (this: *const nsIRedirectChannelRegistrar, id: uint64_t, _retval: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* nsIParentChannel getParentChannel (in uint64_t id); */
    pub GetParentChannel: unsafe extern "system" fn (this: *const nsIRedirectChannelRegistrar, id: uint64_t, _retval: *mut*const nsIParentChannel) -> ::nserror::nsresult,

    /* void deregisterChannels (in uint64_t id); */
    pub DeregisterChannels: unsafe extern "system" fn (this: *const nsIRedirectChannelRegistrar, id: uint64_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRedirectChannelRegistrar {

    /// ```text
    /// /**
    ///    * Register the redirect target channel. The passed id needs to be a
    ///    * unique ID for that channel (see `nsContentUtils::GenerateLoadIdentifier`).
    ///    *
    ///    * Primarily used in ParentChannelListener::AsyncOnChannelRedirect to get
    ///    * a channel id sent to the HttpChannelChild being redirected.
    ///    */
    /// ```
    ///

    /// `void registerChannel (in nsIChannel channel, in uint64_t id);`
    #[inline]
    pub unsafe fn RegisterChannel(&self, channel: *const nsIChannel, id: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).RegisterChannel)(self, channel, id)
    }


    /// ```text
    /// /**
    ///    * First, search for the channel registered under the id.  If found return
    ///    * it.  Then, register under the same id the parent side of IPC protocol
    ///    * to let it be later grabbed back by the originator of the redirect and
    ///    * notifications from the real channel could be forwarded to this parent
    ///    * channel.
    ///    *
    ///    * Primarily used in parent side of an IPC protocol implementation
    ///    * in reaction to nsIChildChannel.connectParent(id) called from the child
    ///    * process.
    ///    */
    /// ```
    ///

    /// `nsIChannel linkChannels (in uint64_t id, in nsIParentChannel channel);`
    #[inline]
    pub unsafe fn LinkChannels(&self, id: uint64_t, channel: *const nsIParentChannel, _retval: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).LinkChannels)(self, id, channel, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns back the channel previously registered under the ID with
    ///    * registerChannel method.
    ///    *
    ///    * Primarilly used in chrome IPC side of protocols when attaching a redirect
    ///    * target channel to an existing 'real' channel implementation.
    ///    */
    /// ```
    ///

    /// `nsIChannel getRegisteredChannel (in uint64_t id);`
    #[inline]
    pub unsafe fn GetRegisteredChannel(&self, id: uint64_t, _retval: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetRegisteredChannel)(self, id, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the stream listener that shall be attached to the redirect target
    ///    * channel, all notification from the redirect target channel will be
    ///    * forwarded to this stream listener.
    ///    *
    ///    * Primarilly used in ParentChannelListener::OnRedirectResult callback
    ///    * to grab the created parent side of the channel and forward notifications
    ///    * to it.
    ///    */
    /// ```
    ///

    /// `nsIParentChannel getParentChannel (in uint64_t id);`
    #[inline]
    pub unsafe fn GetParentChannel(&self, id: uint64_t, _retval: *mut*const nsIParentChannel) -> ::nserror::nsresult {
        ((*self.vtable).GetParentChannel)(self, id, _retval)
    }


    /// ```text
    /// /**
    ///    * To not force all channel implementations to support weak reference
    ///    * consumers of this service must ensure release of registered channels them
    ///    * self.  This releases both the real and parent channel registered under
    ///    * the id.
    ///    *
    ///    * Primarilly used in ParentChannelListener::OnRedirectResult callback.
    ///    */
    /// ```
    ///

    /// `void deregisterChannels (in uint64_t id);`
    #[inline]
    pub unsafe fn DeregisterChannels(&self, id: uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).DeregisterChannels)(self, id)
    }


}


