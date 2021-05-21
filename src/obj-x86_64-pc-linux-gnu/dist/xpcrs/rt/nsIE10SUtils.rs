//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/modules/nsIE10SUtils.idl
//


/// `interface nsIE10SUtils : nsISupports`
///

/// ```text
/// /**
///  * C++ exposed interface for the `E10SUtils` object from the
///  * `resource://gre/modules/E10SUtils.jsm` module.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIE10SUtils {
    vtable: *const nsIE10SUtilsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIE10SUtils.
unsafe impl XpCom for nsIE10SUtils {
    const IID: nsIID = nsID(0x1e18680e, 0x052d, 0x4509,
        [0xa1, 0x7e, 0x67, 0x8f, 0x5c, 0x49, 0x5e, 0x02]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIE10SUtils {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIE10SUtils.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIE10SUtilsCoerce {
    /// Cheaply cast a value of this type from a `nsIE10SUtils`.
    fn coerce_from(v: &nsIE10SUtils) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIE10SUtilsCoerce for nsIE10SUtils {
    #[inline]
    fn coerce_from(v: &nsIE10SUtils) -> &Self {
        v
    }
}

impl nsIE10SUtils {
    /// Cast this `nsIE10SUtils` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIE10SUtilsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIE10SUtils {
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
impl<T: nsISupportsCoerce> nsIE10SUtilsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIE10SUtils) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIE10SUtils
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIE10SUtilsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AUTF8String getRemoteTypeForPrincipal (in nsIPrincipal aPrincipal, in nsIURI aChannelOriginalURI, in boolean aMultiProcess, in boolean aRemoteSubframes, in AUTF8String aPreferredRemoteType, in nsIPrincipal aCurrentPrincipal, in boolean aIsSubframe); */
    pub GetRemoteTypeForPrincipal: unsafe extern "system" fn (this: *const nsIE10SUtils, aPrincipal: *const nsIPrincipal, aChannelOriginalURI: *const nsIURI, aMultiProcess: bool, aRemoteSubframes: bool, aPreferredRemoteType: *const ::nsstring::nsACString, aCurrentPrincipal: *const nsIPrincipal, aIsSubframe: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* AUTF8String getRemoteTypeForWorkerPrincipal (in nsIPrincipal aPrincipal, in nsIE10SUtils_RemoteWorkerType aWorkerType, in boolean aIsMultiProcess, in boolean aIsFission, in AUTF8String aPreferredRemoteType); */
    pub GetRemoteTypeForWorkerPrincipal: unsafe extern "system" fn (this: *const nsIE10SUtils, aPrincipal: *const nsIPrincipal, aWorkerType:  u8, aIsMultiProcess: bool, aIsFission: bool, aPreferredRemoteType: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIE10SUtils {

    /// ```text
    /// /**
    ///    * Determine what remote type should be used to load a document with the given
    ///    * principal.
    ///    *
    ///    * @param aPrincipal  The result principal for the document being loaded.
    ///    * @param aChannelOriginalURI. The original URI being loaded
    ///    *                             (which isn't always the same as the Principal's
        ///    *                              URI)
    ///    * @param aMultiProcess  Does the browser have remote tabs enabled.
    ///    * @param aRemoteSubframes  Does the browser have remote subframes enabled.
    ///    * @param aPreferredRemoteType  If multiple remote types are compatible with
    ///    *                              the load, prefer staying in this remote type.
    ///    * @param aCurrentPrincipal  The principal of the currently loaded document.
    ///    * @param aIsSubframe  Is the process switch occuring in a subframe.
    ///    *
    ///    * @return  The remote type to complete this load in.
    ///    */
    /// ```
    ///

    /// `AUTF8String getRemoteTypeForPrincipal (in nsIPrincipal aPrincipal, in nsIURI aChannelOriginalURI, in boolean aMultiProcess, in boolean aRemoteSubframes, in AUTF8String aPreferredRemoteType, in nsIPrincipal aCurrentPrincipal, in boolean aIsSubframe);`
    #[inline]
    pub unsafe fn GetRemoteTypeForPrincipal(&self, aPrincipal: *const nsIPrincipal, aChannelOriginalURI: *const nsIURI, aMultiProcess: bool, aRemoteSubframes: bool, aPreferredRemoteType: *const ::nsstring::nsACString, aCurrentPrincipal: *const nsIPrincipal, aIsSubframe: bool, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRemoteTypeForPrincipal)(self, aPrincipal, aChannelOriginalURI, aMultiProcess, aRemoteSubframes, aPreferredRemoteType, aCurrentPrincipal, aIsSubframe, _retval)
    }


    /// ```text
    /// /**
    ///    * Determine what remote type should be used to launch a worker script with
    ///    * the given principal.
    ///    *
    ///    * @param aPrincipal
    ///    *   The result principal for the document being loaded.
    ///    * @param aWorkerTypeName
    ///    *   The type of remote worker being launched (Ci.nsIE10SUtils.REMOTE_WORKER_TYPE_*).
    ///    * @param aIsMultiProcess
    ///    *   A boolean to indicate if e10s enabled.
    ///    * @param aIsFission
    ///    *   A boolean to indicate if fission is enabled.
    ///    * @param aPreferredRemoteType
    ///    *   If multiple remote types are compatible with the worker,
    ///    *   prefer staying in this remote type.
    ///    *
    ///    * @return  The remote type to launch the worker in.
    ///    */
    /// ```
    ///

    /// `AUTF8String getRemoteTypeForWorkerPrincipal (in nsIPrincipal aPrincipal, in nsIE10SUtils_RemoteWorkerType aWorkerType, in boolean aIsMultiProcess, in boolean aIsFission, in AUTF8String aPreferredRemoteType);`
    #[inline]
    pub unsafe fn GetRemoteTypeForWorkerPrincipal(&self, aPrincipal: *const nsIPrincipal, aWorkerType:  u8, aIsMultiProcess: bool, aIsFission: bool, aPreferredRemoteType: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRemoteTypeForWorkerPrincipal)(self, aPrincipal, aWorkerType, aIsMultiProcess, aIsFission, aPreferredRemoteType, _retval)
    }


}


