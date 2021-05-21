//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIDirectoryService.idl
//


/// `interface nsIDirectoryServiceProvider : nsISupports`
///

/// ```text
/// /**
///  * nsIDirectoryServiceProvider
///  *
///  * Used by Directory Service to get file locations.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDirectoryServiceProvider {
    vtable: *const nsIDirectoryServiceProviderVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDirectoryServiceProvider.
unsafe impl XpCom for nsIDirectoryServiceProvider {
    const IID: nsIID = nsID(0xbbf8cab0, 0xd43a, 0x11d3,
        [0x8c, 0xc2, 0x00, 0x60, 0x97, 0x92, 0x27, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDirectoryServiceProvider {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDirectoryServiceProvider.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDirectoryServiceProviderCoerce {
    /// Cheaply cast a value of this type from a `nsIDirectoryServiceProvider`.
    fn coerce_from(v: &nsIDirectoryServiceProvider) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDirectoryServiceProviderCoerce for nsIDirectoryServiceProvider {
    #[inline]
    fn coerce_from(v: &nsIDirectoryServiceProvider) -> &Self {
        v
    }
}

impl nsIDirectoryServiceProvider {
    /// Cast this `nsIDirectoryServiceProvider` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDirectoryServiceProviderCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDirectoryServiceProvider {
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
impl<T: nsISupportsCoerce> nsIDirectoryServiceProviderCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirectoryServiceProvider) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDirectoryServiceProvider
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDirectoryServiceProviderVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* nsIFile getFile (in string prop, out boolean persistent); */
    pub GetFile: unsafe extern "system" fn (this: *const nsIDirectoryServiceProvider, prop: *const libc::c_char, persistent: *mut bool, _retval: *mut*const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDirectoryServiceProvider {

    /// ```text
    /// /**
    ///   * getFile
    ///   *
    ///   * Directory Service calls this when it gets the first request for
    ///   * a prop or on every request if the prop is not persistent.
    ///   *
    ///   * @param prop         The symbolic name of the file.
    ///   * @param persistent   TRUE - The returned file will be cached by Directory
    ///   *                     Service. Subsequent requests for this prop will
    ///   *                     bypass the provider and use the cache.
    ///   *                     FALSE - The provider will be asked for this prop
    ///   *                     each time it is requested.
    ///   *
    ///   * @return             The file represented by the property.
    ///   *
    ///   */
    /// ```
    ///

    /// `nsIFile getFile (in string prop, out boolean persistent);`
    #[inline]
    pub unsafe fn GetFile(&self, prop: *const libc::c_char, persistent: *mut bool, _retval: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).GetFile)(self, prop, persistent, _retval)
    }


}


/// `interface nsIDirectoryServiceProvider2 : nsIDirectoryServiceProvider`
///

/// ```text
/// /**
///  * nsIDirectoryServiceProvider2
///  *
///  * An extension of nsIDirectoryServiceProvider which allows
///  * multiple files to be returned for the given key.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDirectoryServiceProvider2 {
    vtable: *const nsIDirectoryServiceProvider2VTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDirectoryServiceProvider2.
unsafe impl XpCom for nsIDirectoryServiceProvider2 {
    const IID: nsIID = nsID(0x2f977d4b, 0x5485, 0x11d4,
        [0x87, 0xe2, 0x00, 0x10, 0xa4, 0xe7, 0x5e, 0xf2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDirectoryServiceProvider2 {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDirectoryServiceProvider2.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDirectoryServiceProvider2Coerce {
    /// Cheaply cast a value of this type from a `nsIDirectoryServiceProvider2`.
    fn coerce_from(v: &nsIDirectoryServiceProvider2) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDirectoryServiceProvider2Coerce for nsIDirectoryServiceProvider2 {
    #[inline]
    fn coerce_from(v: &nsIDirectoryServiceProvider2) -> &Self {
        v
    }
}

impl nsIDirectoryServiceProvider2 {
    /// Cast this `nsIDirectoryServiceProvider2` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDirectoryServiceProvider2Coerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDirectoryServiceProvider2 {
    type Target = nsIDirectoryServiceProvider;
    #[inline]
    fn deref(&self) -> &nsIDirectoryServiceProvider {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDirectoryServiceProviderCoerce> nsIDirectoryServiceProvider2Coerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirectoryServiceProvider2) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDirectoryServiceProvider2
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDirectoryServiceProvider2VTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDirectoryServiceProviderVTable,

    /* nsISimpleEnumerator getFiles (in string prop); */
    pub GetFiles: unsafe extern "system" fn (this: *const nsIDirectoryServiceProvider2, prop: *const libc::c_char, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDirectoryServiceProvider2 {

    /// ```text
    /// /**
    ///   * getFiles
    ///   *
    ///   * Directory Service calls this when it gets a request for
    ///   * a prop and the requested type is nsISimpleEnumerator.
    ///   *
    ///   * @param prop         The symbolic name of the file list.
    ///   *
    ///   * @return             An enumerator for a list of file locations.
    ///   *                     The elements in the enumeration are nsIFile
    ///   * @returnCode         NS_SUCCESS_AGGREGATE_RESULT if this result should be
    ///   *                     aggregated with other "lower" providers.
    ///   */
    /// ```
    ///

    /// `nsISimpleEnumerator getFiles (in string prop);`
    #[inline]
    pub unsafe fn GetFiles(&self, prop: *const libc::c_char, _retval: *mut*const nsISimpleEnumerator) -> ::nserror::nsresult {
        ((*self.vtable).GetFiles)(self, prop, _retval)
    }


}


/// `interface nsIDirectoryService : nsISupports`
///

/// ```text
/// /**
///  * nsIDirectoryService
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDirectoryService {
    vtable: *const nsIDirectoryServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDirectoryService.
unsafe impl XpCom for nsIDirectoryService {
    const IID: nsIID = nsID(0x57a66a60, 0xd43a, 0x11d3,
        [0x8c, 0xc2, 0x00, 0x60, 0x97, 0x92, 0x27, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDirectoryService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDirectoryService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDirectoryServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIDirectoryService`.
    fn coerce_from(v: &nsIDirectoryService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDirectoryServiceCoerce for nsIDirectoryService {
    #[inline]
    fn coerce_from(v: &nsIDirectoryService) -> &Self {
        v
    }
}

impl nsIDirectoryService {
    /// Cast this `nsIDirectoryService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDirectoryServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDirectoryService {
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
impl<T: nsISupportsCoerce> nsIDirectoryServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirectoryService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDirectoryService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDirectoryServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (); */
    pub Init: unsafe extern "system" fn (this: *const nsIDirectoryService) -> ::nserror::nsresult,

    /* void registerProvider (in nsIDirectoryServiceProvider prov); */
    pub RegisterProvider: unsafe extern "system" fn (this: *const nsIDirectoryService, prov: *const nsIDirectoryServiceProvider) -> ::nserror::nsresult,

    /* void unregisterProvider (in nsIDirectoryServiceProvider prov); */
    pub UnregisterProvider: unsafe extern "system" fn (this: *const nsIDirectoryService, prov: *const nsIDirectoryServiceProvider) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDirectoryService {

    /// ```text
    /// /**
    ///   * init
    ///   *
    ///   * Must be called. Used internally by XPCOM initialization.
    ///   *
    ///   */
    /// ```
    ///

    /// `void init ();`
    #[inline]
    pub unsafe fn Init(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, )
    }


    /// ```text
    /// /**
    ///   * registerProvider
    ///   *
    ///   * Register a provider with the service.
    ///   *
    ///   * @param prov            The service will keep a strong reference
    ///   *                        to this object. It will be released when
    ///   *                        the service is released.
    ///   *
    ///   */
    /// ```
    ///

    /// `void registerProvider (in nsIDirectoryServiceProvider prov);`
    #[inline]
    pub unsafe fn RegisterProvider(&self, prov: *const nsIDirectoryServiceProvider) -> ::nserror::nsresult {
        ((*self.vtable).RegisterProvider)(self, prov)
    }


    /// ```text
    /// /**
    ///   * unregisterProvider
    ///   *
    ///   * Unregister a provider with the service.
    ///   *
    ///   * @param prov
    ///   *
    ///   */
    /// ```
    ///

    /// `void unregisterProvider (in nsIDirectoryServiceProvider prov);`
    #[inline]
    pub unsafe fn UnregisterProvider(&self, prov: *const nsIDirectoryServiceProvider) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterProvider)(self, prov)
    }


}


