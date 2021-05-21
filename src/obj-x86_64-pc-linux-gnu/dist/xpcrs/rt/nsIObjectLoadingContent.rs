//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIObjectLoadingContent.idl
//


/// `interface nsIObjectLoadingContent : nsISupports`
///

/// ```text
/// /**
///  * This interface represents a content node that loads objects.
///  *
///  * Please make sure to update the MozObjectLoadingContent WebIDL
///  * mixin to mirror this interface when changing it.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIObjectLoadingContent {
    vtable: *const nsIObjectLoadingContentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIObjectLoadingContent.
unsafe impl XpCom for nsIObjectLoadingContent {
    const IID: nsIID = nsID(0x2eb3195e, 0x3eea, 0x4083,
        [0xbb, 0x1d, 0xd2, 0xd7, 0x0f, 0xa3, 0x5c, 0xcb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIObjectLoadingContent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIObjectLoadingContent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIObjectLoadingContentCoerce {
    /// Cheaply cast a value of this type from a `nsIObjectLoadingContent`.
    fn coerce_from(v: &nsIObjectLoadingContent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIObjectLoadingContentCoerce for nsIObjectLoadingContent {
    #[inline]
    fn coerce_from(v: &nsIObjectLoadingContent) -> &Self {
        v
    }
}

impl nsIObjectLoadingContent {
    /// Cast this `nsIObjectLoadingContent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIObjectLoadingContentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIObjectLoadingContent {
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
impl<T: nsISupportsCoerce> nsIObjectLoadingContentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIObjectLoadingContent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIObjectLoadingContent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIObjectLoadingContentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString actualType; */
    pub GetActualType: unsafe extern "system" fn (this: *const nsIObjectLoadingContent, aActualType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long displayedType; */
    pub GetDisplayedType: unsafe extern "system" fn (this: *const nsIObjectLoadingContent, aDisplayedType: *mut u32) -> ::nserror::nsresult,

    /* unsigned long getContentTypeForMIMEType (in AUTF8String aMimeType); */
    pub GetContentTypeForMIMEType: unsafe extern "system" fn (this: *const nsIObjectLoadingContent, aMimeType: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] readonly attribute nsNPAPIPluginInstancePtr pluginInstance; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPluginInstance: *const ::libc::c_void,

    /* [noscript] void pluginDestroyed (); */
    pub PluginDestroyed: unsafe extern "system" fn (this: *const nsIObjectLoadingContent) -> ::nserror::nsresult,

    /* [noscript] void pluginCrashed (in nsIPluginTag pluginTag, in AString pluginDumpID, in boolean submittedCrashReport); */
    pub PluginCrashed: unsafe extern "system" fn (this: *const nsIObjectLoadingContent, pluginTag: *const nsIPluginTag, pluginDumpID: *const ::nsstring::nsAString, submittedCrashReport: bool) -> ::nserror::nsresult,

    /* void reload (in boolean aClearActivation); */
    pub Reload: unsafe extern "system" fn (this: *const nsIObjectLoadingContent, aClearActivation: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean activated; */
    pub GetActivated: unsafe extern "system" fn (this: *const nsIObjectLoadingContent, aActivated: *mut bool) -> ::nserror::nsresult,

    /* [noscript] void stopPluginInstance (); */
    pub StopPluginInstance: unsafe extern "system" fn (this: *const nsIObjectLoadingContent) -> ::nserror::nsresult,

    /* [noscript] void syncStartPluginInstance (); */
    pub SyncStartPluginInstance: unsafe extern "system" fn (this: *const nsIObjectLoadingContent) -> ::nserror::nsresult,

    /* [noscript] void asyncStartPluginInstance (); */
    pub AsyncStartPluginInstance: unsafe extern "system" fn (this: *const nsIObjectLoadingContent) -> ::nserror::nsresult,

    /* [noscript] void initializeFromChannel (in nsIRequest request); */
    pub InitializeFromChannel: unsafe extern "system" fn (this: *const nsIObjectLoadingContent, request: *const nsIRequest) -> ::nserror::nsresult,

    /* readonly attribute nsIURI srcURI; */
    pub GetSrcURI: unsafe extern "system" fn (this: *const nsIObjectLoadingContent, aSrcURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* void skipFakePlugins (); */
    pub SkipFakePlugins: unsafe extern "system" fn (this: *const nsIObjectLoadingContent) -> ::nserror::nsresult,

    /* [noscript] BrowsingContext upgradeLoadToDocument (in nsIChannel channel); */
    pub UpgradeLoadToDocument: unsafe extern "system" fn (this: *const nsIObjectLoadingContent, channel: *const nsIChannel, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIObjectLoadingContent {
    /// ```text
    /// /**
    ///    * See notes in nsObjectLoadingContent.h
    ///    */
    /// ```
    ///

    pub const TYPE_LOADING: i64 = 0;


    pub const TYPE_IMAGE: i64 = 1;


    pub const TYPE_PLUGIN: i64 = 2;


    pub const TYPE_FAKE_PLUGIN: i64 = 3;


    pub const TYPE_DOCUMENT: i64 = 4;


    pub const TYPE_NULL: i64 = 5;


    pub const PLUGIN_ACTIVE: i64 = 255;


    pub const PLUGIN_UNSUPPORTED: i64 = 0;


    pub const PLUGIN_ALTERNATE: i64 = 1;


    pub const PLUGIN_DISABLED: i64 = 2;


    pub const PLUGIN_BLOCKLISTED: i64 = 3;


    pub const PLUGIN_OUTDATED: i64 = 4;


    pub const PLUGIN_CRASHED: i64 = 5;


    pub const PLUGIN_CLICK_TO_PLAY: i64 = 8;


    pub const PLUGIN_VULNERABLE_UPDATABLE: i64 = 9;


    pub const PLUGIN_VULNERABLE_NO_UPDATE: i64 = 10;


    pub const PLUGIN_CLICK_TO_PLAY_QUIET: i64 = 11;


    pub const PLUGIN_BLOCK_ALL: i64 = 12;


    pub const PLUGIN_PERMISSION_PROMPT_ACTION_QUIET: i64 = 8;

    /// ```text
    /// /**
    ///    * The actual mime type (the one we got back from the network
        ///    * request) for the element.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString actualType;`
    #[inline]
    pub unsafe fn GetActualType(&self, aActualType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetActualType)(self, aActualType)
    }


    /// ```text
    /// /**
    ///    * Gets the type of the content that's currently loaded. See
    ///    * the constants above for the list of possible values.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long displayedType;`
    #[inline]
    pub unsafe fn GetDisplayedType(&self, aDisplayedType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetDisplayedType)(self, aDisplayedType)
    }


    /// ```text
    /// /**
    ///    * Gets the content type that corresponds to the give MIME type.  See the
    ///    * constants above for the list of possible values.  If nothing else fits,
    ///    * TYPE_NULL will be returned.
    ///    */
    /// ```
    ///

    /// `unsigned long getContentTypeForMIMEType (in AUTF8String aMimeType);`
    #[inline]
    pub unsafe fn GetContentTypeForMIMEType(&self, aMimeType: *const ::nsstring::nsACString, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetContentTypeForMIMEType)(self, aMimeType, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns the plugin instance if it has already been instantiated. This
    ///    * will never instantiate the plugin and so is safe to call even when
    ///    * content script must not execute.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] readonly attribute nsNPAPIPluginInstancePtr pluginInstance;`
    const _GetPluginInstance: () = ();


    /// `[noscript] void pluginDestroyed ();`
    #[inline]
    pub unsafe fn PluginDestroyed(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).PluginDestroyed)(self, )
    }



    /// `[noscript] void pluginCrashed (in nsIPluginTag pluginTag, in AString pluginDumpID, in boolean submittedCrashReport);`
    #[inline]
    pub unsafe fn PluginCrashed(&self, pluginTag: *const nsIPluginTag, pluginDumpID: *const ::nsstring::nsAString, submittedCrashReport: bool) -> ::nserror::nsresult {
        ((*self.vtable).PluginCrashed)(self, pluginTag, pluginDumpID, submittedCrashReport)
    }


    /// ```text
    /// /**
    ///    * Forces a re-evaluation and reload of the tag, optionally invalidating its
    ///    * click-to-play state.  This can be used when the MIME type that provides a
    ///    * type has changed, for instance, to force the tag to re-evalulate the
    ///    * handler to use.
    ///    */
    /// ```
    ///

    /// `void reload (in boolean aClearActivation);`
    #[inline]
    pub unsafe fn Reload(&self, aClearActivation: bool) -> ::nserror::nsresult {
        ((*self.vtable).Reload)(self, aClearActivation)
    }


    /// ```text
    /// /**
    ///    * This attribute will return true if the current content type has been
    ///    * activated, either explicitly or by passing checks that would have it be
    ///    * click-to-play.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean activated;`
    #[inline]
    pub unsafe fn GetActivated(&self, aActivated: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetActivated)(self, aActivated)
    }



    /// `[noscript] void stopPluginInstance ();`
    #[inline]
    pub unsafe fn StopPluginInstance(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StopPluginInstance)(self, )
    }



    /// `[noscript] void syncStartPluginInstance ();`
    #[inline]
    pub unsafe fn SyncStartPluginInstance(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SyncStartPluginInstance)(self, )
    }



    /// `[noscript] void asyncStartPluginInstance ();`
    #[inline]
    pub unsafe fn AsyncStartPluginInstance(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).AsyncStartPluginInstance)(self, )
    }


    /// ```text
    /// /**
    ///    * Puts the tag in the "waiting on a channel" state and adopts this
    ///    * channel. This does not override the normal logic of examining attributes
    ///    * and the channel type, so the load may cancel this channel if it decides not
    ///    * to use one.
    ///    *
    ///    * This assumes:
    ///    *  - This tag has not begun loading yet
    ///    *  - This channel has not yet hit OnStartRequest
    ///    *  - The caller will continue to pass channel events to us as a listener
    ///    */
    /// ```
    ///

    /// `[noscript] void initializeFromChannel (in nsIRequest request);`
    #[inline]
    pub unsafe fn InitializeFromChannel(&self, request: *const nsIRequest) -> ::nserror::nsresult {
        ((*self.vtable).InitializeFromChannel)(self, request)
    }


    /// ```text
    /// /**
    ///    * The URL of the data/src loaded in the object. This may be null (i.e.
        ///    * an <embed> with no src).
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI srcURI;`
    #[inline]
    pub unsafe fn GetSrcURI(&self, aSrcURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetSrcURI)(self, aSrcURI)
    }


    /// ```text
    /// /**
    ///    * Disable the use of fake plugins and reload the tag if necessary.
    ///    */
    /// ```
    ///

    /// `void skipFakePlugins ();`
    #[inline]
    pub unsafe fn SkipFakePlugins(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).SkipFakePlugins)(self, )
    }


    /// ```text
    /// /**
    ///    * Switch the tag into the TYPE_DOCUMENT state, and returns the
    ///    * BrowsingContext which the load should complete in.
    ///    */
    /// ```
    ///

    /// `[noscript] BrowsingContext upgradeLoadToDocument (in nsIChannel channel);`
    #[inline]
    pub unsafe fn UpgradeLoadToDocument(&self, channel: *const nsIChannel, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).UpgradeLoadToDocument)(self, channel, _retval)
    }


}


