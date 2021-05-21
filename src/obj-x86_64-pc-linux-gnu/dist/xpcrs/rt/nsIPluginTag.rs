//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginTag.idl
//


/// `interface nsIPluginTag : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPluginTag {
    vtable: *const nsIPluginTagVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPluginTag.
unsafe impl XpCom for nsIPluginTag {
    const IID: nsIID = nsID(0x5daa99d5, 0x265a, 0x4397,
        [0xb4, 0x29, 0xc9, 0x43, 0x80, 0x3e, 0x26, 0x19]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPluginTag {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPluginTag.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPluginTagCoerce {
    /// Cheaply cast a value of this type from a `nsIPluginTag`.
    fn coerce_from(v: &nsIPluginTag) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPluginTagCoerce for nsIPluginTag {
    #[inline]
    fn coerce_from(v: &nsIPluginTag) -> &Self {
        v
    }
}

impl nsIPluginTag {
    /// Cast this `nsIPluginTag` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPluginTagCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPluginTag {
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
impl<T: nsISupportsCoerce> nsIPluginTagCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginTag) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPluginTag
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPluginTagVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AUTF8String description; */
    pub GetDescription: unsafe extern "system" fn (this: *const nsIPluginTag, aDescription: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String filename; */
    pub GetFilename: unsafe extern "system" fn (this: *const nsIPluginTag, aFilename: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String fullpath; */
    pub GetFullpath: unsafe extern "system" fn (this: *const nsIPluginTag, aFullpath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String version; */
    pub GetVersion: unsafe extern "system" fn (this: *const nsIPluginTag, aVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIPluginTag, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String niceName; */
    pub GetNiceName: unsafe extern "system" fn (this: *const nsIPluginTag, aNiceName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean blocklisted; */
    pub GetBlocklisted: unsafe extern "system" fn (this: *const nsIPluginTag, aBlocklisted: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isEnabledStateLocked; */
    pub GetIsEnabledStateLocked: unsafe extern "system" fn (this: *const nsIPluginTag, aIsEnabledStateLocked: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean active; */
    pub GetActive: unsafe extern "system" fn (this: *const nsIPluginTag, aActive: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute unsigned long blocklistState; */
    pub GetBlocklistState: unsafe extern "system" fn (this: *const nsIPluginTag, aBlocklistState: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean disabled; */
    pub GetDisabled: unsafe extern "system" fn (this: *const nsIPluginTag, aDisabled: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean clicktoplay; */
    pub GetClicktoplay: unsafe extern "system" fn (this: *const nsIPluginTag, aClicktoplay: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean loaded; */
    pub GetLoaded: unsafe extern "system" fn (this: *const nsIPluginTag, aLoaded: *mut bool) -> ::nserror::nsresult,

    /* attribute unsigned long enabledState; */
    pub GetEnabledState: unsafe extern "system" fn (this: *const nsIPluginTag, aEnabledState: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long enabledState; */
    pub SetEnabledState: unsafe extern "system" fn (this: *const nsIPluginTag, aEnabledState: u32) -> ::nserror::nsresult,

    /* readonly attribute PRTime lastModifiedTime; */
    pub GetLastModifiedTime: unsafe extern "system" fn (this: *const nsIPluginTag, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute boolean isFlashPlugin; */
    pub GetIsFlashPlugin: unsafe extern "system" fn (this: *const nsIPluginTag, aIsFlashPlugin: *mut bool) -> ::nserror::nsresult,

    /* Array<AUTF8String> getMimeTypes (); */
    pub GetMimeTypes: unsafe extern "system" fn (this: *const nsIPluginTag, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* Array<AUTF8String> getMimeDescriptions (); */
    pub GetMimeDescriptions: unsafe extern "system" fn (this: *const nsIPluginTag, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* Array<AUTF8String> getExtensions (); */
    pub GetExtensions: unsafe extern "system" fn (this: *const nsIPluginTag, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute unsigned long id; */
    pub GetId: unsafe extern "system" fn (this: *const nsIPluginTag, aId: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPluginTag {

    pub const STATE_DISABLED: i64 = 0;


    pub const STATE_CLICKTOPLAY: i64 = 1;


    pub const STATE_ENABLED: i64 = 2;


    /// `readonly attribute AUTF8String description;`
    #[inline]
    pub unsafe fn GetDescription(&self, aDescription: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDescription)(self, aDescription)
    }



    /// `readonly attribute AUTF8String filename;`
    #[inline]
    pub unsafe fn GetFilename(&self, aFilename: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFilename)(self, aFilename)
    }



    /// `readonly attribute AUTF8String fullpath;`
    #[inline]
    pub unsafe fn GetFullpath(&self, aFullpath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetFullpath)(self, aFullpath)
    }



    /// `readonly attribute AUTF8String version;`
    #[inline]
    pub unsafe fn GetVersion(&self, aVersion: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetVersion)(self, aVersion)
    }



    /// `readonly attribute AUTF8String name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `readonly attribute AUTF8String niceName;`
    #[inline]
    pub unsafe fn GetNiceName(&self, aNiceName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetNiceName)(self, aNiceName)
    }


    /// ```text
    /// /**
    ///    * true only if this plugin is "hardblocked" and cannot be enabled.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean blocklisted;`
    #[inline]
    pub unsafe fn GetBlocklisted(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetBlocklisted)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * true if the state is non-default and locked, false otherwise.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isEnabledStateLocked;`
    #[inline]
    pub unsafe fn GetIsEnabledStateLocked(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsEnabledStateLocked)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean active;`
    #[inline]
    pub unsafe fn GetActive(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetActive)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute unsigned long blocklistState;`
    #[inline]
    pub unsafe fn GetBlocklistState(&self) -> u32 {
        let mut result = <u32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetBlocklistState)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean disabled;`
    #[inline]
    pub unsafe fn GetDisabled(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetDisabled)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean clicktoplay;`
    #[inline]
    pub unsafe fn GetClicktoplay(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetClicktoplay)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[infallible] readonly attribute boolean loaded;`
    #[inline]
    pub unsafe fn GetLoaded(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetLoaded)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `attribute unsigned long enabledState;`
    #[inline]
    pub unsafe fn GetEnabledState(&self, aEnabledState: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetEnabledState)(self, aEnabledState)
    }



    /// `attribute unsigned long enabledState;`
    #[inline]
    pub unsafe fn SetEnabledState(&self, aEnabledState: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetEnabledState)(self, aEnabledState)
    }



    /// `readonly attribute PRTime lastModifiedTime;`
    #[inline]
    pub unsafe fn GetLastModifiedTime(&self, aLastModifiedTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModifiedTime)(self, aLastModifiedTime)
    }



    /// `readonly attribute boolean isFlashPlugin;`
    #[inline]
    pub unsafe fn GetIsFlashPlugin(&self, aIsFlashPlugin: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsFlashPlugin)(self, aIsFlashPlugin)
    }



    /// `Array<AUTF8String> getMimeTypes ();`
    #[inline]
    pub unsafe fn GetMimeTypes(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetMimeTypes)(self, _retval)
    }



    /// `Array<AUTF8String> getMimeDescriptions ();`
    #[inline]
    pub unsafe fn GetMimeDescriptions(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetMimeDescriptions)(self, _retval)
    }



    /// `Array<AUTF8String> getExtensions ();`
    #[inline]
    pub unsafe fn GetExtensions(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetExtensions)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * An id for this plugin. 0 is a valid id.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long id;`
    #[inline]
    pub unsafe fn GetId(&self, aId: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetId)(self, aId)
    }


}


/// `interface nsIFakePluginTag : nsIPluginTag`
///

/// ```text
/// /**
///  * An interface representing a "fake" plugin: one implemented in JavaScript, not
///  * as a NPAPI plug-in.  See nsIPluginHost.registerFakePlugin and the
///  * documentation for the FakePluginTagInit dictionary.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIFakePluginTag {
    vtable: *const nsIFakePluginTagVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIFakePluginTag.
unsafe impl XpCom for nsIFakePluginTag {
    const IID: nsIID = nsID(0x6d22c968, 0x226d, 0x4156,
        [0xb2, 0x30, 0xda, 0x6a, 0xd6, 0xbb, 0xf6, 0xe8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIFakePluginTag {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIFakePluginTag.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIFakePluginTagCoerce {
    /// Cheaply cast a value of this type from a `nsIFakePluginTag`.
    fn coerce_from(v: &nsIFakePluginTag) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIFakePluginTagCoerce for nsIFakePluginTag {
    #[inline]
    fn coerce_from(v: &nsIFakePluginTag) -> &Self {
        v
    }
}

impl nsIFakePluginTag {
    /// Cast this `nsIFakePluginTag` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIFakePluginTagCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIFakePluginTag {
    type Target = nsIPluginTag;
    #[inline]
    fn deref(&self) -> &nsIPluginTag {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIPluginTagCoerce> nsIFakePluginTagCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIFakePluginTag) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIFakePluginTag
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIFakePluginTagVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIPluginTagVTable,

    /* readonly attribute nsIURI handlerURI; */
    pub GetHandlerURI: unsafe extern "system" fn (this: *const nsIFakePluginTag, aHandlerURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute AString sandboxScript; */
    pub GetSandboxScript: unsafe extern "system" fn (this: *const nsIFakePluginTag, aSandboxScript: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIFakePluginTag {

    /// ```text
    /// /**
    ///    * The URI that should be loaded into the tag (as a frame) to handle the
    ///    * plugin. Note that the original data/src value for the plugin is not loaded
    ///    * and will need to be requested by the handler via XHR or similar if desired.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI handlerURI;`
    #[inline]
    pub unsafe fn GetHandlerURI(&self, aHandlerURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetHandlerURI)(self, aHandlerURI)
    }


    /// ```text
    /// /**
    ///    * Optional script to run in a sandbox when instantiating a plugin. If this
    ///    * value is an empty string then no such script will be run.
    ///    * The script runs in a sandbox with system principal in the process that
    ///    * contains the element that instantiates the plugin (ie the EMBED or OBJECT
        ///    * element). The sandbox global has a 'pluginElement' property that the script
    ///    * can use to access the element that instantiates the plugin.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString sandboxScript;`
    #[inline]
    pub unsafe fn GetSandboxScript(&self, aSandboxScript: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSandboxScript)(self, aSandboxScript)
    }


}


