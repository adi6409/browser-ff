//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/backgroundhangmonitor/nsIHangDetails.idl
//


/// `interface nsIHangDetails : nsISupports`
///

/// ```text
/// /**
///  * A scriptable interface for getting information about a BHR detected hang.
///  * This is the type of the subject of the "bhr-thread-hang" observer topic.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHangDetails {
    vtable: *const nsIHangDetailsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHangDetails.
unsafe impl XpCom for nsIHangDetails {
    const IID: nsIID = nsID(0x23d63fff, 0x38d6, 0x4003,
        [0x9c, 0x57, 0x2c, 0x90, 0xac, 0xa1, 0x18, 0x0a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHangDetails {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHangDetails.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHangDetailsCoerce {
    /// Cheaply cast a value of this type from a `nsIHangDetails`.
    fn coerce_from(v: &nsIHangDetails) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHangDetailsCoerce for nsIHangDetails {
    #[inline]
    fn coerce_from(v: &nsIHangDetails) -> &Self {
        v
    }
}

impl nsIHangDetails {
    /// Cast this `nsIHangDetails` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHangDetailsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHangDetails {
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
impl<T: nsISupportsCoerce> nsIHangDetailsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHangDetails) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHangDetails
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHangDetailsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute bool wasPersisted; */
    pub GetWasPersisted: unsafe extern "system" fn (this: *const nsIHangDetails, aWasPersisted: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute double duration; */
    pub GetDuration: unsafe extern "system" fn (this: *const nsIHangDetails, aDuration: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute ACString thread; */
    pub GetThread: unsafe extern "system" fn (this: *const nsIHangDetails, aThread: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString runnableName; */
    pub GetRunnableName: unsafe extern "system" fn (this: *const nsIHangDetails, aRunnableName: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString process; */
    pub GetProcess: unsafe extern "system" fn (this: *const nsIHangDetails, aProcess: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String remoteType; */
    pub GetRemoteType: unsafe extern "system" fn (this: *const nsIHangDetails, aRemoteType: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval stack; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetStack: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval modules; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetModules: *const ::libc::c_void,

    /* [implicit_jscontext] readonly attribute jsval annotations; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetAnnotations: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHangDetails {

    /// ```text
    /// /**
    ///    * The hang was persisted to disk as a permahang, so we can clear the
    ///    * permahang file once we submit this.
    ///    */
    /// ```
    ///

    /// `readonly attribute bool wasPersisted;`
    #[inline]
    pub unsafe fn GetWasPersisted(&self, aWasPersisted: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetWasPersisted)(self, aWasPersisted)
    }


    /// ```text
    /// /**
    ///    * The detected duration of the hang in milliseconds.
    ///    */
    /// ```
    ///

    /// `readonly attribute double duration;`
    #[inline]
    pub unsafe fn GetDuration(&self, aDuration: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetDuration)(self, aDuration)
    }


    /// ```text
    /// /**
    ///    * The name of the thread which hung.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString thread;`
    #[inline]
    pub unsafe fn GetThread(&self, aThread: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetThread)(self, aThread)
    }


    /// ```text
    /// /**
    ///    * The name of the runnable which hung if it hung on the main thread.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString runnableName;`
    #[inline]
    pub unsafe fn GetRunnableName(&self, aRunnableName: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRunnableName)(self, aRunnableName)
    }


    /// ```text
    /// /**
    ///    * The type of process which produced the hang. This should be either:
    ///    * "default", "content", or "gpu".
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString process;`
    #[inline]
    pub unsafe fn GetProcess(&self, aProcess: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetProcess)(self, aProcess)
    }


    /// ```text
    /// /**
    ///    * The remote type of the content process which produced the hang.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String remoteType;`
    #[inline]
    pub unsafe fn GetRemoteType(&self, aRemoteType: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetRemoteType)(self, aRemoteType)
    }


    /// ```text
    /// /**
    ///    * Returns the stack which was captured by BHR. The offset is encoded as a hex
    ///    * string, as it can contain numbers larger than JS can hold losslessly.
    ///    *
    ///    * This value takes the following form:
    ///    * [ [moduleIndex, offset], ... ]
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval stack;`
    const _GetStack: () = ();

    /// ```text
    /// /**
    ///    * Returns the modules which were captured by BHR.
    ///    *
    ///    * This value takes the following form:
    ///    * [ ["fileName", "BreakpadId"], ... ]
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval modules;`
    const _GetModules: () = ();

    /// ```text
    /// /**
    ///    * The hang annotations which were captured when the hang occured. This
    ///    * attribute is a JS object of key-value pairs.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval annotations;`
    const _GetAnnotations: () = ();

}


