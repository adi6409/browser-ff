//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginHost.idl
//


/// `interface nsIClearSiteDataCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIClearSiteDataCallback {
    vtable: *const nsIClearSiteDataCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIClearSiteDataCallback.
unsafe impl XpCom for nsIClearSiteDataCallback {
    const IID: nsIID = nsID(0x9c311778, 0x7c2c, 0x4ad8,
        [0xb4, 0x39, 0xb8, 0xa2, 0x78, 0x6a, 0x20, 0xdd]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIClearSiteDataCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIClearSiteDataCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIClearSiteDataCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIClearSiteDataCallback`.
    fn coerce_from(v: &nsIClearSiteDataCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIClearSiteDataCallbackCoerce for nsIClearSiteDataCallback {
    #[inline]
    fn coerce_from(v: &nsIClearSiteDataCallback) -> &Self {
        v
    }
}

impl nsIClearSiteDataCallback {
    /// Cast this `nsIClearSiteDataCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIClearSiteDataCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIClearSiteDataCallback {
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
impl<T: nsISupportsCoerce> nsIClearSiteDataCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIClearSiteDataCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIClearSiteDataCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIClearSiteDataCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void callback (in nsresult rv); */
    pub Callback: unsafe extern "system" fn (this: *const nsIClearSiteDataCallback, rv: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIClearSiteDataCallback {

    /// ```text
    /// /**
    ///      * callback with the result from a call to clearSiteData
    ///      */
    /// ```
    ///

    /// `void callback (in nsresult rv);`
    #[inline]
    pub unsafe fn Callback(&self, rv: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).Callback)(self, rv)
    }


}


/// `interface nsIPluginHost : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPluginHost {
    vtable: *const nsIPluginHostVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPluginHost.
unsafe impl XpCom for nsIPluginHost {
    const IID: nsIID = nsID(0xf938f5ba, 0x7093, 0x42cd,
        [0xa5, 0x59, 0xaf, 0x80, 0x39, 0xd9, 0x92, 0x04]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPluginHost {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPluginHost.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPluginHostCoerce {
    /// Cheaply cast a value of this type from a `nsIPluginHost`.
    fn coerce_from(v: &nsIPluginHost) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPluginHostCoerce for nsIPluginHost {
    #[inline]
    fn coerce_from(v: &nsIPluginHost) -> &Self {
        v
    }
}

impl nsIPluginHost {
    /// Cast this `nsIPluginHost` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPluginHostCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPluginHost {
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
impl<T: nsISupportsCoerce> nsIPluginHostCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginHost) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPluginHost
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPluginHostVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void reloadPlugins (); */
    pub ReloadPlugins: unsafe extern "system" fn (this: *const nsIPluginHost) -> ::nserror::nsresult,

    /* Array<nsIPluginTag> getPluginTags (); */
    pub GetPluginTags: unsafe extern "system" fn (this: *const nsIPluginHost, _retval: *mut thin_vec::ThinVec<RefPtr<nsIPluginTag>>) -> ::nserror::nsresult,

    /* void clearSiteData (in nsIPluginTag plugin, in AUTF8String domain, in uint64_t flags, in int64_t maxAge, in nsIClearSiteDataCallback callback); */
    pub ClearSiteData: unsafe extern "system" fn (this: *const nsIPluginHost, plugin: *const nsIPluginTag, domain: *const ::nsstring::nsACString, flags: uint64_t, maxAge: int64_t, callback: *const nsIClearSiteDataCallback) -> ::nserror::nsresult,

    /* boolean siteHasData (in nsIPluginTag plugin, in AUTF8String domain); */
    pub SiteHasData: unsafe extern "system" fn (this: *const nsIPluginHost, plugin: *const nsIPluginTag, domain: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* ACString getPermissionStringForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
    pub GetPermissionStringForType: unsafe extern "system" fn (this: *const nsIPluginHost, mimeType: *const ::nsstring::nsACString, excludeFlags: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getPermissionStringForTag (in nsIPluginTag tag, [optional] in uint32_t excludeFlags); */
    pub GetPermissionStringForTag: unsafe extern "system" fn (this: *const nsIPluginHost, tag: *const nsIPluginTag, excludeFlags: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* nsIPluginTag getPluginTagForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
    pub GetPluginTagForType: unsafe extern "system" fn (this: *const nsIPluginHost, mimeType: *const ::nsstring::nsACString, excludeFlags: uint32_t, _retval: *mut *const nsIPluginTag) -> ::nserror::nsresult,

    /* unsigned long getStateForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags); */
    pub GetStateForType: unsafe extern "system" fn (this: *const nsIPluginHost, mimeType: *const ::nsstring::nsACString, excludeFlags: uint32_t, _retval: *mut u32) -> ::nserror::nsresult,

    /* uint32_t getBlocklistStateForType (in AUTF8String aMimeType, [optional] in uint32_t excludeFlags); */
    pub GetBlocklistStateForType: unsafe extern "system" fn (this: *const nsIPluginHost, aMimeType: *const ::nsstring::nsACString, excludeFlags: uint32_t, _retval: *mut uint32_t) -> ::nserror::nsresult,

    /* [implicit_jscontext] nsIFakePluginTag registerFakePlugin (in jsval initDictionary); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RegisterFakePlugin: *const ::libc::c_void,

    /* [implicit_jscontext] nsIFakePluginTag createFakePlugin (in jsval initDictionary); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub CreateFakePlugin: *const ::libc::c_void,

    /* nsIFakePluginTag getFakePlugin (in AUTF8String mimeType); */
    pub GetFakePlugin: unsafe extern "system" fn (this: *const nsIPluginHost, mimeType: *const ::nsstring::nsACString, _retval: *mut *const nsIFakePluginTag) -> ::nserror::nsresult,

    /* void unregisterFakePlugin (in AUTF8String handlerURI); */
    pub UnregisterFakePlugin: unsafe extern "system" fn (this: *const nsIPluginHost, handlerURI: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPluginHost {

    pub const FLAG_CLEAR_ALL: i64 = 0;


    pub const FLAG_CLEAR_CACHE: i64 = 1;


    pub const EXCLUDE_NONE: i64 = 0;


    pub const EXCLUDE_DISABLED: i64 = 1;


    pub const EXCLUDE_FAKE: i64 = 2;

    /// ```text
    /// /**
    ///    * Causes the plugins directory to be searched again for new plugin
    ///    * libraries.
    ///    */
    /// ```
    ///

    /// `void reloadPlugins ();`
    #[inline]
    pub unsafe fn ReloadPlugins(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ReloadPlugins)(self, )
    }



    /// `Array<nsIPluginTag> getPluginTags ();`
    #[inline]
    pub unsafe fn GetPluginTags(&self, _retval: *mut thin_vec::ThinVec<RefPtr<nsIPluginTag>>) -> ::nserror::nsresult {
        ((*self.vtable).GetPluginTags)(self, _retval)
    }



    /// `void clearSiteData (in nsIPluginTag plugin, in AUTF8String domain, in uint64_t flags, in int64_t maxAge, in nsIClearSiteDataCallback callback);`
    #[inline]
    pub unsafe fn ClearSiteData(&self, plugin: *const nsIPluginTag, domain: *const ::nsstring::nsACString, flags: uint64_t, maxAge: int64_t, callback: *const nsIClearSiteDataCallback) -> ::nserror::nsresult {
        ((*self.vtable).ClearSiteData)(self, plugin, domain, flags, maxAge, callback)
    }



    /// `boolean siteHasData (in nsIPluginTag plugin, in AUTF8String domain);`
    #[inline]
    pub unsafe fn SiteHasData(&self, plugin: *const nsIPluginTag, domain: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).SiteHasData)(self, plugin, domain, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the "permission string" for the plugin.  This is a string that can be
    ///    * passed to the permission manager to see whether the plugin is allowed to
    ///    * run, for example.  This will typically be based on the plugin's "nice name"
    ///    * and its blocklist state.
    ///    *
    ///    * @mimeType The MIME type we're interested in.
    ///    * @excludeFlags Set of the EXCLUDE_* flags above, defaulting to EXCLUDE_NONE.
    ///    */
    /// ```
    ///

    /// `ACString getPermissionStringForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags);`
    #[inline]
    pub unsafe fn GetPermissionStringForType(&self, mimeType: *const ::nsstring::nsACString, excludeFlags: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPermissionStringForType)(self, mimeType, excludeFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the "permission string" for the plugin.  This is a string that can be
    ///    * passed to the permission manager to see whether the plugin is allowed to
    ///    * run, for example.  This will typically be based on the plugin's "nice name"
    ///    * and its blocklist state.
    ///    *
    ///    * @tag The tage we're interested in
    ///    * @excludeFlags Set of the EXCLUDE_* flags above, defaulting to EXCLUDE_NONE.
    ///    */
    /// ```
    ///

    /// `ACString getPermissionStringForTag (in nsIPluginTag tag, [optional] in uint32_t excludeFlags);`
    #[inline]
    pub unsafe fn GetPermissionStringForTag(&self, tag: *const nsIPluginTag, excludeFlags: uint32_t, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPermissionStringForTag)(self, tag, excludeFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the nsIPluginTag for this MIME type. This method works with both
    ///    * enabled and disabled/blocklisted plugins, but an enabled plugin will
    ///    * always be returned if available.
    ///    *
    ///    * A fake plugin tag, if one exists and is available, will be returned in
    ///    * preference to NPAPI plugin tags unless excluded by the excludeFlags.
    ///    *
    ///    * @mimeType The MIME type we're interested in.
    ///    * @excludeFlags Set of the EXCLUDE_* flags above, defaulting to EXCLUDE_NONE.
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE if no plugin is available for this MIME
    ///    *         type.
    ///    */
    /// ```
    ///

    /// `nsIPluginTag getPluginTagForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags);`
    #[inline]
    pub unsafe fn GetPluginTagForType(&self, mimeType: *const ::nsstring::nsACString, excludeFlags: uint32_t, _retval: *mut *const nsIPluginTag) -> ::nserror::nsresult {
        ((*self.vtable).GetPluginTagForType)(self, mimeType, excludeFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the nsIPluginTag enabled state for this MIME type.  See
    ///    * nsIPluginTag.enabledState.
    ///    *
    ///    * @mimeType The MIME type we're interested in.
    ///    * @excludeFlags Set of the EXCLUDE_* flags above, defaulting to EXCLUDE_NONE.
    ///    */
    /// ```
    ///

    /// `unsigned long getStateForType (in AUTF8String mimeType, [optional] in uint32_t excludeFlags);`
    #[inline]
    pub unsafe fn GetStateForType(&self, mimeType: *const ::nsstring::nsACString, excludeFlags: uint32_t, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetStateForType)(self, mimeType, excludeFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the blocklist state for a MIME type.  See nsIPluginTag.blocklistState.
    ///    *
    ///    * @mimeType The MIME type we're interested in.
    ///    * @excludeFlags Set of the EXCLUDE_* flags above, defaulting to EXCLUDE_NONE.
    ///    */
    /// ```
    ///

    /// `uint32_t getBlocklistStateForType (in AUTF8String aMimeType, [optional] in uint32_t excludeFlags);`
    #[inline]
    pub unsafe fn GetBlocklistStateForType(&self, aMimeType: *const ::nsstring::nsACString, excludeFlags: uint32_t, _retval: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetBlocklistStateForType)(self, aMimeType, excludeFlags, _retval)
    }


    /// ```text
    /// /**
    ///    * Create a fake plugin tag, register it, and return it.  The argument is a
    ///    * FakePluginTagInit dictionary.  See documentation in
    ///    * FakePluginTagInit.webidl for what it should look like.  Will throw
    ///    * NS_ERROR_UNEXPECTED if there is already a fake plugin registered with the
    ///    * given handler URI.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] nsIFakePluginTag registerFakePlugin (in jsval initDictionary);`
    const _RegisterFakePlugin: () = ();

    /// ```text
    /// /**
    ///    * Create a fake plugin tag without registering it.
    ///    *
    ///    * Only for use in tests.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] nsIFakePluginTag createFakePlugin (in jsval initDictionary);`
    const _CreateFakePlugin: () = ();

    /// ```text
    /// /**
    ///    * Get a reference to an existing fake plugin tag for the given MIME type, if
    ///    * any.  Can return null.
    ///    */
    /// ```
    ///

    /// `nsIFakePluginTag getFakePlugin (in AUTF8String mimeType);`
    #[inline]
    pub unsafe fn GetFakePlugin(&self, mimeType: *const ::nsstring::nsACString, _retval: *mut *const nsIFakePluginTag) -> ::nserror::nsresult {
        ((*self.vtable).GetFakePlugin)(self, mimeType, _retval)
    }


    /// ```text
    /// /**
    ///    * Unregister a fake plugin.  The argument can be the .handlerURI.spec of an
    ///    * existing nsIFakePluginTag, or just a known handler URI string that was
    ///    * passed in the FakePluginTagInit when registering.
    ///    */
    /// ```
    ///

    /// `void unregisterFakePlugin (in AUTF8String handlerURI);`
    #[inline]
    pub unsafe fn UnregisterFakePlugin(&self, handlerURI: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterFakePlugin)(self, handlerURI)
    }


}


