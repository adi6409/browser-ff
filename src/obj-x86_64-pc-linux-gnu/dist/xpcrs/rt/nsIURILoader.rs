//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/uriloader/base/nsIURILoader.idl
//


/// `interface nsIURILoader : nsISupports`
///

/// ```text
/// /**
///  * The uri dispatcher is responsible for taking uri's, determining
///  * the content and routing the opened url to the correct content
///  * handler.
///  *
///  * When you encounter a url you want to open, you typically call
///  * openURI, passing it the content listener for the window the uri is
///  * originating from. The uri dispatcher opens the url to discover the
///  * content type. It then gives the content listener first crack at
///  * handling the content. If it doesn't want it, the dispatcher tries
///  * to hand it off one of the registered content listeners. This allows
///  * running applications the chance to jump in and handle the content.
///  *
///  * If that also fails, then the uri dispatcher goes to the registry
///  * looking for the preferred content handler for the content type
///  * of the uri. The content handler may create an app instance
///  * or it may hand the contents off to a platform specific plugin
///  * or helper app. Or it may hand the url off to an OS registered
///  * application.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIURILoader {
    vtable: *const nsIURILoaderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIURILoader.
unsafe impl XpCom for nsIURILoader {
    const IID: nsIID = nsID(0x8762c4e7, 0xbe35, 0x4958,
        [0x9b, 0x81, 0xa0, 0x56, 0x85, 0xbb, 0x51, 0x6d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIURILoader {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIURILoader.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIURILoaderCoerce {
    /// Cheaply cast a value of this type from a `nsIURILoader`.
    fn coerce_from(v: &nsIURILoader) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIURILoaderCoerce for nsIURILoader {
    #[inline]
    fn coerce_from(v: &nsIURILoader) -> &Self {
        v
    }
}

impl nsIURILoader {
    /// Cast this `nsIURILoader` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIURILoaderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIURILoader {
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
impl<T: nsISupportsCoerce> nsIURILoaderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIURILoader) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIURILoader
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIURILoaderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void registerContentListener (in nsIURIContentListener aContentListener); */
    pub RegisterContentListener: unsafe extern "system" fn (this: *const nsIURILoader, aContentListener: *const nsIURIContentListener) -> ::nserror::nsresult,

    /* void unRegisterContentListener (in nsIURIContentListener aContentListener); */
    pub UnRegisterContentListener: unsafe extern "system" fn (this: *const nsIURILoader, aContentListener: *const nsIURIContentListener) -> ::nserror::nsresult,

    /* void openURI (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
    pub OpenURI: unsafe extern "system" fn (this: *const nsIURILoader, aChannel: *const nsIChannel, aFlags: u32, aWindowContext: *const nsIInterfaceRequestor) -> ::nserror::nsresult,

    /* nsIStreamListener openChannel (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext); */
    pub OpenChannel: unsafe extern "system" fn (this: *const nsIURILoader, aChannel: *const nsIChannel, aFlags: u32, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult,

    /* void stop (in nsISupports aLoadCookie); */
    pub Stop: unsafe extern "system" fn (this: *const nsIURILoader, aLoadCookie: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIURILoader {
    /// ```text
    /// /**
    ///    * @name Flags for opening URIs.
    ///    */
    /// /**
    ///    * Should the content be displayed in a container that prefers the
    ///    * content-type, or will any container do.
    ///    */
    /// ```
    ///

    pub const IS_CONTENT_PREFERRED: i64 = 1;

    /// ```text
    /// /**
    ///    * If this flag is set, only the listener of the specified window context will
    ///    * be considered for content handling; if it refuses the load, an error will
    ///    * be indicated.
    ///    */
    /// ```
    ///

    pub const DONT_RETARGET: i64 = 2;

    /// ```text
    /// /**
    ///    * As applications such as messenger and the browser are instantiated,
    ///    * they register content listener's with the uri dispatcher corresponding
    ///    * to content windows within that application.
    ///    *
    ///    * Note to self: we may want to optimize things a bit more by requiring
    ///    * the content types the registered content listener cares about.
    ///    *
    ///    * @param aContentListener
    ///    *        The listener to register. This listener must implement
    ///    *        nsISupportsWeakReference.
    ///    *
    ///    * @see the nsIURILoader class description
    ///    */
    /// ```
    ///

    /// `void registerContentListener (in nsIURIContentListener aContentListener);`
    #[inline]
    pub unsafe fn RegisterContentListener(&self, aContentListener: *const nsIURIContentListener) -> ::nserror::nsresult {
        ((*self.vtable).RegisterContentListener)(self, aContentListener)
    }



    /// `void unRegisterContentListener (in nsIURIContentListener aContentListener);`
    #[inline]
    pub unsafe fn UnRegisterContentListener(&self, aContentListener: *const nsIURIContentListener) -> ::nserror::nsresult {
        ((*self.vtable).UnRegisterContentListener)(self, aContentListener)
    }


    /// ```text
    /// /**
    ///    * OpenURI requires the following parameters.....
    ///    * @param aChannel
    ///    *        The channel that should be opened. This must not be asyncOpen'd yet!
    ///    *        If a loadgroup is set on the channel, it will get replaced with a
    ///    *        different one.
    ///    * @param aFlags
    ///    *        Combination (bitwise OR) of the flags specified above. 0 indicates
    ///    *        default handling.
    ///    * @param aWindowContext
    ///    *        If you are running the url from a doc shell or a web shell, this is
    ///    *        your window context. If you have a content listener you want to
    ///    *        give first crack to, the uri loader needs to be able to get it
    ///    *        from the window context. We will also be using the window context
    ///    *        to get at the progress event sink interface.
    ///    *        <b>Must not be null!</b>
    ///    */
    /// ```
    ///

    /// `void openURI (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext);`
    #[inline]
    pub unsafe fn OpenURI(&self, aChannel: *const nsIChannel, aFlags: u32, aWindowContext: *const nsIInterfaceRequestor) -> ::nserror::nsresult {
        ((*self.vtable).OpenURI)(self, aChannel, aFlags, aWindowContext)
    }


    /// ```text
    /// /**
    ///    * Loads data from a channel. This differs from openURI in that the channel
    ///    * may already be opened, and that it returns a stream listener into which the
    ///    * caller should pump data. The caller is responsible for opening the channel
    ///    * and pumping the channel's data into the returned stream listener.
    ///    *
    ///    * Note: If the channel already has a loadgroup, it will be replaced with the
    ///    * window context's load group, or null if the context doesn't have one.
    ///    *
    ///    * If the window context's nsIURIContentListener refuses the load immediately
    ///    * (e.g. in nsIURIContentListener::onStartURIOpen), this method will return
    ///    * NS_ERROR_WONT_HANDLE_CONTENT. At that point, the caller should probably
    ///    * cancel the channel if it's already open (this method will not cancel the
        ///    * channel).
    ///    *
    ///    * If flags include DONT_RETARGET, and the content listener refuses the load
    ///    * during onStartRequest (e.g. in canHandleContent/isPreferred), then the
    ///    * returned stream listener's onStartRequest method will return
    ///    * NS_ERROR_WONT_HANDLE_CONTENT.
    ///    *
    ///    * @param aChannel
    ///    *        The channel that should be loaded. The channel may already be
    ///    *        opened. It must not be closed (i.e. this must be called before the
        ///    *        channel calls onStopRequest on its stream listener).
    ///    * @param aFlags
    ///    *        Combination (bitwise OR) of the flags specified above. 0 indicates
    ///    *        default handling.
    ///    * @param aWindowContext
    ///    *        If you are running the url from a doc shell or a web shell, this is
    ///    *        your window context. If you have a content listener you want to
    ///    *        give first crack to, the uri loader needs to be able to get it
    ///    *        from the window context. We will also be using the window context
    ///    *        to get at the progress event sink interface.
    ///    *        <b>Must not be null!</b>
    ///    */
    /// ```
    ///

    /// `nsIStreamListener openChannel (in nsIChannel aChannel, in unsigned long aFlags, in nsIInterfaceRequestor aWindowContext);`
    #[inline]
    pub unsafe fn OpenChannel(&self, aChannel: *const nsIChannel, aFlags: u32, aWindowContext: *const nsIInterfaceRequestor, _retval: *mut*const nsIStreamListener) -> ::nserror::nsresult {
        ((*self.vtable).OpenChannel)(self, aChannel, aFlags, aWindowContext, _retval)
    }


    /// ```text
    /// /**
    ///    * Stops an in progress load
    ///    */
    /// ```
    ///

    /// `void stop (in nsISupports aLoadCookie);`
    #[inline]
    pub unsafe fn Stop(&self, aLoadCookie: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).Stop)(self, aLoadCookie)
    }


}


