//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlListManager.idl
//


/// `interface nsIUrlListManager : nsISupports`
///

/// ```text
/// /**
///  * Interface for a class that manages updates of the url classifier database.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlListManager {
    vtable: *const nsIUrlListManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlListManager.
unsafe impl XpCom for nsIUrlListManager {
    const IID: nsIID = nsID(0xd60a08ee, 0x5c83, 0x4eb6,
        [0xbd, 0xfb, 0x79, 0xfd, 0x07, 0x16, 0x50, 0x1e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlListManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlListManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlListManagerCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlListManager`.
    fn coerce_from(v: &nsIUrlListManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlListManagerCoerce for nsIUrlListManager {
    #[inline]
    fn coerce_from(v: &nsIUrlListManager) -> &Self {
        v
    }
}

impl nsIUrlListManager {
    /// Cast this `nsIUrlListManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlListManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlListManager {
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
impl<T: nsISupportsCoerce> nsIUrlListManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlListManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlListManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlListManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString getGethashUrl (in ACString tableName); */
    pub GetGethashUrl: unsafe extern "system" fn (this: *const nsIUrlListManager, tableName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getUpdateUrl (in ACString tableName); */
    pub GetUpdateUrl: unsafe extern "system" fn (this: *const nsIUrlListManager, tableName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean registerTable (in ACString tableName, in ACString providerName, in ACString updateUrl, in ACString gethashUrl); */
    pub RegisterTable: unsafe extern "system" fn (this: *const nsIUrlListManager, tableName: *const ::nsstring::nsACString, providerName: *const ::nsstring::nsACString, updateUrl: *const ::nsstring::nsACString, gethashUrl: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void unregisterTable (in ACString tableName); */
    pub UnregisterTable: unsafe extern "system" fn (this: *const nsIUrlListManager, tableName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void enableUpdate (in ACString tableName); */
    pub EnableUpdate: unsafe extern "system" fn (this: *const nsIUrlListManager, tableName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void disableAllUpdates (); */
    pub DisableAllUpdates: unsafe extern "system" fn (this: *const nsIUrlListManager) -> ::nserror::nsresult,

    /* void disableUpdate (in ACString tableName); */
    pub DisableUpdate: unsafe extern "system" fn (this: *const nsIUrlListManager, tableName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void maybeToggleUpdateChecking (); */
    pub MaybeToggleUpdateChecking: unsafe extern "system" fn (this: *const nsIUrlListManager) -> ::nserror::nsresult,

    /* boolean checkForUpdates (in ACString updateUrl); */
    pub CheckForUpdates: unsafe extern "system" fn (this: *const nsIUrlListManager, updateUrl: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean forceUpdates (in ACString tableNames); */
    pub ForceUpdates: unsafe extern "system" fn (this: *const nsIUrlListManager, tableNames: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* uint64_t getBackOffTime (in ACString provider); */
    pub GetBackOffTime: unsafe extern "system" fn (this: *const nsIUrlListManager, provider: *const ::nsstring::nsACString, _retval: *mut uint64_t) -> ::nserror::nsresult,

    /* boolean isRegistered (); */
    pub IsRegistered: unsafe extern "system" fn (this: *const nsIUrlListManager, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlListManager {

    /// ```text
    /// /**
    ///      * Get the gethash url for this table
    ///      */
    /// ```
    ///

    /// `ACString getGethashUrl (in ACString tableName);`
    #[inline]
    pub unsafe fn GetGethashUrl(&self, tableName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetGethashUrl)(self, tableName, _retval)
    }


    /// ```text
    /// /**
    ///      * Get the update url for this table
    ///      */
    /// ```
    ///

    /// `ACString getUpdateUrl (in ACString tableName);`
    #[inline]
    pub unsafe fn GetUpdateUrl(&self, tableName: *const ::nsstring::nsACString, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUpdateUrl)(self, tableName, _retval)
    }


    /// ```text
    /// /**
    ///      * Add a table to the list of tables we are managing. The name is a
    ///      * string of the format provider_name-semantic_type-table_type.  For
    ///      * @param tableName A string of the format
    ///      *        provider_name-semantic_type-table_type.  For example,
    ///      *        goog-white-enchash or goog-black-url.
    ///      * @param providerName The name of the entity providing the list.
    ///      * @param updateUrl The URL from which to fetch updates.
    ///      * @param gethashUrl The URL from which to fetch hash completions.
    ///      */
    /// ```
    ///

    /// `boolean registerTable (in ACString tableName, in ACString providerName, in ACString updateUrl, in ACString gethashUrl);`
    #[inline]
    pub unsafe fn RegisterTable(&self, tableName: *const ::nsstring::nsACString, providerName: *const ::nsstring::nsACString, updateUrl: *const ::nsstring::nsACString, gethashUrl: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).RegisterTable)(self, tableName, providerName, updateUrl, gethashUrl, _retval)
    }


    /// ```text
    /// /**
    ///      * Unregister table from the list
    ///      */
    /// ```
    ///

    /// `void unregisterTable (in ACString tableName);`
    #[inline]
    pub unsafe fn UnregisterTable(&self, tableName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).UnregisterTable)(self, tableName)
    }


    /// ```text
    /// /**
    ///      * Turn on update checking for a table. I.e., during the next server
    ///      * check, download updates for this table.
    ///      */
    /// ```
    ///

    /// `void enableUpdate (in ACString tableName);`
    #[inline]
    pub unsafe fn EnableUpdate(&self, tableName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).EnableUpdate)(self, tableName)
    }


    /// ```text
    /// /**
    ///      * Turn off update checking for all tables.
    ///      */
    /// ```
    ///

    /// `void disableAllUpdates ();`
    #[inline]
    pub unsafe fn DisableAllUpdates(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DisableAllUpdates)(self, )
    }


    /// ```text
    /// /**
    ///      * Turn off update checking for a single table. Only used in tests.
    ///      */
    /// ```
    ///

    /// `void disableUpdate (in ACString tableName);`
    #[inline]
    pub unsafe fn DisableUpdate(&self, tableName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).DisableUpdate)(self, tableName)
    }


    /// ```text
    /// /**
    ///      * Toggle update checking, if necessary.
    ///      */
    /// ```
    ///

    /// `void maybeToggleUpdateChecking ();`
    #[inline]
    pub unsafe fn MaybeToggleUpdateChecking(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).MaybeToggleUpdateChecking)(self, )
    }


