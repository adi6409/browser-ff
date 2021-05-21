//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/mime/nsIMIMEInfo.idl
//


/// `typedef int32_t  nsHandlerInfoAction;`
///


pub type nsHandlerInfoAction = i32;


/// `interface nsIHandlerInfo : nsISupports`
///

/// ```text
/// /**
///  * nsIHandlerInfo gives access to the information about how a given protocol
///  * scheme or MIME-type is handled.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHandlerInfo {
    vtable: *const nsIHandlerInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHandlerInfo.
unsafe impl XpCom for nsIHandlerInfo {
    const IID: nsIID = nsID(0x325e56a7, 0x3762, 0x4312,
        [0xae, 0xc7, 0xf1, 0xfc, 0xf8, 0x4b, 0x41, 0x45]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHandlerInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHandlerInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHandlerInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIHandlerInfo`.
    fn coerce_from(v: &nsIHandlerInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHandlerInfoCoerce for nsIHandlerInfo {
    #[inline]
    fn coerce_from(v: &nsIHandlerInfo) -> &Self {
        v
    }
}

impl nsIHandlerInfo {
    /// Cast this `nsIHandlerInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHandlerInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHandlerInfo {
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
impl<T: nsISupportsCoerce> nsIHandlerInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHandlerInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHandlerInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHandlerInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute ACString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIHandlerInfo, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AString description; */
    pub GetDescription: unsafe extern "system" fn (this: *const nsIHandlerInfo, aDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString description; */
    pub SetDescription: unsafe extern "system" fn (this: *const nsIHandlerInfo, aDescription: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute nsIHandlerApp preferredApplicationHandler; */
    pub GetPreferredApplicationHandler: unsafe extern "system" fn (this: *const nsIHandlerInfo, aPreferredApplicationHandler: *mut*const nsIHandlerApp) -> ::nserror::nsresult,

    /* attribute nsIHandlerApp preferredApplicationHandler; */
    pub SetPreferredApplicationHandler: unsafe extern "system" fn (this: *const nsIHandlerInfo, aPreferredApplicationHandler: *const nsIHandlerApp) -> ::nserror::nsresult,

    /* readonly attribute nsIMutableArray possibleApplicationHandlers; */
    pub GetPossibleApplicationHandlers: unsafe extern "system" fn (this: *const nsIHandlerInfo, aPossibleApplicationHandlers: *mut*const nsIMutableArray) -> ::nserror::nsresult,

    /* readonly attribute boolean hasDefaultHandler; */
    pub GetHasDefaultHandler: unsafe extern "system" fn (this: *const nsIHandlerInfo, aHasDefaultHandler: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString defaultDescription; */
    pub GetDefaultDescription: unsafe extern "system" fn (this: *const nsIHandlerInfo, aDefaultDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void launchWithURI (in nsIURI aURI, [optional] in BrowsingContext aBrowsingContext); */
    pub LaunchWithURI: unsafe extern "system" fn (this: *const nsIHandlerInfo, aURI: *const nsIURI, aBrowsingContext: *const libc::c_void) -> ::nserror::nsresult,

    /* attribute nsHandlerInfoAction preferredAction; */
    pub GetPreferredAction: unsafe extern "system" fn (this: *const nsIHandlerInfo, aPreferredAction: *mut nsHandlerInfoAction) -> ::nserror::nsresult,

    /* attribute nsHandlerInfoAction preferredAction; */
    pub SetPreferredAction: unsafe extern "system" fn (this: *const nsIHandlerInfo, aPreferredAction: nsHandlerInfoAction) -> ::nserror::nsresult,

    /* attribute boolean alwaysAskBeforeHandling; */
    pub GetAlwaysAskBeforeHandling: unsafe extern "system" fn (this: *const nsIHandlerInfo, aAlwaysAskBeforeHandling: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean alwaysAskBeforeHandling; */
    pub SetAlwaysAskBeforeHandling: unsafe extern "system" fn (this: *const nsIHandlerInfo, aAlwaysAskBeforeHandling: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHandlerInfo {

    pub const saveToDisk: i64 = 0;

    /// ```text
    /// /**
    ///      * Used to indicate that we know nothing about what to do with this.  You
    ///      * could consider this to be not initialized.
    ///      */
    /// ```
    ///

    pub const alwaysAsk: i64 = 1;


    pub const useHelperApp: i64 = 2;


    pub const handleInternally: i64 = 3;


    pub const useSystemDefault: i64 = 4;

    /// ```text
    /// /**
    ///      * The type of this handler info.  For MIME handlers, this is the MIME type.
    ///      * For protocol handlers, it's the scheme.
    ///      *
    ///      * @return String representing the type.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///      * A human readable description of the handler type
    ///      */
    /// ```
    ///

    /// `attribute AString description;`
    #[inline]
    pub unsafe fn GetDescription(&self, aDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDescription)(self, aDescription)
    }


    /// ```text
    /// /**
    ///      * A human readable description of the handler type
    ///      */
    /// ```
    ///

    /// `attribute AString description;`
    #[inline]
    pub unsafe fn SetDescription(&self, aDescription: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDescription)(self, aDescription)
    }


    /// ```text
    /// /**
    ///      * The application the user has said they want associated with this content
    ///      * type. This is not always guaranteed to be set!!
    ///      */
    /// ```
    ///

    /// `attribute nsIHandlerApp preferredApplicationHandler;`
    #[inline]
    pub unsafe fn GetPreferredApplicationHandler(&self, aPreferredApplicationHandler: *mut*const nsIHandlerApp) -> ::nserror::nsresult {
        ((*self.vtable).GetPreferredApplicationHandler)(self, aPreferredApplicationHandler)
    }


    /// ```text
    /// /**
    ///      * The application the user has said they want associated with this content
    ///      * type. This is not always guaranteed to be set!!
    ///      */
    /// ```
    ///

    /// `attribute nsIHandlerApp preferredApplicationHandler;`
    #[inline]
    pub unsafe fn SetPreferredApplicationHandler(&self, aPreferredApplicationHandler: *const nsIHandlerApp) -> ::nserror::nsresult {
        ((*self.vtable).SetPreferredApplicationHandler)(self, aPreferredApplicationHandler)
    }


    /// ```text
    /// /**
    ///      * Applications that can handle this content type.
    ///      *
    ///      * The list will include the preferred handler, if any.  Elements of this
    ///      * array are nsIHandlerApp objects, and this attribute will always reference
    ///      * an array, whether or not there are any possible handlers.  If there are
    ///      * no possible handlers, the array will contain no elements, so just check
    ///      * its length (nsIArray::length) to see if there are any possible handlers.
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIMutableArray possibleApplicationHandlers;`
    #[inline]
    pub unsafe fn GetPossibleApplicationHandlers(&self, aPossibleApplicationHandlers: *mut*const nsIMutableArray) -> ::nserror::nsresult {
        ((*self.vtable).GetPossibleApplicationHandlers)(self, aPossibleApplicationHandlers)
    }


    /// ```text
    /// /**
    ///      * Indicates whether a default application handler exists,
    ///      * i.e. whether launchWithFile with action = useSystemDefault is possible
    ///      * and defaultDescription will contain usable information.
    ///      */
    /// ```
    ///

    /// `readonly attribute boolean hasDefaultHandler;`
    #[inline]
    pub unsafe fn GetHasDefaultHandler(&self, aHasDefaultHandler: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasDefaultHandler)(self, aHasDefaultHandler)
    }


    /// ```text
    /// /**
    ///      * A pretty name description of the associated default application. Only
    ///      * usable if hasDefaultHandler is true.
    ///      */
    /// ```
    ///

    /// `readonly attribute AString defaultDescription;`
    #[inline]
    pub unsafe fn GetDefaultDescription(&self, aDefaultDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultDescription)(self, aDefaultDescription)
    }


    /// ```text
    /// /**
    ///      * Launches the application with the specified URI, in a way that
    ///      * depends on the value of preferredAction. preferredAction must be
    ///      * useHelperApp or useSystemDefault.
    ///      *
    ///      * @note Only the URI scheme is used to determine how to launch.  This is
    ///      * essentially a pass-by-value operation.  This means that in the case of
    ///      * a file: URI, the handler that is registered for file: will be launched
    ///      * and our code will not make any decision based on the content-type or
    ///      * extension, though the invoked file: handler is free to do so.
    ///      *
    ///      * @param aURI
    ///      *        The URI to launch this application with
    ///      *
    ///      * @param aBrowsingContext
    ///      *        The window to parent the dialog against, and, if a web handler
    ///      *        is chosen, it is loaded in this window as well.  See
    ///      *        nsIHandlerApp.launchWithURI for more details.
    ///      *
    ///      * @throw NS_ERROR_INVALID_ARG if preferredAction is not valid for this
    ///      * call. Other exceptions may be thrown.
    ///      */
    /// ```
    ///

    /// `void launchWithURI (in nsIURI aURI, [optional] in BrowsingContext aBrowsingContext);`
    #[inline]
    pub unsafe fn LaunchWithURI(&self, aURI: *const nsIURI, aBrowsingContext: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).LaunchWithURI)(self, aURI, aBrowsingContext)
    }


    /// ```text
    /// /**
    ///      * preferredAction is how the user specified they would like to handle
    ///      * this content type: save to disk, use specified helper app, use OS
    ///      * default handler or handle using navigator; possible value constants
    ///      * listed below
    ///      */
    /// ```
    ///

    /// `attribute nsHandlerInfoAction preferredAction;`
    #[inline]
    pub unsafe fn GetPreferredAction(&self, aPreferredAction: *mut nsHandlerInfoAction) -> ::nserror::nsresult {
        ((*self.vtable).GetPreferredAction)(self, aPreferredAction)
    }


    /// ```text
    /// /**
    ///      * preferredAction is how the user specified they would like to handle
    ///      * this content type: save to disk, use specified helper app, use OS
    ///      * default handler or handle using navigator; possible value constants
    ///      * listed below
    ///      */
    /// ```
    ///

    /// `attribute nsHandlerInfoAction preferredAction;`
    #[inline]
    pub unsafe fn SetPreferredAction(&self, aPreferredAction: nsHandlerInfoAction) -> ::nserror::nsresult {
        ((*self.vtable).SetPreferredAction)(self, aPreferredAction)
    }


    /// ```text
    /// /**
    ///      * alwaysAskBeforeHandling: if true, we should always give the user a
    ///      * dialog asking how to dispose of this content.
    ///      */
    /// ```
    ///

    /// `attribute boolean alwaysAskBeforeHandling;`
    #[inline]
    pub unsafe fn GetAlwaysAskBeforeHandling(&self, aAlwaysAskBeforeHandling: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAlwaysAskBeforeHandling)(self, aAlwaysAskBeforeHandling)
    }


    /// ```text
    /// /**
    ///      * alwaysAskBeforeHandling: if true, we should always give the user a
    ///      * dialog asking how to dispose of this content.
    ///      */
    /// ```
    ///

    /// `attribute boolean alwaysAskBeforeHandling;`
    #[inline]
    pub unsafe fn SetAlwaysAskBeforeHandling(&self, aAlwaysAskBeforeHandling: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAlwaysAskBeforeHandling)(self, aAlwaysAskBeforeHandling)
    }


}


/// `interface nsIMIMEInfo : nsIHandlerInfo`
///

/// ```text
/// /**
///  * nsIMIMEInfo extends nsIHandlerInfo with a bunch of information specific to
///  * MIME content-types. There is a one-to-many relationship between MIME types
///  * and file extensions. This means that a MIMEInfo object may have multiple
///  * file extensions associated with it.  However, the reverse is not true.
///  *
///  * MIMEInfo objects are generally retrieved from the MIME Service
///  * @see nsIMIMEService
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIMIMEInfo {
    vtable: *const nsIMIMEInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIMIMEInfo.
unsafe impl XpCom for nsIMIMEInfo {
    const IID: nsIID = nsID(0x1c21acef, 0xc7a1, 0x40c6,
        [0x9d, 0x40, 0xa2, 0x04, 0x80, 0xee, 0x53, 0xa1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIMIMEInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIMIMEInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIMIMEInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIMIMEInfo`.
    fn coerce_from(v: &nsIMIMEInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIMIMEInfoCoerce for nsIMIMEInfo {
    #[inline]
    fn coerce_from(v: &nsIMIMEInfo) -> &Self {
        v
    }
}

impl nsIMIMEInfo {
    /// Cast this `nsIMIMEInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIMIMEInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIMIMEInfo {
    type Target = nsIHandlerInfo;
    #[inline]
    fn deref(&self) -> &nsIHandlerInfo {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIHandlerInfoCoerce> nsIMIMEInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIMIMEInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIMIMEInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIMIMEInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIHandlerInfoVTable,

    /* nsIUTF8StringEnumerator getFileExtensions (); */
    pub GetFileExtensions: unsafe extern "system" fn (this: *const nsIMIMEInfo, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult,

    /* void setFileExtensions (in AUTF8String aExtensions); */
    pub SetFileExtensions: unsafe extern "system" fn (this: *const nsIMIMEInfo, aExtensions: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean extensionExists (in AUTF8String aExtension); */
    pub ExtensionExists: unsafe extern "system" fn (this: *const nsIMIMEInfo, aExtension: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void appendExtension (in AUTF8String aExtension); */
    pub AppendExtension: unsafe extern "system" fn (this: *const nsIMIMEInfo, aExtension: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String primaryExtension; */
    pub GetPrimaryExtension: unsafe extern "system" fn (this: *const nsIMIMEInfo, aPrimaryExtension: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String primaryExtension; */
    pub SetPrimaryExtension: unsafe extern "system" fn (this: *const nsIMIMEInfo, aPrimaryExtension: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString MIMEType; */
    pub GetMIMEType: unsafe extern "system" fn (this: *const nsIMIMEInfo, aMIMEType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean equals (in nsIMIMEInfo aMIMEInfo); */
    pub Equals: unsafe extern "system" fn (this: *const nsIMIMEInfo, aMIMEInfo: *const nsIMIMEInfo, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsIArray possibleLocalHandlers; */
    pub GetPossibleLocalHandlers: unsafe extern "system" fn (this: *const nsIMIMEInfo, aPossibleLocalHandlers: *mut*const nsIArray) -> ::nserror::nsresult,

    /* void launchWithFile (in nsIFile aFile); */
    pub LaunchWithFile: unsafe extern "system" fn (this: *const nsIMIMEInfo, aFile: *const nsIFile) -> ::nserror::nsresult,

    /* boolean isCurrentAppOSDefault (); */
    pub IsCurrentAppOSDefault: unsafe extern "system" fn (this: *const nsIMIMEInfo, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIMIMEInfo {

    /// ```text
    /// /**
    ///      * Gives you an array of file types associated with this type.
    ///      *
    ///      * @return Number of elements in the array.
    ///      * @return Array of extensions.
    ///      */
    /// ```
    ///

    /// `nsIUTF8StringEnumerator getFileExtensions ();`
    #[inline]
    pub unsafe fn GetFileExtensions(&self, _retval: *mut*const nsIUTF8StringEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetFileExtensions)(self, _retval)
    }


    /// ```text
    /// /**
    ///      * Set File Extensions. Input is a comma delimited list of extensions.
    ///      */
    /// ```
    ///

    /// `void setFileExtensions (in AUTF8String aExtensions);`
    #[inline]
    pub unsafe fn SetFileExtensions(&self, aExtensions: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetFileExtensions)(self, aExtensions)
    }


    /// ```text
    /// /**
    ///      * Returns whether or not the given extension is
    ///      * associated with this MIME info.
    ///      *
    ///      * @return TRUE if the association exists.
    ///      */
    /// ```
    ///

    /// `boolean extensionExists (in AUTF8String aExtension);`
    #[inline]
    pub unsafe fn ExtensionExists(&self, aExtension: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ExtensionExists)(self, aExtension, _retval)
    }


    /// ```text
    /// /**
    ///      * Append a given extension to the set of extensions
    ///      */
    /// ```
    ///

    /// `void appendExtension (in AUTF8String aExtension);`
    #[inline]
    pub unsafe fn AppendExtension(&self, aExtension: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).AppendExtension)(self, aExtension)
    }


    /// ```text
    /// /**
    ///      * Returns the first extension association in
    ///      * the internal set of extensions.
    ///      *
    ///      * @return The first extension.
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String primaryExtension;`
    #[inline]
    pub unsafe fn GetPrimaryExtension(&self, aPrimaryExtension: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryExtension)(self, aPrimaryExtension)
    }


    /// ```text
    /// /**
    ///      * Returns the first extension association in
    ///      * the internal set of extensions.
    ///      *
    ///      * @return The first extension.
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String primaryExtension;`
    #[inline]
    pub unsafe fn SetPrimaryExtension(&self, aPrimaryExtension: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetPrimaryExtension)(self, aPrimaryExtension)
    }


    /// ```text
    /// /**
    ///      * The MIME type of this MIMEInfo.
    ///      *
    ///      * @return String representing the MIME type.
    ///      *
    ///      * @deprecated  use nsIHandlerInfo::type instead.
    ///      */
    /// ```
    ///

    /// `readonly attribute ACString MIMEType;`
    #[inline]
    pub unsafe fn GetMIMEType(&self, aMIMEType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMIMEType)(self, aMIMEType)
    }


    /// ```text
    /// /**
    ///      * Returns whether or not these two nsIMIMEInfos are logically
    ///      * equivalent.
    ///      *
    ///      * @returns PR_TRUE if the two are considered equal
    ///      */
    /// ```
    ///

    /// `boolean equals (in nsIMIMEInfo aMIMEInfo);`
    #[inline]
    pub unsafe fn Equals(&self, aMIMEInfo: *const nsIMIMEInfo, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Equals)(self, aMIMEInfo, _retval)
    }


    /// ```text
    /// /**
    ///      * Returns a list of nsILocalHandlerApp objects containing
    ///      * handlers associated with this mimeinfo. Implemented per
    ///      * platform using information in this object to generate the
    ///      * best list. Typically used for an "open with" style user
    ///      * option.
    ///      *
    ///      * @return nsIArray of nsILocalHandlerApp
    ///      */
    /// ```
    ///

    /// `readonly attribute nsIArray possibleLocalHandlers;`
    #[inline]
    pub unsafe fn GetPossibleLocalHandlers(&self, aPossibleLocalHandlers: *mut*const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).GetPossibleLocalHandlers)(self, aPossibleLocalHandlers)
    }


    /// ```text
    /// /**
    ///      * Launches the application with the specified file, in a way that
    ///      * depends on the value of preferredAction. preferredAction must be
    ///      * useHelperApp or useSystemDefault.
    ///      *
    ///      * @param aFile The file to launch this application with.
    ///      *
    ///      * @throw NS_ERROR_INVALID_ARG if action is not valid for this function.
    ///      * Other exceptions may be thrown.
    ///      */
    /// ```
    ///

    /// `void launchWithFile (in nsIFile aFile);`
    #[inline]
    pub unsafe fn LaunchWithFile(&self, aFile: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).LaunchWithFile)(self, aFile)
    }


    /// ```text
    /// /**
    ///      * Check if we ourselves are registered as the OS default for this type.
    ///      */
    /// ```
    ///

    /// `boolean isCurrentAppOSDefault ();`
    #[inline]
    pub unsafe fn IsCurrentAppOSDefault(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsCurrentAppOSDefault)(self, _retval)
    }


}


/// `interface nsIHandlerApp : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHandlerApp {
    vtable: *const nsIHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHandlerApp.
unsafe impl XpCom for nsIHandlerApp {
    const IID: nsIID = nsID(0x8bdf20a4, 0x9170, 0x4548,
        [0xaf, 0x52, 0x78, 0x31, 0x1a, 0x44, 0xf9, 0x20]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHandlerApp {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHandlerApp.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHandlerAppCoerce {
    /// Cheaply cast a value of this type from a `nsIHandlerApp`.
    fn coerce_from(v: &nsIHandlerApp) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHandlerAppCoerce for nsIHandlerApp {
    #[inline]
    fn coerce_from(v: &nsIHandlerApp) -> &Self {
        v
    }
}

impl nsIHandlerApp {
    /// Cast this `nsIHandlerApp` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHandlerApp {
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
impl<T: nsISupportsCoerce> nsIHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHandlerApp
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHandlerAppVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIHandlerApp, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString name; */
    pub SetName: unsafe extern "system" fn (this: *const nsIHandlerApp, aName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString detailedDescription; */
    pub GetDetailedDescription: unsafe extern "system" fn (this: *const nsIHandlerApp, aDetailedDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString detailedDescription; */
    pub SetDetailedDescription: unsafe extern "system" fn (this: *const nsIHandlerApp, aDetailedDescription: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean equals (in nsIHandlerApp aHandlerApp); */
    pub Equals: unsafe extern "system" fn (this: *const nsIHandlerApp, aHandlerApp: *const nsIHandlerApp, _retval: *mut bool) -> ::nserror::nsresult,

    /* void launchWithURI (in nsIURI aURI, [optional] in BrowsingContext aBrowsingContext); */
    pub LaunchWithURI: unsafe extern "system" fn (this: *const nsIHandlerApp, aURI: *const nsIURI, aBrowsingContext: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHandlerApp {

    /// ```text
    /// /**
    ///  * nsIHandlerApp represents an external application that can handle content
    ///  * of some sort (either a MIME type or a protocol).
    ///  *
    ///  * FIXME: now that we've made nsIWebHandlerApp inherit from nsIHandlerApp,
    ///  * we should also try to make nsIWebContentHandlerInfo inherit from or possibly
    ///  * be replaced by nsIWebHandlerApp (bug 394710).
    ///  */
    /// /**
    ///      * Human readable name for the handler
    ///      */
    /// ```
    ///

    /// `attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///  * nsIHandlerApp represents an external application that can handle content
    ///  * of some sort (either a MIME type or a protocol).
    ///  *
    ///  * FIXME: now that we've made nsIWebHandlerApp inherit from nsIHandlerApp,
    ///  * we should also try to make nsIWebContentHandlerInfo inherit from or possibly
    ///  * be replaced by nsIWebHandlerApp (bug 394710).
    ///  */
    /// /**
    ///      * Human readable name for the handler
    ///      */
    /// ```
    ///

    /// `attribute AString name;`
    #[inline]
    pub unsafe fn SetName(&self, aName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetName)(self, aName)
    }


    /// ```text
    /// /**
    ///      * Detailed description for this handler. Suitable for
    ///      * a tooltip or short informative sentence.
    ///      */
    /// ```
    ///

    /// `attribute AString detailedDescription;`
    #[inline]
    pub unsafe fn GetDetailedDescription(&self, aDetailedDescription: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDetailedDescription)(self, aDetailedDescription)
    }


    /// ```text
    /// /**
    ///      * Detailed description for this handler. Suitable for
    ///      * a tooltip or short informative sentence.
    ///      */
    /// ```
    ///

    /// `attribute AString detailedDescription;`
    #[inline]
    pub unsafe fn SetDetailedDescription(&self, aDetailedDescription: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetDetailedDescription)(self, aDetailedDescription)
    }


    /// ```text
    /// /**
    ///      * Whether or not the given handler app is logically equivalent to the
    ///      * invokant (i.e. they represent the same app).
    ///      *
    ///      * Two apps are the same if they are both either local or web handlers
    ///      * and their executables/URI templates and command line parameters are
    ///      * the same.
    ///      *
    ///      * @param aHandlerApp the handler app to compare to the invokant
    ///      *
    ///      * @returns true if the two are logically equivalent, false otherwise
    ///      */
    /// ```
    ///

    /// `boolean equals (in nsIHandlerApp aHandlerApp);`
    #[inline]
    pub unsafe fn Equals(&self, aHandlerApp: *const nsIHandlerApp, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Equals)(self, aHandlerApp, _retval)
    }


    /// ```text
    /// /**
    ///      * Launches the application with the specified URI.
    ///      *
    ///      * @param aURI
    ///      *        The URI to launch this application with
    ///      *
    ///      * @param aBrowsingContext
    ///      *
    ///      *        This represents the docshell to load the handler in and is passed
    ///      *        through to nsIURILoader.openURI.  If this parameter is null or
    ///      *        not present, the web handler app implementation will attempt to
    ///      *        find/create a place to load the handler and do so.  As of this
    ///      *        writing, it tries to load the web handler in a new window using
    ///      *        nsIBrowserDOMWindow.openURI.  In the future, it may attempt to
    ///      *        have a more comprehensive strategy which could include handing
    ///      *        off to the system default browser (bug 394479).
    ///      */
    /// ```
    ///

    /// `void launchWithURI (in nsIURI aURI, [optional] in BrowsingContext aBrowsingContext);`
    #[inline]
    pub unsafe fn LaunchWithURI(&self, aURI: *const nsIURI, aBrowsingContext: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).LaunchWithURI)(self, aURI, aBrowsingContext)
    }


}


/// `interface nsILocalHandlerApp : nsIHandlerApp`
///

/// ```text
/// /**
///  * nsILocalHandlerApp is a local OS-level executable
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILocalHandlerApp {
    vtable: *const nsILocalHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILocalHandlerApp.
unsafe impl XpCom for nsILocalHandlerApp {
    const IID: nsIID = nsID(0xd36b6329, 0x52ae, 0x4f45,
        [0x80, 0xf4, 0xb2, 0x53, 0x6a, 0xe5, 0xf8, 0xb2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILocalHandlerApp {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILocalHandlerApp.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILocalHandlerAppCoerce {
    /// Cheaply cast a value of this type from a `nsILocalHandlerApp`.
    fn coerce_from(v: &nsILocalHandlerApp) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILocalHandlerAppCoerce for nsILocalHandlerApp {
    #[inline]
    fn coerce_from(v: &nsILocalHandlerApp) -> &Self {
        v
    }
}

impl nsILocalHandlerApp {
    /// Cast this `nsILocalHandlerApp` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILocalHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILocalHandlerApp {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIHandlerAppCoerce> nsILocalHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILocalHandlerApp
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILocalHandlerAppVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIHandlerAppVTable,

    /* attribute nsIFile executable; */
    pub GetExecutable: unsafe extern "system" fn (this: *const nsILocalHandlerApp, aExecutable: *mut*const nsIFile) -> ::nserror::nsresult,

    /* attribute nsIFile executable; */
    pub SetExecutable: unsafe extern "system" fn (this: *const nsILocalHandlerApp, aExecutable: *const nsIFile) -> ::nserror::nsresult,

    /* readonly attribute unsigned long parameterCount; */
    pub GetParameterCount: unsafe extern "system" fn (this: *const nsILocalHandlerApp, aParameterCount: *mut u32) -> ::nserror::nsresult,

    /* void clearParameters (); */
    pub ClearParameters: unsafe extern "system" fn (this: *const nsILocalHandlerApp) -> ::nserror::nsresult,

    /* void appendParameter (in AString param); */
    pub AppendParameter: unsafe extern "system" fn (this: *const nsILocalHandlerApp, param: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getParameter (in unsigned long parameterIndex); */
    pub GetParameter: unsafe extern "system" fn (this: *const nsILocalHandlerApp, parameterIndex: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean parameterExists (in AString param); */
    pub ParameterExists: unsafe extern "system" fn (this: *const nsILocalHandlerApp, param: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILocalHandlerApp {

    /// ```text
    /// /**
    ///      * Pointer to the executable file used to handle content
    ///      */
    /// ```
    ///

    /// `attribute nsIFile executable;`
    #[inline]
    pub unsafe fn GetExecutable(&self, aExecutable: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetExecutable)(self, aExecutable)
    }


    /// ```text
    /// /**
    ///      * Pointer to the executable file used to handle content
    ///      */
    /// ```
    ///

    /// `attribute nsIFile executable;`
    #[inline]
    pub unsafe fn SetExecutable(&self, aExecutable: *const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).SetExecutable)(self, aExecutable)
    }


    /// ```text
    /// /**
    ///      * Returns the current number of command line parameters.
    ///      */
    /// ```
    ///

    /// `readonly attribute unsigned long parameterCount;`
    #[inline]
    pub unsafe fn GetParameterCount(&self, aParameterCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetParameterCount)(self, aParameterCount)
    }


    /// ```text
    /// /**
    ///      * Clears the current list of command line parameters.
    ///      */
    /// ```
    ///

    /// `void clearParameters ();`
    #[inline]
    pub unsafe fn ClearParameters(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearParameters)(self, )
    }


    /// ```text
    /// /**
    ///      * Appends a command line parameter to the command line
    ///      * parameter list.
    ///      *
    ///      * @param param the parameter to add.
    ///      */
    /// ```
    ///

    /// `void appendParameter (in AString param);`
    #[inline]
    pub unsafe fn AppendParameter(&self, param: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).AppendParameter)(self, param)
    }


    /// ```text
    /// /**
    ///      * Retrieves a specific command line parameter.
    ///      *
    ///      * @param param the index of the parameter to return.
    ///      *
    ///      * @return the parameter string.
    ///      *
    ///      * @throw NS_ERROR_INVALID_ARG if the index is out of range.
    ///      */
    /// ```
    ///

    /// `AString getParameter (in unsigned long parameterIndex);`
    #[inline]
    pub unsafe fn GetParameter(&self, parameterIndex: u32, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetParameter)(self, parameterIndex, _retval)
    }


    /// ```text
    /// /**
    ///      * Checks to see if a parameter exists in the command line
    ///      * parameter list.
    ///      *
    ///      * @param param the parameter to search for.
    ///      *
    ///      * @return TRUE if the parameter exists in the current list.
    ///      */
    /// ```
    ///

    /// `boolean parameterExists (in AString param);`
    #[inline]
    pub unsafe fn ParameterExists(&self, param: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ParameterExists)(self, param, _retval)
    }


}


/// `interface nsIWebHandlerApp : nsIHandlerApp`
///

/// ```text
/// /**
///  * nsIWebHandlerApp is a web-based handler, as speced by the WhatWG HTML5
///  * draft.  Currently, only GET-based handlers are supported.  At some point,
///  * we probably want to work with WhatWG to spec out and implement POST-based
///  * handlers as well.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebHandlerApp {
    vtable: *const nsIWebHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebHandlerApp.
unsafe impl XpCom for nsIWebHandlerApp {
    const IID: nsIID = nsID(0x7521a093, 0xc498, 0x45ce,
        [0xb4, 0x62, 0xdf, 0x7b, 0xa0, 0xd8, 0x82, 0xf6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebHandlerApp {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebHandlerApp.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebHandlerAppCoerce {
    /// Cheaply cast a value of this type from a `nsIWebHandlerApp`.
    fn coerce_from(v: &nsIWebHandlerApp) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebHandlerAppCoerce for nsIWebHandlerApp {
    #[inline]
    fn coerce_from(v: &nsIWebHandlerApp) -> &Self {
        v
    }
}

impl nsIWebHandlerApp {
    /// Cast this `nsIWebHandlerApp` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebHandlerApp {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIHandlerAppCoerce> nsIWebHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebHandlerApp
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebHandlerAppVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIHandlerAppVTable,

    /* attribute AUTF8String uriTemplate; */
    pub GetUriTemplate: unsafe extern "system" fn (this: *const nsIWebHandlerApp, aUriTemplate: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String uriTemplate; */
    pub SetUriTemplate: unsafe extern "system" fn (this: *const nsIWebHandlerApp, aUriTemplate: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebHandlerApp {

    /// ```text
    /// /**
    ///      * Template used to construct the URI to GET.  Template is expected to have
    ///      * a %s in it, and the escaped URI to be handled is inserted in place of
    ///      * that %s, as per the HTML5 spec.
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String uriTemplate;`
    #[inline]
    pub unsafe fn GetUriTemplate(&self, aUriTemplate: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUriTemplate)(self, aUriTemplate)
    }


    /// ```text
    /// /**
    ///      * Template used to construct the URI to GET.  Template is expected to have
    ///      * a %s in it, and the escaped URI to be handled is inserted in place of
    ///      * that %s, as per the HTML5 spec.
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String uriTemplate;`
    #[inline]
    pub unsafe fn SetUriTemplate(&self, aUriTemplate: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetUriTemplate)(self, aUriTemplate)
    }


}


/// `interface nsIDBusHandlerApp : nsIHandlerApp`
///

/// ```text
/// /**
///  * nsIDBusHandlerApp represents local applications launched by DBus a message
///  * invoking a method taking a single string argument descibing a URI
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDBusHandlerApp {
    vtable: *const nsIDBusHandlerAppVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDBusHandlerApp.
unsafe impl XpCom for nsIDBusHandlerApp {
    const IID: nsIID = nsID(0x1ffc274b, 0x4cbf, 0x4bb5,
        [0xa6, 0x35, 0x05, 0xad, 0x2c, 0xbb, 0x65, 0x34]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDBusHandlerApp {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDBusHandlerApp.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDBusHandlerAppCoerce {
    /// Cheaply cast a value of this type from a `nsIDBusHandlerApp`.
    fn coerce_from(v: &nsIDBusHandlerApp) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDBusHandlerAppCoerce for nsIDBusHandlerApp {
    #[inline]
    fn coerce_from(v: &nsIDBusHandlerApp) -> &Self {
        v
    }
}

impl nsIDBusHandlerApp {
    /// Cast this `nsIDBusHandlerApp` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDBusHandlerAppCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDBusHandlerApp {
    type Target = nsIHandlerApp;
    #[inline]
    fn deref(&self) -> &nsIHandlerApp {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIHandlerAppCoerce> nsIDBusHandlerAppCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDBusHandlerApp) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDBusHandlerApp
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDBusHandlerAppVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIHandlerAppVTable,

    /* attribute AUTF8String service; */
    pub GetService: unsafe extern "system" fn (this: *const nsIDBusHandlerApp, aService: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String service; */
    pub SetService: unsafe extern "system" fn (this: *const nsIDBusHandlerApp, aService: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String objectPath; */
    pub GetObjectPath: unsafe extern "system" fn (this: *const nsIDBusHandlerApp, aObjectPath: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String objectPath; */
    pub SetObjectPath: unsafe extern "system" fn (this: *const nsIDBusHandlerApp, aObjectPath: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String dBusInterface; */
    pub GetDBusInterface: unsafe extern "system" fn (this: *const nsIDBusHandlerApp, aDBusInterface: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String dBusInterface; */
    pub SetDBusInterface: unsafe extern "system" fn (this: *const nsIDBusHandlerApp, aDBusInterface: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String method; */
    pub GetMethod: unsafe extern "system" fn (this: *const nsIDBusHandlerApp, aMethod: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String method; */
    pub SetMethod: unsafe extern "system" fn (this: *const nsIDBusHandlerApp, aMethod: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDBusHandlerApp {

    /// ```text
    /// /**
    ///      * Service defines the dbus service that should handle this protocol.
    ///      * If its not set,  NS_ERROR_FAILURE will be returned by LaunchWithURI
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String service;`
    #[inline]
    pub unsafe fn GetService(&self, aService: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetService)(self, aService)
    }


    /// ```text
    /// /**
    ///      * Service defines the dbus service that should handle this protocol.
    ///      * If its not set,  NS_ERROR_FAILURE will be returned by LaunchWithURI
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String service;`
    #[inline]
    pub unsafe fn SetService(&self, aService: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetService)(self, aService)
    }


    /// ```text
    /// /**
    ///      * Objpath defines the object path of the dbus service that should handle
    ///      * this protocol. If its not set,  NS_ERROR_FAILURE will be returned
    ///      * by LaunchWithURI
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String objectPath;`
    #[inline]
    pub unsafe fn GetObjectPath(&self, aObjectPath: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetObjectPath)(self, aObjectPath)
    }


    /// ```text
    /// /**
    ///      * Objpath defines the object path of the dbus service that should handle
    ///      * this protocol. If its not set,  NS_ERROR_FAILURE will be returned
    ///      * by LaunchWithURI
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String objectPath;`
    #[inline]
    pub unsafe fn SetObjectPath(&self, aObjectPath: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetObjectPath)(self, aObjectPath)
    }


    /// ```text
    /// /**
    ///      * DBusInterface defines the interface of the dbus service that should
    ///      * handle this protocol. If its not set,  NS_ERROR_FAILURE will be
    ///      * returned by LaunchWithURI
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String dBusInterface;`
    #[inline]
    pub unsafe fn GetDBusInterface(&self, aDBusInterface: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDBusInterface)(self, aDBusInterface)
    }


    /// ```text
    /// /**
    ///      * DBusInterface defines the interface of the dbus service that should
    ///      * handle this protocol. If its not set,  NS_ERROR_FAILURE will be
    ///      * returned by LaunchWithURI
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String dBusInterface;`
    #[inline]
    pub unsafe fn SetDBusInterface(&self, aDBusInterface: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetDBusInterface)(self, aDBusInterface)
    }


    /// ```text
    /// /**
    ///      * Method defines the dbus method that should be invoked to handle this
    ///      * protocol. If its not set,  NS_ERROR_FAILURE will be returned by
    ///      * LaunchWithURI
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String method;`
    #[inline]
    pub unsafe fn GetMethod(&self, aMethod: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetMethod)(self, aMethod)
    }


    /// ```text
    /// /**
    ///      * Method defines the dbus method that should be invoked to handle this
    ///      * protocol. If its not set,  NS_ERROR_FAILURE will be returned by
    ///      * LaunchWithURI
    ///      */
    /// ```
    ///

    /// `attribute AUTF8String method;`
    #[inline]
    pub unsafe fn SetMethod(&self, aMethod: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetMethod)(self, aMethod)
    }


}


