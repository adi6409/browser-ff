//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/windowwatcher/nsIOpenWindowInfo.idl
//


/// `interface nsIBrowsingContextReadyCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBrowsingContextReadyCallback {
    vtable: *const nsIBrowsingContextReadyCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBrowsingContextReadyCallback.
unsafe impl XpCom for nsIBrowsingContextReadyCallback {
    const IID: nsIID = nsID(0x0524ee06, 0x7f4c, 0x4cd3,
        [0xab, 0x80, 0x08, 0x45, 0x62, 0x74, 0x5c, 0xad]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBrowsingContextReadyCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBrowsingContextReadyCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBrowsingContextReadyCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIBrowsingContextReadyCallback`.
    fn coerce_from(v: &nsIBrowsingContextReadyCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBrowsingContextReadyCallbackCoerce for nsIBrowsingContextReadyCallback {
    #[inline]
    fn coerce_from(v: &nsIBrowsingContextReadyCallback) -> &Self {
        v
    }
}

impl nsIBrowsingContextReadyCallback {
    /// Cast this `nsIBrowsingContextReadyCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBrowsingContextReadyCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBrowsingContextReadyCallback {
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
impl<T: nsISupportsCoerce> nsIBrowsingContextReadyCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBrowsingContextReadyCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBrowsingContextReadyCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBrowsingContextReadyCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void browsingContextReady (in BrowsingContext bc); */
    pub BrowsingContextReady: unsafe extern "system" fn (this: *const nsIBrowsingContextReadyCallback, bc: *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBrowsingContextReadyCallback {


    /// `void browsingContextReady (in BrowsingContext bc);`
    #[inline]
    pub unsafe fn BrowsingContextReady(&self, bc: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).BrowsingContextReady)(self, bc)
    }


}


/// `interface nsIOpenWindowInfo : nsISupports`
///

/// ```text
/// /**
///  * nsIOpenWindowInfo is a helper type which contains details used when opening
///  * new content windows. This object is used to correctly create new initial
///  * content documents when creating a new window.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOpenWindowInfo {
    vtable: *const nsIOpenWindowInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOpenWindowInfo.
unsafe impl XpCom for nsIOpenWindowInfo {
    const IID: nsIID = nsID(0x30359edb, 0x126c, 0x4f65,
        [0xae, 0x80, 0x07, 0xfb, 0x15, 0x86, 0x97, 0xf9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOpenWindowInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOpenWindowInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOpenWindowInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIOpenWindowInfo`.
    fn coerce_from(v: &nsIOpenWindowInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOpenWindowInfoCoerce for nsIOpenWindowInfo {
    #[inline]
    fn coerce_from(v: &nsIOpenWindowInfo) -> &Self {
        v
    }
}

impl nsIOpenWindowInfo {
    /// Cast this `nsIOpenWindowInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOpenWindowInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOpenWindowInfo {
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
impl<T: nsISupportsCoerce> nsIOpenWindowInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOpenWindowInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOpenWindowInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOpenWindowInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute BrowsingContext parent; */
    pub GetParent: unsafe extern "system" fn (this: *const nsIOpenWindowInfo, aParent: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isRemote; */
    pub GetIsRemote: unsafe extern "system" fn (this: *const nsIOpenWindowInfo, aIsRemote: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean forceNoOpener; */
    pub GetForceNoOpener: unsafe extern "system" fn (this: *const nsIOpenWindowInfo, aForceNoOpener: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isForPrinting; */
    pub GetIsForPrinting: unsafe extern "system" fn (this: *const nsIOpenWindowInfo, aIsForPrinting: *mut bool) -> ::nserror::nsresult,

    /* [infallible] readonly attribute boolean isForWindowDotPrint; */
    pub GetIsForWindowDotPrint: unsafe extern "system" fn (this: *const nsIOpenWindowInfo, aIsForWindowDotPrint: *mut bool) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] BrowserParent getNextRemoteBrowser (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetNextRemoteBrowser: *const ::libc::c_void,

    /* [binaryname(ScriptableOriginAttributes),implicit_jscontext] readonly attribute jsval originAttributes; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetScriptableOriginAttributes: *const ::libc::c_void,

    /* [binaryname(GetOriginAttributes),nostdcall,notxpcom] const_OriginAttributes binaryGetOriginAttributes (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetOriginAttributes: *const ::libc::c_void,

    /* [nostdcall,notxpcom] nsIBrowsingContextReadyCallback browsingContextReadyCallback (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub BrowsingContextReadyCallback: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOpenWindowInfo {

    /// ```text
    /// /** BrowsingContext which requested the creation of this new window */
    /// ```
    ///

    /// `[infallible] readonly attribute BrowsingContext parent;`
    #[inline]
    pub unsafe fn GetParent(&self, aParent: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetParent)(self, aParent)
    }


    /// ```text
    /// /** If `true`, the content document should be created initially-remote */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isRemote;`
    #[inline]
    pub unsafe fn GetIsRemote(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsRemote)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** Should |opener| be set on the newly-created content window? */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean forceNoOpener;`
    #[inline]
    pub unsafe fn GetForceNoOpener(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetForceNoOpener)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** Whether this is a window opened for printing */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isForPrinting;`
    #[inline]
    pub unsafe fn GetIsForPrinting(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsForPrinting)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Whether this is a window opened for window.print().
    ///    * When this is true, isForPrinting is necessarily true as well.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute boolean isForWindowDotPrint;`
    #[inline]
    pub unsafe fn GetIsForWindowDotPrint(&self) -> bool {
        let mut result = <bool as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetIsForWindowDotPrint)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /** BrowserParent instance to use in the new window */
    /// ```
    ///

    /// `[nostdcall,notxpcom] BrowserParent getNextRemoteBrowser ();`
    const _GetNextRemoteBrowser: () = ();

    /// ```text
    /// /** Origin Attributes for the to-be-created toplevel BrowsingContext */
    /// ```
    ///

    /// `[binaryname(ScriptableOriginAttributes),implicit_jscontext] readonly attribute jsval originAttributes;`
    const _GetScriptableOriginAttributes: () = ();


    /// `[binaryname(GetOriginAttributes),nostdcall,notxpcom] const_OriginAttributes binaryGetOriginAttributes ();`
    const _GetOriginAttributes: () = ();


    /// `[nostdcall,notxpcom] nsIBrowsingContextReadyCallback browsingContextReadyCallback ();`
    const _BrowsingContextReadyCallback: () = ();

}