    /// ```text
    /// /**
    ///      * This is currently used by about:url-classifier to force an update
    ///      * for the update url. Update may still fail because of backoff algorithm.
    ///      */
    /// ```
    ///

    /// `boolean checkForUpdates (in ACString updateUrl);`
    #[inline]
    pub unsafe fn CheckForUpdates(&self, updateUrl: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CheckForUpdates)(self, updateUrl, _retval)
    }


    /// ```text
    /// /**
    ///      * Force updates for the given tables, updates are still restricted to
    ///      * backoff algorithm.
    ///      * @param tables  A string lists all the tables that we want to trigger updates.
    ///      *                table names are separated with ','.
    ///      */
    /// ```
    ///

    /// `boolean forceUpdates (in ACString tableNames);`
    #[inline]
    pub unsafe fn ForceUpdates(&self, tableNames: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).ForceUpdates)(self, tableNames, _retval)
    }


    /// ```text
    /// /**
    ///      * This is currently used by about:url-classifier to get back-off time
    ///      * (in millisecond since epoch) for the given provider. Return 0 if we
    ///      * are not in back-off mode.
    ///      */
    /// ```
    ///

    /// `uint64_t getBackOffTime (in ACString provider);`
    #[inline]
    pub unsafe fn GetBackOffTime(&self, provider: *const ::nsstring::nsACString, _retval: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetBackOffTime)(self, provider, _retval)
    }


    /// ```text
    /// /**
    ///      * Return true if someone registers a table, this is used by testcase
    ///      * to figure out it SafeBrowsing.jsm is initialized.
    ///      */
    /// ```
    ///

    /// `boolean isRegistered ();`
    #[inline]
    pub unsafe fn IsRegistered(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsRegistered)(self, _retval)
    }


}


