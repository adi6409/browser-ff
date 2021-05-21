//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/osfile/nsINativeOSFileInternals.idl
//


/// `interface nsINativeOSFileResult : nsISupports`
///

/// ```text
/// /**
///  * The result of a successful asynchronous operation.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeOSFileResult {
    vtable: *const nsINativeOSFileResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeOSFileResult.
unsafe impl XpCom for nsINativeOSFileResult {
    const IID: nsIID = nsID(0x08b4cf29, 0x3d65, 0x4e79,
        [0xb5, 0x22, 0xa6, 0x94, 0xc3, 0x22, 0xed, 0x07]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeOSFileResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeOSFileResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeOSFileResultCoerce {
    /// Cheaply cast a value of this type from a `nsINativeOSFileResult`.
    fn coerce_from(v: &nsINativeOSFileResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeOSFileResultCoerce for nsINativeOSFileResult {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileResult) -> &Self {
        v
    }
}

impl nsINativeOSFileResult {
    /// Cast this `nsINativeOSFileResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeOSFileResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeOSFileResult {
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
impl<T: nsISupportsCoerce> nsINativeOSFileResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeOSFileResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeOSFileResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] readonly attribute jsval result; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetResult: *const ::libc::c_void,

    /* readonly attribute double dispatchDurationMS; */
    pub GetDispatchDurationMS: unsafe extern "system" fn (this: *const nsINativeOSFileResult, aDispatchDurationMS: *mut libc::c_double) -> ::nserror::nsresult,

    /* readonly attribute double executionDurationMS; */
    pub GetExecutionDurationMS: unsafe extern "system" fn (this: *const nsINativeOSFileResult, aExecutionDurationMS: *mut libc::c_double) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeOSFileResult {

    /// ```text
    /// /**
    ///    * The actual value produced by the operation.
    ///    *
    ///    * Actual type of this value depends on the options passed to the
    ///    * operation.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval result;`
    const _GetResult: () = ();

    /// ```text
    /// /**
    ///    * Delay between when the operation was requested on the main thread and
    ///    * when the operation was started off main thread.
    ///    */
    /// ```
    ///

    /// `readonly attribute double dispatchDurationMS;`
    #[inline]
    pub unsafe fn GetDispatchDurationMS(&self, aDispatchDurationMS: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetDispatchDurationMS)(self, aDispatchDurationMS)
    }


    /// ```text
    /// /**
    ///    * Duration of the off main thread execution.
    ///    */
    /// ```
    ///

    /// `readonly attribute double executionDurationMS;`
    #[inline]
    pub unsafe fn GetExecutionDurationMS(&self, aExecutionDurationMS: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetExecutionDurationMS)(self, aExecutionDurationMS)
    }


}


/// `interface nsINativeOSFileSuccessCallback : nsISupports`
///

/// ```text
/// /**
///  * A callback invoked in case of success.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeOSFileSuccessCallback {
    vtable: *const nsINativeOSFileSuccessCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeOSFileSuccessCallback.
unsafe impl XpCom for nsINativeOSFileSuccessCallback {
    const IID: nsIID = nsID(0x2c1922ca, 0xca1b, 0x4099,
        [0x8b, 0x61, 0xec, 0x23, 0xcf, 0xf4, 0x94, 0x12]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeOSFileSuccessCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeOSFileSuccessCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeOSFileSuccessCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsINativeOSFileSuccessCallback`.
    fn coerce_from(v: &nsINativeOSFileSuccessCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeOSFileSuccessCallbackCoerce for nsINativeOSFileSuccessCallback {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileSuccessCallback) -> &Self {
        v
    }
}

impl nsINativeOSFileSuccessCallback {
    /// Cast this `nsINativeOSFileSuccessCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeOSFileSuccessCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeOSFileSuccessCallback {
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
impl<T: nsISupportsCoerce> nsINativeOSFileSuccessCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileSuccessCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeOSFileSuccessCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeOSFileSuccessCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void complete (in nsINativeOSFileResult result); */
    pub Complete: unsafe extern "system" fn (this: *const nsINativeOSFileSuccessCallback, result: *const nsINativeOSFileResult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeOSFileSuccessCallback {


    /// `void complete (in nsINativeOSFileResult result);`
    #[inline]
    pub unsafe fn Complete(&self, result: *const nsINativeOSFileResult) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, result)
    }


}


/// `interface nsINativeOSFileErrorCallback : nsISupports`
///

/// ```text
/// /**
///  * A callback invoked in case of error.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeOSFileErrorCallback {
    vtable: *const nsINativeOSFileErrorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeOSFileErrorCallback.
unsafe impl XpCom for nsINativeOSFileErrorCallback {
    const IID: nsIID = nsID(0xf612e0fc, 0x6736, 0x4d24,
        [0xaa, 0x50, 0xfd, 0x66, 0x1b, 0x3b, 0x40, 0xb6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeOSFileErrorCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeOSFileErrorCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeOSFileErrorCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsINativeOSFileErrorCallback`.
    fn coerce_from(v: &nsINativeOSFileErrorCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeOSFileErrorCallbackCoerce for nsINativeOSFileErrorCallback {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileErrorCallback) -> &Self {
        v
    }
}

impl nsINativeOSFileErrorCallback {
    /// Cast this `nsINativeOSFileErrorCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeOSFileErrorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeOSFileErrorCallback {
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
impl<T: nsISupportsCoerce> nsINativeOSFileErrorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileErrorCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeOSFileErrorCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeOSFileErrorCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void complete (in ACString operation, in long OSstatus); */
    pub Complete: unsafe extern "system" fn (this: *const nsINativeOSFileErrorCallback, operation: *const ::nsstring::nsACString, OSstatus: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeOSFileErrorCallback {

    /// ```text
    /// /**
    ///    * @param operation The name of the failed operation. Provided to aid
    ///    * debugging only, may change without notice.
    ///    * @param OSstatus The OS status of the operation (errno under Unix,
        ///    * GetLastError under Windows).
    ///    */
    /// ```
    ///

    /// `void complete (in ACString operation, in long OSstatus);`
    #[inline]
    pub unsafe fn Complete(&self, operation: *const ::nsstring::nsACString, OSstatus: i32) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, operation, OSstatus)
    }


}


/// `interface nsINativeOSFileInternalsService : nsISupports`
///

/// ```text
/// /**
///  * A service providing native implementations of some of the features
///  * of OS.File.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeOSFileInternalsService {
    vtable: *const nsINativeOSFileInternalsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeOSFileInternalsService.
unsafe impl XpCom for nsINativeOSFileInternalsService {
    const IID: nsIID = nsID(0x913362ad, 0x1526, 0x4623,
        [0x9e, 0x6b, 0xa2, 0xeb, 0x08, 0xaf, 0xbb, 0xb9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeOSFileInternalsService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeOSFileInternalsService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeOSFileInternalsServiceCoerce {
    /// Cheaply cast a value of this type from a `nsINativeOSFileInternalsService`.
    fn coerce_from(v: &nsINativeOSFileInternalsService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeOSFileInternalsServiceCoerce for nsINativeOSFileInternalsService {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileInternalsService) -> &Self {
        v
    }
}

impl nsINativeOSFileInternalsService {
    /// Cast this `nsINativeOSFileInternalsService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeOSFileInternalsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeOSFileInternalsService {
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
impl<T: nsISupportsCoerce> nsINativeOSFileInternalsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeOSFileInternalsService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeOSFileInternalsService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeOSFileInternalsServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void read (in AString path, in jsval options, in nsINativeOSFileSuccessCallback onSuccess, in nsINativeOSFileErrorCallback onError); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Read: *const ::libc::c_void,

    /* [implicit_jscontext] void writeAtomic (in AString path, in jsval buffer, in jsval options, in nsINativeOSFileSuccessCallback onSuccess, in nsINativeOSFileErrorCallback onError); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub WriteAtomic: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeOSFileInternalsService {

    /// ```text
    /// /**
    ///    * Implementation of OS.File.read
    ///    *
    ///    * @param path The absolute path to the file to read.
    ///    * @param options An object that may contain some of the following fields
    ///    * - {number} bytes The maximal number of bytes to read.
    ///    * - {string} encoding If provided, return the result as a string, decoded
    ///    *   using this encoding. Otherwise, pass the result as an ArrayBuffer.
    ///    *   Invalid encodings cause onError to be called with the platform-specific
    ///    *   "invalid argument" constant.
    ///    * - {string} compression Unimplemented at the moment.
    ///    * @param onSuccess The success callback.
    ///    * @param onError The error callback.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void read (in AString path, in jsval options, in nsINativeOSFileSuccessCallback onSuccess, in nsINativeOSFileErrorCallback onError);`
    const _Read: () = ();

    /// ```text
    /// /**
    ///    * Implementation of OS.File.writeAtomic
    ///    *
    ///    * @param path the absolute path of the file to write to.
    ///    * @param buffer the data as an array buffer to be written to the file.
    ///    * @param options An object that may contain the following fields
    ///    * - {number} bytes If provided, the number of bytes written is equal to this.
    ///    *   The default value is the size of the |buffer|.
    ///    * - {string} tmpPath If provided and not null, first write to this path, and
    ///    *   move to |path| after writing.
    ///    * - {string} backupPath if provided, backup file at |path| to this path
    ///    *   before overwriting it.
    ///    * - {bool} flush if provided and true, flush the contents of the buffer after
    ///    *   writing. This is slower, but safer.
    ///    * - {bool} noOverwrite if provided and true, do not write if a file already
    ///    *   exists at |path|.
    ///    * @param onSuccess The success callback.
    ///    * @param onError The error callback.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void writeAtomic (in AString path, in jsval buffer, in jsval options, in nsINativeOSFileSuccessCallback onSuccess, in nsINativeOSFileErrorCallback onError);`
    const _WriteAtomic: () = ();

}


