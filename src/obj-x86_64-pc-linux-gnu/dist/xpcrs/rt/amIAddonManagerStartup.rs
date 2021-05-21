//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/mozapps/extensions/amIAddonManagerStartup.idl
//


/// `interface amIAddonManagerStartup : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct amIAddonManagerStartup {
    vtable: *const amIAddonManagerStartupVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for amIAddonManagerStartup.
unsafe impl XpCom for amIAddonManagerStartup {
    const IID: nsIID = nsID(0x01dfa47b, 0x87e4, 0x4135,
        [0x87, 0x7b, 0x58, 0x6d, 0x03, 0x3e, 0x1b, 0x5d]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for amIAddonManagerStartup {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from amIAddonManagerStartup.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait amIAddonManagerStartupCoerce {
    /// Cheaply cast a value of this type from a `amIAddonManagerStartup`.
    fn coerce_from(v: &amIAddonManagerStartup) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl amIAddonManagerStartupCoerce for amIAddonManagerStartup {
    #[inline]
    fn coerce_from(v: &amIAddonManagerStartup) -> &Self {
        v
    }
}

impl amIAddonManagerStartup {
    /// Cast this `amIAddonManagerStartup` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: amIAddonManagerStartupCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for amIAddonManagerStartup {
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
impl<T: nsISupportsCoerce> amIAddonManagerStartupCoerce for T {
    #[inline]
    fn coerce_from(v: &amIAddonManagerStartup) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every amIAddonManagerStartup
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct amIAddonManagerStartupVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] jsval readStartupData (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub ReadStartupData: *const ::libc::c_void,

    /* [implicit_jscontext] nsIJSRAIIHelper registerChrome (in nsIURI manifestURI, in jsval entries); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RegisterChrome: *const ::libc::c_void,

    /* [implicit_jscontext] jsval encodeBlob (in jsval value); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub EncodeBlob: *const ::libc::c_void,

    /* [implicit_jscontext] jsval decodeBlob (in jsval value); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub DecodeBlob: *const ::libc::c_void,

    /* Array<AString> enumerateJAR (in nsIURI uri, in AUTF8String pattern); */
    pub EnumerateJAR: unsafe extern "system" fn (this: *const amIAddonManagerStartup, uri: *const nsIURI, pattern: *const ::nsstring::nsACString, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* Array<AString> enumerateJARSubtree (in nsIURI uri); */
    pub EnumerateJARSubtree: unsafe extern "system" fn (this: *const amIAddonManagerStartup, uri: *const nsIURI, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult,

    /* void initializeURLPreloader (); */
    pub InitializeURLPreloader: unsafe extern "system" fn (this: *const amIAddonManagerStartup) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl amIAddonManagerStartup {

    /// ```text
    /// /**
    ///    * Reads and parses startup data from the addonState.json.lz4 file, checks
    ///    * for modifications, and returns the result.
    ///    *
    ///    * Returns null for an empty or nonexistent state file, but throws for an
    ///    * invalid one.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] jsval readStartupData ();`
    const _ReadStartupData: () = ();

    /// ```text
    /// /**
    ///    * Registers a set of dynamic chrome registry entries, and returns an object
    ///    * with a `destruct()` method which must be called in order to unregister
    ///    * the entries.
    ///    *
    ///    * @param manifestURI The base manifest URI for the entries. URL values are
    ///    *        resolved relative to this URI.
    ///    * @param entries An array of arrays, each containing a registry entry as it
    ///    *        would appar in a chrome.manifest file. Only the following entry
    ///    *        types are currently accepted:
    ///    *
    ///    *         - "locale" A locale package entry. Must be a 4-element array.
    ///    *         - "override" A URL override entry. Must be a 3-element array.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] nsIJSRAIIHelper registerChrome (in nsIURI manifestURI, in jsval entries);`
    const _RegisterChrome: () = ();


    /// `[implicit_jscontext] jsval encodeBlob (in jsval value);`
    const _EncodeBlob: () = ();


    /// `[implicit_jscontext] jsval decodeBlob (in jsval value);`
    const _DecodeBlob: () = ();

    /// ```text
    /// /**
    ///    * Enumerates over all entries in the JAR file at the given URI, and returns
    ///    * an array of entry paths which match the given pattern. The URI may be
    ///    * either a file: URL pointing directly to a zip file, or a jar: URI
    ///    * pointing to a zip file nested within another zip file. Only one level of
    ///    * nesting is supported.
    ///    *
    ///    * This should be used in preference to manually opening or retrieving a
    ///    * ZipReader from the zip cache, since the former causes main thread IO and
    ///    * the latter can lead to file locking issues due to unpredictable GC behavior
    ///    * keeping the cached ZipReader alive after the cache is flushed.
    ///    *
    ///    * @param uri The URI of the zip file to enumerate.
    ///    * @param pattern The pattern to match, as passed to nsIZipReader.findEntries.
    ///    */
    /// ```
    ///

    /// `Array<AString> enumerateJAR (in nsIURI uri, in AUTF8String pattern);`
    #[inline]
    pub unsafe fn EnumerateJAR(&self, uri: *const nsIURI, pattern: *const ::nsstring::nsACString, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).EnumerateJAR)(self, uri, pattern, _retval)
    }


    /// ```text
    /// /**
    ///    * Similar to |enumerateJAR| above, but accepts the URI of a directory
    ///    * within a JAR file, and returns a list of all entries below it.
    ///    *
    ///    * The given URI must be a jar: URI, and its JAR file must point either to a
    ///    * file: URI, or to a singly-nested JAR within another JAR file (i.e.,
        ///    * "jar:file:///thing.jar!/" or "jar:jar:file:///thing.jar!/stuff.jar!/").
    ///    * Multiple levels of nesting are not supported.
    ///    */
    /// ```
    ///

    /// `Array<AString> enumerateJARSubtree (in nsIURI uri);`
    #[inline]
    pub unsafe fn EnumerateJARSubtree(&self, uri: *const nsIURI, _retval: *mut thin_vec::ThinVec<::nsstring::nsString>) -> ::nserror::nsresult {
        ((*self.vtable).EnumerateJARSubtree)(self, uri, _retval)
    }


    /// ```text
    /// /**
    ///    * Initializes the URL Preloader.
    ///    *
    ///    * NOT FOR USE OUTSIDE OF UNIT TESTS.
    ///    */
    /// ```
    ///

    /// `void initializeURLPreloader ();`
    #[inline]
    pub unsafe fn InitializeURLPreloader(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).InitializeURLPreloader)(self, )
    }


}


