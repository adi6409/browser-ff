//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIParentChannel.idl
//


/// `interface nsIParentChannel : nsIStreamListener`
///

/// ```text
/// /**
///  * Implemented by chrome side of IPC protocols.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIParentChannel {
    vtable: *const nsIParentChannelVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIParentChannel.
unsafe impl XpCom for nsIParentChannel {
    const IID: nsIID = nsID(0xe0fc4801, 0x6030, 0x4653,
        [0xa5, 0x9f, 0x1f, 0xb2, 0x82, 0xbd, 0x1a, 0x04]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIParentChannel {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIParentChannel.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIParentChannelCoerce {
    /// Cheaply cast a value of this type from a `nsIParentChannel`.
    fn coerce_from(v: &nsIParentChannel) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIParentChannelCoerce for nsIParentChannel {
    #[inline]
    fn coerce_from(v: &nsIParentChannel) -> &Self {
        v
    }
}

impl nsIParentChannel {
    /// Cast this `nsIParentChannel` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIParentChannelCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIParentChannel {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIStreamListenerCoerce> nsIParentChannelCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIParentChannel) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIParentChannel
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIParentChannelVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamListenerVTable,

    /* [noscript] void setParentListener (in ParentChannelListener listener); */
    /// Unable to generate binding because `native type mozilla::net::ParentChannelListener unsupported`
    pub SetParentListener: *const ::libc::c_void,

    /* [noscript] void notifyFlashPluginStateChanged (in nsIHttpChannel_FlashPluginState aState); */
    pub NotifyFlashPluginStateChanged: unsafe extern "system" fn (this: *const nsIParentChannel, aState:  u8) -> ::nserror::nsresult,

    /* [noscript] void setClassifierMatchedInfo (in ACString aList, in ACString aProvider, in ACString aFullHash); */
    pub SetClassifierMatchedInfo: unsafe extern "system" fn (this: *const nsIParentChannel, aList: *const ::nsstring::nsACString, aProvider: *const ::nsstring::nsACString, aFullHash: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void setClassifierMatchedTrackingInfo (in ACString aLists, in ACString aFullHashes); */
    pub SetClassifierMatchedTrackingInfo: unsafe extern "system" fn (this: *const nsIParentChannel, aLists: *const ::nsstring::nsACString, aFullHashes: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void notifyClassificationFlags (in uint32_t aClassificationFlags, in bool aIsThirdParty); */
    pub NotifyClassificationFlags: unsafe extern "system" fn (this: *const nsIParentChannel, aClassificationFlags: uint32_t, aIsThirdParty: bool) -> ::nserror::nsresult,

    /* void delete (); */
    pub Delete: unsafe extern "system" fn (this: *const nsIParentChannel) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String remoteType; */
    pub GetRemoteType: unsafe extern "system" fn (this: *const nsIParentChannel, aRemoteType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIParentChannel {

    /// ```text
    /// /**
    ///    * Called to set the ParentChannelListener object (optional).
    ///    */
    /// ```
    ///

    /// `[noscript] void setParentListener (in ParentChannelListener listener);`
    const _SetParentListener: () = ();

    /// ```text
    /// /**
    ///    * Called to notify the HttpChannelChild that flash plugin state has changed.
    ///    */
    /// ```
    ///

    /// `[noscript] void notifyFlashPluginStateChanged (in nsIHttpChannel_FlashPluginState aState);`
    #[inline]
    pub unsafe fn NotifyFlashPluginStateChanged(&self, aState:  u8) -> ::nserror::nsresult {
        ((*self.vtable).NotifyFlashPluginStateChanged)(self, aState)
    }


    /// ```text
    /// /**
    ///    * Called to set matched information when URL matches SafeBrowsing list.
    ///    * @param aList
    ///    *        Name of the list that matched
    ///    * @param aProvider
    ///    *        Name of provider that matched
    ///    * @param aFullHash
    ///    *        String represents full hash that matched
    ///    */
    /// ```
    ///

    /// `[noscript] void setClassifierMatchedInfo (in ACString aList, in ACString aProvider, in ACString aFullHash);`
    #[inline]
    pub unsafe fn SetClassifierMatchedInfo(&self, aList: *const ::nsstring::nsACString, aProvider: *const ::nsstring::nsACString, aFullHash: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetClassifierMatchedInfo)(self, aList, aProvider, aFullHash)
    }


    /// ```text
    /// /**
    ///    * Called to set matched tracking information when URL matches tracking annotation list.
    ///    * @param aList
    ///    *        Comma-separated list of tables that matched
    ///    * @param aFullHashes
    ///    *        Comma-separated list of base64 encoded full hashes that matched
    ///    */
    /// ```
    ///

    /// `[noscript] void setClassifierMatchedTrackingInfo (in ACString aLists, in ACString aFullHashes);`
    #[inline]
    pub unsafe fn SetClassifierMatchedTrackingInfo(&self, aLists: *const ::nsstring::nsACString, aFullHashes: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetClassifierMatchedTrackingInfo)(self, aLists, aFullHashes)
    }


    /// ```text
    /// /**
    ///    * Called to notify the HttpChannelChild that the resource being loaded
    ///    * has been classified.
    ///    * @param aClassificationFlags
    ///    *        What classifier identifies this channel.
    ///    * @param aIsThirdParty
    ///    *        Whether or not the resourced is considered first-party
    ///    *        with the URI of the window.
    ///    */
    /// ```
    ///

    /// `[noscript] void notifyClassificationFlags (in uint32_t aClassificationFlags, in bool aIsThirdParty);`
    #[inline]
    pub unsafe fn NotifyClassificationFlags(&self, aClassificationFlags: uint32_t, aIsThirdParty: bool) -> ::nserror::nsresult {
        ((*self.vtable).NotifyClassificationFlags)(self, aClassificationFlags, aIsThirdParty)
    }


    /// ```text
    /// /**
    ///    * Called to invoke deletion of the IPC protocol.
    ///    */
    /// ```
    ///

    /// `void delete ();`
    #[inline]
    pub unsafe fn Delete(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Delete)(self, )
    }


    /// ```text
    /// /**
    ///    * The remote type of the target process for this load.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String remoteType;`
    #[inline]
    pub unsafe fn GetRemoteType(&self, aRemoteType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRemoteType)(self, aRemoteType)
    }


}


