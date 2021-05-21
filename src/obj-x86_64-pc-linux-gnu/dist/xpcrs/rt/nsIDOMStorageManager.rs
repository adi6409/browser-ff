//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/storage/nsIDOMStorageManager.idl
//


/// `interface nsIDOMStorageManager : nsISupports`
///

/// ```text
/// /**
///  * General purpose interface that has two implementations, for localStorage
///  * with "@mozilla.org/dom/localStorage-manager;1".
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMStorageManager {
    vtable: *const nsIDOMStorageManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMStorageManager.
unsafe impl XpCom for nsIDOMStorageManager {
    const IID: nsIID = nsID(0xa20c742e, 0x3ed1, 0x44fb,
        [0xb8, 0x97, 0x40, 0x80, 0xa7, 0x5b, 0x16, 0x62]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMStorageManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMStorageManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMStorageManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMStorageManager`.
    fn coerce_from(v: &nsIDOMStorageManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMStorageManagerCoerce for nsIDOMStorageManager {
    #[inline]
    fn coerce_from(v: &nsIDOMStorageManager) -> &Self {
        v
    }
}

impl nsIDOMStorageManager {
    /// Cast this `nsIDOMStorageManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMStorageManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMStorageManager {
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
impl<T: nsISupportsCoerce> nsIDOMStorageManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMStorageManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMStorageManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMStorageManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Storage precacheStorage (in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal); */
    pub PrecacheStorage: unsafe extern "system" fn (this: *const nsIDOMStorageManager, aPrincipal: *const nsIPrincipal, aStoragePrincipal: *const nsIPrincipal, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Storage createStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal, in AString aDocumentURI, [optional] in bool aPrivate); */
    pub CreateStorage: unsafe extern "system" fn (this: *const nsIDOMStorageManager, aWindow: *const mozIDOMWindow, aPrincipal: *const nsIPrincipal, aStoragePrincipal: *const nsIPrincipal, aDocumentURI: *const ::nsstring::nsAString, aPrivate: bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Storage getStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal, [optional] in bool aPrivate); */
    pub GetStorage: unsafe extern "system" fn (this: *const nsIDOMStorageManager, aWindow: *const mozIDOMWindow, aPrincipal: *const nsIPrincipal, aStoragePrincipal: *const nsIPrincipal, aPrivate: bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void cloneStorage (in Storage aStorageToCloneFrom); */
    pub CloneStorage: unsafe extern "system" fn (this: *const nsIDOMStorageManager, aStorageToCloneFrom: *const libc::c_void) -> ::nserror::nsresult,

    /* bool checkStorage (in nsIPrincipal aPrincipal, in Storage aStorage); */
    pub CheckStorage: unsafe extern "system" fn (this: *const nsIDOMStorageManager, aPrincipal: *const nsIPrincipal, aStorage: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMStorageManager {

    /// ```text
    /// /**
    ///    * This starts async preloading of a storage cache for scope
    ///    * defined by the principal and storage principal.
    ///    *
    ///    * Because of how multi-e10s support was implemented in bug 1285898, the
    ///    * StorageCache instance can no longer use a timer to keep itself alive.  So a
    ///    * Storage instance is returned if precaching believes the storage principal may
    ///    * have localStorage data.  (Previously the StorageCache would be brought into
        ///    * existence and kept alive by the timer so that it could be returned if a
        ///    * call to createStorage was made due to a request by the page.)
    ///    */
    /// ```
    ///

    /// `Storage precacheStorage (in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal);`
    #[inline]
    pub unsafe fn PrecacheStorage(&self, aPrincipal: *const nsIPrincipal, aStoragePrincipal: *const nsIPrincipal, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).PrecacheStorage)(self, aPrincipal, aStoragePrincipal, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns instance of DOM storage object for given principal.
    ///    * A new object is always returned and it is ensured there is
    ///    * a storage for the scope created.
    ///    *
    ///    * @param aWindow
    ///    *    The parent window.
    ///    * @param aPrincipal
    ///    *    Principal to bound storage to.
    ///    * @param aStoragePrincipal
    ///    *    StoragePrincipal to bound storage to.
    ///    * @param aDocumentURI
    ///    *    URL of the demanding document, used for DOM storage event only.
    ///    * @param aPrivate
    ///    *    Whether the demanding document is running in Private Browsing mode or not.
    ///    */
    /// ```
    ///

    /// `Storage createStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal, in AString aDocumentURI, [optional] in bool aPrivate);`
    #[inline]
    pub unsafe fn CreateStorage(&self, aWindow: *const mozIDOMWindow, aPrincipal: *const nsIPrincipal, aStoragePrincipal: *const nsIPrincipal, aDocumentURI: *const ::nsstring::nsAString, aPrivate: bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CreateStorage)(self, aWindow, aPrincipal, aStoragePrincipal, aDocumentURI, aPrivate, _retval)
    }


    /// ```text
    /// /**
    ///    * DEPRECATED.  The only good reason to use this was if you were writing a
    ///    * test and wanted to hackily determine if a preload happened.  That's now
    ///    * covered by `nsILocalStorageManager.isPreloaded` and you should use that if
    ///    * that's what you want.  If LSNG is in use, this will throw.
    ///    *
    ///    * Returns instance of DOM storage object for given principal.
    ///    * If there is no storage managed for the scope, then null is returned and
    ///    * no object is created.  Otherwise, an object (new) for the existing storage
    ///    * scope is returned.
    ///    *
    ///    * @param aWindow
    ///    *    The parent window.
    ///    * @param aPrincipal
    ///    *    Principal to bound storage to.
    ///    * @param aStoragePrincipal
    ///    *    StoragePrincipal to bound storage to.
    ///    * @param aPrivate
    ///    *    Whether the demanding document is running in Private Browsing mode or not.
    ///    */
    /// ```
    ///

    /// `Storage getStorage (in mozIDOMWindow aWindow, in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal, [optional] in bool aPrivate);`
    #[inline]
    pub unsafe fn GetStorage(&self, aWindow: *const mozIDOMWindow, aPrincipal: *const nsIPrincipal, aStoragePrincipal: *const nsIPrincipal, aPrivate: bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetStorage)(self, aWindow, aPrincipal, aStoragePrincipal, aPrivate, _retval)
    }


    /// ```text
    /// /**
    ///    * Clones given storage into this storage manager.
    ///    *
    ///    * @param aStorageToCloneFrom
    ///    *    The storage to copy all items from into this manager.  Manager will then
    ///    *    return a new and independent object that contains snapshot of data from
    ///    *    the moment this method was called.  Modification to this new object will
    ///    *    not affect the original storage content we cloned from and vice versa.
    ///    */
    /// ```
    ///

    /// `void cloneStorage (in Storage aStorageToCloneFrom);`
    #[inline]
    pub unsafe fn CloneStorage(&self, aStorageToCloneFrom: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).CloneStorage)(self, aStorageToCloneFrom)
    }


    /// ```text
    /// /**
    ///    * Returns true if the storage belongs to the given principal and is managed
    ///    * (i.e. has been created and is cached) by this storage manager.
    ///    *
    ///    * @param aPrincipal
    ///    *    Principal to check the storage against.
    ///    * @param aStorage
    ///    *    The storage object to examine.
    ///    *
    ///    * @result
    ///    *    true when the storage object is bound with the principal and is managed
    ///    *         by this storage manager.
    ///    *    false otherwise
    ///    */
    /// ```
    ///

    /// `bool checkStorage (in nsIPrincipal aPrincipal, in Storage aStorage);`
    #[inline]
    pub unsafe fn CheckStorage(&self, aPrincipal: *const nsIPrincipal, aStorage: *const libc::c_void, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckStorage)(self, aPrincipal, aStorage, _retval)
    }


}


/// `interface nsIDOMSessionStorageManager : nsIDOMStorageManager`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDOMSessionStorageManager {
    vtable: *const nsIDOMSessionStorageManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDOMSessionStorageManager.
unsafe impl XpCom for nsIDOMSessionStorageManager {
    const IID: nsIID = nsID(0xb3bfbbd0, 0xe738, 0x4cbf,
        [0xb0, 0xf0, 0xb6, 0x5f, 0x25, 0x26, 0x5e, 0x82]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDOMSessionStorageManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDOMSessionStorageManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDOMSessionStorageManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIDOMSessionStorageManager`.
    fn coerce_from(v: &nsIDOMSessionStorageManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDOMSessionStorageManagerCoerce for nsIDOMSessionStorageManager {
    #[inline]
    fn coerce_from(v: &nsIDOMSessionStorageManager) -> &Self {
        v
    }
}

impl nsIDOMSessionStorageManager {
    /// Cast this `nsIDOMSessionStorageManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDOMSessionStorageManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDOMSessionStorageManager {
    type Target = nsIDOMStorageManager;
    #[inline]
    fn deref(&self) -> &nsIDOMStorageManager {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIDOMStorageManagerCoerce> nsIDOMSessionStorageManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDOMSessionStorageManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDOMSessionStorageManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDOMSessionStorageManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIDOMStorageManagerVTable,

    /* [noscript] SessionStorageCacheAddRefed getSessionStorageCache (in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetSessionStorageCache: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDOMSessionStorageManager {

    /// ```text
    /// /**
    ///    * Returns a SessionStorageCache object for the principal scope.
    ///    *
    ///    * @param aPrincipal
    ///    *    Principal to bound storage to.
    ///    * @param aStoragePrincipal
    ///    *    StoragePrincipal to bound storage to.
    ///    */
    /// ```
    ///

    /// `[noscript] SessionStorageCacheAddRefed getSessionStorageCache (in nsIPrincipal aPrincipal, in nsIPrincipal aStoragePrincipal);`
    const _GetSessionStorageCache: () = ();

}


