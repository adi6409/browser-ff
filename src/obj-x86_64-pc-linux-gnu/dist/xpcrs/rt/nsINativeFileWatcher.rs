//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/filewatcher/nsINativeFileWatcher.idl
//


/// `interface nsINativeFileWatcherErrorCallback : nsISupports`
///

/// ```text
/// /**
///  * The interface for the callback invoked when there is an error.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeFileWatcherErrorCallback {
    vtable: *const nsINativeFileWatcherErrorCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeFileWatcherErrorCallback.
unsafe impl XpCom for nsINativeFileWatcherErrorCallback {
    const IID: nsIID = nsID(0x5daeddc3, 0xfc94, 0x4880,
        [0x8a, 0x4f, 0x26, 0xd9, 0x10, 0xb9, 0x26, 0x62]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeFileWatcherErrorCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeFileWatcherErrorCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeFileWatcherErrorCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsINativeFileWatcherErrorCallback`.
    fn coerce_from(v: &nsINativeFileWatcherErrorCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeFileWatcherErrorCallbackCoerce for nsINativeFileWatcherErrorCallback {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherErrorCallback) -> &Self {
        v
    }
}

impl nsINativeFileWatcherErrorCallback {
    /// Cast this `nsINativeFileWatcherErrorCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeFileWatcherErrorCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeFileWatcherErrorCallback {
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
impl<T: nsISupportsCoerce> nsINativeFileWatcherErrorCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherErrorCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeFileWatcherErrorCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeFileWatcherErrorCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void complete (in nsresult xpcomError, in long osError); */
    pub Complete: unsafe extern "system" fn (this: *const nsINativeFileWatcherErrorCallback, xpcomError: ::nserror::nsresult, osError: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeFileWatcherErrorCallback {

    /// ```text
    /// /**
    ///    * @param xpcomError The XPCOM error code.
    ///    * @param osError The native OS error (errno under Unix, GetLastError under Windows).
    ///    */
    /// ```
    ///

    /// `void complete (in nsresult xpcomError, in long osError);`
    #[inline]
    pub unsafe fn Complete(&self, xpcomError: ::nserror::nsresult, osError: i32) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, xpcomError, osError)
    }


}


/// `interface nsINativeFileWatcherCallback : nsISupports`
///

/// ```text
/// /**
///  * The interface for the callback invoked when a change on a watched
///  * resource is detected.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeFileWatcherCallback {
    vtable: *const nsINativeFileWatcherCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeFileWatcherCallback.
unsafe impl XpCom for nsINativeFileWatcherCallback {
    const IID: nsIID = nsID(0xfe4d86c9, 0x243f, 0x4195,
        [0xb5, 0x44, 0xae, 0xce, 0x3d, 0xf4, 0xb8, 0x6a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeFileWatcherCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeFileWatcherCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeFileWatcherCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsINativeFileWatcherCallback`.
    fn coerce_from(v: &nsINativeFileWatcherCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeFileWatcherCallbackCoerce for nsINativeFileWatcherCallback {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherCallback) -> &Self {
        v
    }
}

impl nsINativeFileWatcherCallback {
    /// Cast this `nsINativeFileWatcherCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeFileWatcherCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeFileWatcherCallback {
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
impl<T: nsISupportsCoerce> nsINativeFileWatcherCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeFileWatcherCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeFileWatcherCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void changed (in AString resourcePath, in int32_t flags); */
    pub Changed: unsafe extern "system" fn (this: *const nsINativeFileWatcherCallback, resourcePath: *const ::nsstring::nsAString, flags: int32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeFileWatcherCallback {

    /// ```text
    /// /**
    ///    * @param resourcePath
    ///    *        The path of the changed resource. If there were too many changes,
    ///    *        the string "*" is passed.
    ///    * @param flags Reserved for future uses, not currently used.
    ///    */
    /// ```
    ///

    /// `void changed (in AString resourcePath, in int32_t flags);`
    #[inline]
    pub unsafe fn Changed(&self, resourcePath: *const ::nsstring::nsAString, flags: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).Changed)(self, resourcePath, flags)
    }


}


/// `interface nsINativeFileWatcherSuccessCallback : nsISupports`
///

/// ```text
/// /**
///  * The interface for the callback invoked when a file watcher operation
///  * successfully completes.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeFileWatcherSuccessCallback {
    vtable: *const nsINativeFileWatcherSuccessCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeFileWatcherSuccessCallback.
unsafe impl XpCom for nsINativeFileWatcherSuccessCallback {
    const IID: nsIID = nsID(0xc3d7f542, 0x681b, 0x4abd,
        [0x9d, 0x65, 0x9d, 0x79, 0x9b, 0x29, 0xa4, 0x2b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeFileWatcherSuccessCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeFileWatcherSuccessCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeFileWatcherSuccessCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsINativeFileWatcherSuccessCallback`.
    fn coerce_from(v: &nsINativeFileWatcherSuccessCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeFileWatcherSuccessCallbackCoerce for nsINativeFileWatcherSuccessCallback {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherSuccessCallback) -> &Self {
        v
    }
}

impl nsINativeFileWatcherSuccessCallback {
    /// Cast this `nsINativeFileWatcherSuccessCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeFileWatcherSuccessCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeFileWatcherSuccessCallback {
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
impl<T: nsISupportsCoerce> nsINativeFileWatcherSuccessCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherSuccessCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeFileWatcherSuccessCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeFileWatcherSuccessCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void complete (in AString resourcePath); */
    pub Complete: unsafe extern "system" fn (this: *const nsINativeFileWatcherSuccessCallback, resourcePath: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeFileWatcherSuccessCallback {

    /// ```text
    /// /**
    ///    * @param resourcePath
    ///    *        The path of the resource for which the operation completes.
    ///    */
    /// ```
    ///

    /// `void complete (in AString resourcePath);`
    #[inline]
    pub unsafe fn Complete(&self, resourcePath: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Complete)(self, resourcePath)
    }


}


/// `interface nsINativeFileWatcherService : nsISupports`
///

/// ```text
/// /**
///  * A service providing native implementations of path changes notification.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINativeFileWatcherService {
    vtable: *const nsINativeFileWatcherServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINativeFileWatcherService.
unsafe impl XpCom for nsINativeFileWatcherService {
    const IID: nsIID = nsID(0xb3a4e8d8, 0x7dc8, 0x47db,
        [0xa8, 0xb4, 0x83, 0x73, 0x6d, 0x7a, 0xc1, 0xaa]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINativeFileWatcherService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINativeFileWatcherService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINativeFileWatcherServiceCoerce {
    /// Cheaply cast a value of this type from a `nsINativeFileWatcherService`.
    fn coerce_from(v: &nsINativeFileWatcherService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINativeFileWatcherServiceCoerce for nsINativeFileWatcherService {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherService) -> &Self {
        v
    }
}

impl nsINativeFileWatcherService {
    /// Cast this `nsINativeFileWatcherService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINativeFileWatcherServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINativeFileWatcherService {
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
impl<T: nsISupportsCoerce> nsINativeFileWatcherServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINativeFileWatcherService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINativeFileWatcherService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINativeFileWatcherServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addPath (in AString pathToWatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
    pub AddPath: unsafe extern "system" fn (this: *const nsINativeFileWatcherService, pathToWatch: *const ::nsstring::nsAString, onChange: *const nsINativeFileWatcherCallback, onError: *const nsINativeFileWatcherErrorCallback, onSuccess: *const nsINativeFileWatcherSuccessCallback) -> ::nserror::nsresult,

    /* void removePath (in AString pathToUnwatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess); */
    pub RemovePath: unsafe extern "system" fn (this: *const nsINativeFileWatcherService, pathToUnwatch: *const ::nsstring::nsAString, onChange: *const nsINativeFileWatcherCallback, onError: *const nsINativeFileWatcherErrorCallback, onSuccess: *const nsINativeFileWatcherSuccessCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINativeFileWatcherService {

    /// ```text
    /// /**
    ///    * Watches the passed path for changes. If it's a directory, every file
    ///    * it contains is watched. Recursively watches subdirectories. If the
    ///    * resource is already being watched, does nothing. If the passed path
    ///    * is a file, the behaviour is not specified.
    ///    *
    ///    * @param pathToWatch The path to watch for changes.
    ///    * @param onChange
    ///    *        The callback invoked whenever a change on a watched
    ///    *        resource is detected.
    ///    * @param onError
    ///    *        The optional callback invoked whenever an error occurs.
    ///    * @param onSuccess
    ///    *        The optional callback invoked when the file watcher starts
    ///    *        watching the resource for changes.
    ///    */
    /// ```
    ///

    /// `void addPath (in AString pathToWatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess);`
    #[inline]
    pub unsafe fn AddPath(&self, pathToWatch: *const ::nsstring::nsAString, onChange: *const nsINativeFileWatcherCallback, onError: *const nsINativeFileWatcherErrorCallback, onSuccess: *const nsINativeFileWatcherSuccessCallback) -> ::nserror::nsresult {
        ((*self.vtable).AddPath)(self, pathToWatch, onChange, onError, onSuccess)
    }


    /// ```text
    /// /**
    ///    * Removes the provided path from the watched resources. If the path
    ///    * was not being watched or the callbacks were not registered, silently
    ///    * ignores the request.
    ///    * Please note that the file watcher only considers the onChange callbacks
    ///    * when deciding to close a watch on a resource. If there are no more onChange
    ///    * callbacks associated to the watch, it gets closed (even though it still has
        ///    * some error callbacks associated).
    ///    *
    ///    * @param pathToUnwatch The path to un-watch.
    ///    * @param onChange
    ///    *        The registered callback invoked whenever a change on a watched
    ///    *        resource is detected.
    ///    * @param onError
    ///    *        The optionally registered callback invoked whenever an error
    ///    *        occurs.
    ///    * @param onSuccess
    ///    *        The optional callback invoked when the file watcher stops
    ///    *        watching the resource for changes.
    ///    */
    /// ```
    ///

    /// `void removePath (in AString pathToUnwatch, in nsINativeFileWatcherCallback onChange, [optional] in nsINativeFileWatcherErrorCallback onError, [optional] in nsINativeFileWatcherSuccessCallback onSuccess);`
    #[inline]
    pub unsafe fn RemovePath(&self, pathToUnwatch: *const ::nsstring::nsAString, onChange: *const nsINativeFileWatcherCallback, onError: *const nsINativeFileWatcherErrorCallback, onSuccess: *const nsINativeFileWatcherSuccessCallback) -> ::nserror::nsresult {
        ((*self.vtable).RemovePath)(self, pathToUnwatch, onChange, onError, onSuccess)
    }


}


