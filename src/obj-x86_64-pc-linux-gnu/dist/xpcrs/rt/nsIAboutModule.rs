//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/about/nsIAboutModule.idl
//


/// `interface nsIAboutModule : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAboutModule {
    vtable: *const nsIAboutModuleVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAboutModule.
unsafe impl XpCom for nsIAboutModule {
    const IID: nsIID = nsID(0xc0c19db9, 0x1b5a, 0x4ac5,
        [0xb6, 0x56, 0xed, 0x6f, 0x81, 0x49, 0xfa, 0x48]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAboutModule {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAboutModule.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAboutModuleCoerce {
    /// Cheaply cast a value of this type from a `nsIAboutModule`.
    fn coerce_from(v: &nsIAboutModule) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAboutModuleCoerce for nsIAboutModule {
    #[inline]
    fn coerce_from(v: &nsIAboutModule) -> &Self {
        v
    }
}

impl nsIAboutModule {
    /// Cast this `nsIAboutModule` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAboutModuleCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAboutModule {
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
impl<T: nsISupportsCoerce> nsIAboutModuleCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAboutModule) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAboutModule
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAboutModuleVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadInfo); */
    pub NewChannel: unsafe extern "system" fn (this: *const nsIAboutModule, aURI: *const nsIURI, aLoadInfo: *const nsILoadInfo, _retval: *mut*const nsIChannel) -> ::nserror::nsresult,

    /* unsigned long getURIFlags (in nsIURI aURI); */
    pub GetURIFlags: unsafe extern "system" fn (this: *const nsIAboutModule, aURI: *const nsIURI, _retval: *mut u32) -> ::nserror::nsresult,

    /* nsIURI getChromeURI (in nsIURI aURI); */
    pub GetChromeURI: unsafe extern "system" fn (this: *const nsIAboutModule, aURI: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAboutModule {
    /// ```text
    /// /**
    ///      * A flag that indicates whether a URI should be run with content
    ///      * privileges. If it is, the about: protocol handler will enforce that
    ///      * the principal of channels created for it be based on their
    ///      * originalURI or URI (depending on the channel flags), by setting
    ///      * their "owner" to null.
    ///      * If content needs to be able to link to this URI, specify
    ///      * URI_CONTENT_LINKABLE as well.
    ///      */
    /// ```
    ///

    pub const URI_SAFE_FOR_UNTRUSTED_CONTENT: i64 = 1;

    /// ```text
    /// /**
    ///      * A flag that indicates whether script should be enabled for the
    ///      * given about: URI even if it's disabled in general.
    ///      */
    /// ```
    ///

    pub const ALLOW_SCRIPT: i64 = 2;

    /// ```text
    /// /**
    ///      * A flag that indicates whether this about: URI doesn't want to be listed
    ///      * in about:about, especially if it's not useful without a query string.
    ///      */
    /// ```
    ///

    pub const HIDE_FROM_ABOUTABOUT: i64 = 4;

    /// ```text
    /// /**
    ///      * A flag that indicates whether this about: URI wants Indexed DB enabled.
    ///      */
    /// ```
    ///

    pub const ENABLE_INDEXED_DB: i64 = 8;

    /// ```text
    /// /**
    ///      * A flag that indicates that this URI can be loaded in a child process
    ///      */
    /// ```
    ///

    pub const URI_CAN_LOAD_IN_CHILD: i64 = 16;

    /// ```text
    /// /**
    ///      * A flag that indicates that this URI must be loaded in a child process
    ///      */
    /// ```
    ///

    pub const URI_MUST_LOAD_IN_CHILD: i64 = 32;

    /// ```text
    /// /**
    ///      * Obsolete. This flag no longer has any effect and will be removed in future.
    ///      */
    /// ```
    ///

    pub const MAKE_UNLINKABLE: i64 = 64;

    /// ```text
    /// /**
    ///      * A flag that indicates that this URI should be linkable from content.
    ///      * Ignored unless URI_SAFE_FOR_UNTRUSTED_CONTENT is also specified.
    ///      *
    ///      * When adding a new about module with this flag make sure to also update
    ///      * IsSafeToLinkForUntrustedContent() in nsAboutProtocolHandler.cpp
    ///      */
    /// ```
    ///

    pub const MAKE_LINKABLE: i64 = 128;

    /// ```text
    /// /**
    ///      * A flag that indicates that this URI can be loaded in the privileged
    ///      * activity stream content process if said process is enabled. Ignored unless
    ///      * URI_MUST_LOAD_IN_CHILD is also specified.
    ///      */
    /// ```
    ///

    pub const URI_CAN_LOAD_IN_PRIVILEGEDABOUT_PROCESS: i64 = 256;

    /// ```text
    /// /**
    ///      * A flag that indicates that this URI must be loaded in an extension process (if available).
    ///      */
    /// ```
    ///

    pub const URI_MUST_LOAD_IN_EXTENSION_PROCESS: i64 = 512;

    /// ```text
    /// /**
    ///      * A flag that indicates that this about: URI needs to allow unsanitized content.
    ///      * Only to be used by about:home and about:newtab.
    ///      */
    /// ```
    ///

    pub const ALLOW_UNSANITIZED_CONTENT: i64 = 1024;

    /// ```text
    /// /**
    ///      * Constructs a new channel for the about protocol module.
    ///      *
    ///      * @param aURI the uri of the new channel
    ///      * @param aLoadInfo the loadinfo of the new channel
    ///      */
    /// ```
    ///

    /// `nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadInfo);`
    #[inline]
    pub unsafe fn NewChannel(&self, aURI: *const nsIURI, aLoadInfo: *const nsILoadInfo, _retval: *mut*const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).NewChannel)(self, aURI, aLoadInfo, _retval)
    }


    /// ```text
    /// /**
    ///      * A method to get the flags that apply to a given about: URI.  The URI
    ///      * passed in is guaranteed to be one of the URIs that this module
    ///      * registered to deal with.
    ///      */
    /// ```
    ///

    /// `unsigned long getURIFlags (in nsIURI aURI);`
    #[inline]
    pub unsafe fn GetURIFlags(&self, aURI: *const nsIURI, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetURIFlags)(self, aURI, _retval)
    }


    /// ```text
    /// /**
    ///      * A method to get the chrome URI that corresponds to a given about URI.
    ///      */
    /// ```
    ///

    /// `nsIURI getChromeURI (in nsIURI aURI);`
    #[inline]
    pub unsafe fn GetChromeURI(&self, aURI: *const nsIURI, _retval: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetChromeURI)(self, aURI, _retval)
    }


}


