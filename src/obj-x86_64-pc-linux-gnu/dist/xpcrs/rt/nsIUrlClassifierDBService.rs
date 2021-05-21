//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/url-classifier/nsIUrlClassifierDBService.idl
//


/// `interface nsIUrlClassifierCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIUrlClassifierCallback {
    vtable: *const nsIUrlClassifierCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIUrlClassifierCallback.
unsafe impl XpCom for nsIUrlClassifierCallback {
    const IID: nsIID = nsID(0x4ca27b6b, 0xa674, 0x4b3d,
        [0xab, 0x30, 0xd2, 0x1e, 0x2d, 0xa2, 0xdf, 0xfb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIUrlClassifierCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIUrlClassifierCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIUrlClassifierCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIUrlClassifierCallback`.
    fn coerce_from(v: &nsIUrlClassifierCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIUrlClassifierCallbackCoerce for nsIUrlClassifierCallback {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierCallback) -> &Self {
        v
    }
}

impl nsIUrlClassifierCallback {
    /// Cast this `nsIUrlClassifierCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIUrlClassifierCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIUrlClassifierCallback {
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
impl<T: nsISupportsCoerce> nsIUrlClassifierCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIUrlClassifierCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIUrlClassifierCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIUrlClassifierCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handleEvent (in ACString value); */
    pub HandleEvent: unsafe extern "system" fn (this: *const nsIUrlClassifierCallback, value: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIUrlClassifierCallback {


    /// `void handleEvent (in ACString value);`
    #[inline]
    pub unsafe fn HandleEvent(&self, value: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).HandleEvent)(self, value)
    }


}


/// `interface nsIUrlClassifierUpdateObserver : nsISupports`
///

/// ```text
/// /**
///  * The nsIUrlClassifierUpdateObserver interface is implemented by
///  * clients streaming updates to the url-classifier (usually
    ///  * nsUrlClassifierStreamUpdater.
    ///  */
    /// ```
    ///

    // The actual type definition for the interface. This struct has methods
    // declared on it which will call through its vtable. You never want to pass
    // this type around by value, always pass it behind a reference.

    #[repr(C)]
    pub struct nsIUrlClassifierUpdateObserver {
        vtable: *const nsIUrlClassifierUpdateObserverVTable,

        /// This field is a phantomdata to ensure that the VTable type and any
        /// struct containing it is not safe to send across threads, as XPCOM is
        /// generally not threadsafe.
        ///
        /// XPCOM interfaces in general are not safe to send across threads.
        __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
    }

    // Implementing XpCom for an interface exposes its IID, which allows for easy
    // use of the `.query_interface<T>` helper method. This also defines that
    // method for nsIUrlClassifierUpdateObserver.
    unsafe impl XpCom for nsIUrlClassifierUpdateObserver {
        const IID: nsIID = nsID(0x9fa11561, 0x5816, 0x4e1b,
            [0xbc, 0xc9, 0xb6, 0x29, 0xca, 0x05, 0xcc, 0xe6]);
    }

    // We need to implement the RefCounted trait so we can be used with `RefPtr`.
    // This trait teaches `RefPtr` how to manage our memory.
    unsafe impl RefCounted for nsIUrlClassifierUpdateObserver {
        #[inline]
        unsafe fn addref(&self) {
            self.AddRef();
        }
        #[inline]
        unsafe fn release(&self) {
            self.Release();
        }
    }

    // This trait is implemented on all types which can be coerced to from nsIUrlClassifierUpdateObserver.
    // It is used in the implementation of `fn coerce<T>`. We hide it from the
    // documentation, because it clutters it up a lot.
    #[doc(hidden)]
    pub trait nsIUrlClassifierUpdateObserverCoerce {
        /// Cheaply cast a value of this type from a `nsIUrlClassifierUpdateObserver`.
        fn coerce_from(v: &nsIUrlClassifierUpdateObserver) -> &Self;
    }

    // The trivial implementation: We can obviously coerce ourselves to ourselves.
    impl nsIUrlClassifierUpdateObserverCoerce for nsIUrlClassifierUpdateObserver {
        #[inline]
        fn coerce_from(v: &nsIUrlClassifierUpdateObserver) -> &Self {
            v
        }
    }

    impl nsIUrlClassifierUpdateObserver {
        /// Cast this `nsIUrlClassifierUpdateObserver` to one of its base interfaces.
        #[inline]
        pub fn coerce<T: nsIUrlClassifierUpdateObserverCoerce>(&self) -> &T {
            T::coerce_from(self)
        }
    }

    // Every interface struct type implements `Deref` to its base interface. This
    // causes methods on the base interfaces to be directly avaliable on the
    // object. For example, you can call `.AddRef` or `.QueryInterface` directly
    // on any interface which inherits from `nsISupports`.
    impl ::std::ops::Deref for nsIUrlClassifierUpdateObserver {
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
    impl<T: nsISupportsCoerce> nsIUrlClassifierUpdateObserverCoerce for T {
        #[inline]
        fn coerce_from(v: &nsIUrlClassifierUpdateObserver) -> &Self {
            T::coerce_from(v)
        }
    }

    // This struct represents the interface's VTable. A pointer to a statically
    // allocated version of this struct is at the beginning of every nsIUrlClassifierUpdateObserver
    // object. It contains one pointer field for each method in the interface. In
    // the case where we can't generate a binding for a method, we include a void
    // pointer.
    #[doc(hidden)]
    #[repr(C)]
    pub struct nsIUrlClassifierUpdateObserverVTable {
        /// We need to include the members from the base interface's vtable at the start
        /// of the VTable definition.
        pub __base: nsISupportsVTable,

        /* void updateUrlRequested (in ACString url, in ACString table); */
        pub UpdateUrlRequested: unsafe extern "system" fn (this: *const nsIUrlClassifierUpdateObserver, url: *const ::nsstring::nsACString, table: *const ::nsstring::nsACString) -> ::nserror::nsresult,

        /* void streamFinished (in nsresult status, in unsigned long delay); */
        pub StreamFinished: unsafe extern "system" fn (this: *const nsIUrlClassifierUpdateObserver, status: ::nserror::nsresult, delay: u32) -> ::nserror::nsresult,

        /* void updateError (in nsresult error); */
        pub UpdateError: unsafe extern "system" fn (this: *const nsIUrlClassifierUpdateObserver, error: ::nserror::nsresult) -> ::nserror::nsresult,

        /* void updateSuccess (in unsigned long requestedTimeout); */
        pub UpdateSuccess: unsafe extern "system" fn (this: *const nsIUrlClassifierUpdateObserver, requestedTimeout: u32) -> ::nserror::nsresult,
    }


    // The implementations of the function wrappers which are exposed to rust code.
    // Call these methods rather than manually calling through the VTable struct.
    impl nsIUrlClassifierUpdateObserver {

        /// ```text
        /// /**
        ///    * The update requested a new URL whose contents should be downloaded
        ///    * and sent to the classifier as a new stream.
        ///    *
        ///    * @param url The url that was requested.
        ///    * @param table The table name that this URL's contents will be associated
        ///    *              with.  This should be passed back to beginStream().
        ///    */
        /// ```
        ///

        /// `void updateUrlRequested (in ACString url, in ACString table);`
        #[inline]
        pub unsafe fn UpdateUrlRequested(&self, url: *const ::nsstring::nsACString, table: *const ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).UpdateUrlRequested)(self, url, table)
        }


        /// ```text
        /// /**
        ///    * A stream update has completed.
        ///    *
        ///    * @param status The state of the update process.
        ///    * @param delay The amount of time the updater should wait to fetch the
        ///    *              next URL in ms.
        ///    */
        /// ```
        ///

        /// `void streamFinished (in nsresult status, in unsigned long delay);`
        #[inline]
        pub unsafe fn StreamFinished(&self, status: ::nserror::nsresult, delay: u32) -> ::nserror::nsresult {
            ((*self.vtable).StreamFinished)(self, status, delay)
        }



        /// `void updateError (in nsresult error);`
        #[inline]
        pub unsafe fn UpdateError(&self, error: ::nserror::nsresult) -> ::nserror::nsresult {
            ((*self.vtable).UpdateError)(self, error)
        }


        /// ```text
        /// /**
        ///    * The update has completed successfully.
        ///    *
        ///    * @param requestedTimeout The number of seconds that the caller should
        ///    *                         wait before trying to update again.
        ///    **/
        /// ```
        ///

        /// `void updateSuccess (in unsigned long requestedTimeout);`
        #[inline]
        pub unsafe fn UpdateSuccess(&self, requestedTimeout: u32) -> ::nserror::nsresult {
            ((*self.vtable).UpdateSuccess)(self, requestedTimeout)
        }


    }


    /// `interface nsIUrlClassifierDBService : nsISupports`
    ///

    /// ```text
    /// /**
    ///  * This is a proxy class that is instantiated and called from the JS thread.
    ///  * It provides async methods for querying and updating the database.  As the
    ///  * methods complete, they call the callback function.
    ///  */
    /// ```
    ///

    // The actual type definition for the interface. This struct has methods
    // declared on it which will call through its vtable. You never want to pass
    // this type around by value, always pass it behind a reference.

    #[repr(C)]
    pub struct nsIUrlClassifierDBService {
        vtable: *const nsIUrlClassifierDBServiceVTable,

        /// This field is a phantomdata to ensure that the VTable type and any
        /// struct containing it is not safe to send across threads, as XPCOM is
        /// generally not threadsafe.
        ///
        /// XPCOM interfaces in general are not safe to send across threads.
        __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
    }

    // Implementing XpCom for an interface exposes its IID, which allows for easy
    // use of the `.query_interface<T>` helper method. This also defines that
    // method for nsIUrlClassifierDBService.
    unsafe impl XpCom for nsIUrlClassifierDBService {
        const IID: nsIID = nsID(0x7a258022, 0x6765, 0x11e5,
            [0xb3, 0x79, 0xb3, 0x7b, 0x1f, 0x23, 0x54, 0xbe]);
    }

    // We need to implement the RefCounted trait so we can be used with `RefPtr`.
    // This trait teaches `RefPtr` how to manage our memory.
    unsafe impl RefCounted for nsIUrlClassifierDBService {
        #[inline]
        unsafe fn addref(&self) {
            self.AddRef();
        }
        #[inline]
        unsafe fn release(&self) {
            self.Release();
        }
    }

    // This trait is implemented on all types which can be coerced to from nsIUrlClassifierDBService.
    // It is used in the implementation of `fn coerce<T>`. We hide it from the
    // documentation, because it clutters it up a lot.
    #[doc(hidden)]
    pub trait nsIUrlClassifierDBServiceCoerce {
        /// Cheaply cast a value of this type from a `nsIUrlClassifierDBService`.
        fn coerce_from(v: &nsIUrlClassifierDBService) -> &Self;
    }

    // The trivial implementation: We can obviously coerce ourselves to ourselves.
    impl nsIUrlClassifierDBServiceCoerce for nsIUrlClassifierDBService {
        #[inline]
        fn coerce_from(v: &nsIUrlClassifierDBService) -> &Self {
            v
        }
    }

    impl nsIUrlClassifierDBService {
        /// Cast this `nsIUrlClassifierDBService` to one of its base interfaces.
        #[inline]
        pub fn coerce<T: nsIUrlClassifierDBServiceCoerce>(&self) -> &T {
            T::coerce_from(self)
        }
    }

    // Every interface struct type implements `Deref` to its base interface. This
    // causes methods on the base interfaces to be directly avaliable on the
    // object. For example, you can call `.AddRef` or `.QueryInterface` directly
    // on any interface which inherits from `nsISupports`.
    impl ::std::ops::Deref for nsIUrlClassifierDBService {
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
    impl<T: nsISupportsCoerce> nsIUrlClassifierDBServiceCoerce for T {
        #[inline]
        fn coerce_from(v: &nsIUrlClassifierDBService) -> &Self {
            T::coerce_from(v)
        }
    }

    // This struct represents the interface's VTable. A pointer to a statically
    // allocated version of this struct is at the beginning of every nsIUrlClassifierDBService
    // object. It contains one pointer field for each method in the interface. In
    // the case where we can't generate a binding for a method, we include a void
    // pointer.
    #[doc(hidden)]
    #[repr(C)]
    pub struct nsIUrlClassifierDBServiceVTable {
        /// We need to include the members from the base interface's vtable at the start
        /// of the VTable definition.
        pub __base: nsISupportsVTable,

        /* void lookup (in nsIPrincipal principal, in ACString tables, in nsIUrlClassifierCallback c); */
        pub Lookup: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService, principal: *const nsIPrincipal, tables: *const ::nsstring::nsACString, c: *const nsIUrlClassifierCallback) -> ::nserror::nsresult,

        /* void getTables (in nsIUrlClassifierCallback c); */
        pub GetTables: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService, c: *const nsIUrlClassifierCallback) -> ::nserror::nsresult,

        /* void setHashCompleter (in ACString tableName, in nsIUrlClassifierHashCompleter completer); */
        pub SetHashCompleter: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService, tableName: *const ::nsstring::nsACString, completer: *const nsIUrlClassifierHashCompleter) -> ::nserror::nsresult,

        /* void clearLastResults (); */
        pub ClearLastResults: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService) -> ::nserror::nsresult,

        /* void beginUpdate (in nsIUrlClassifierUpdateObserver updater, in ACString tables); */
        pub BeginUpdate: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService, updater: *const nsIUrlClassifierUpdateObserver, tables: *const ::nsstring::nsACString) -> ::nserror::nsresult,

        /* void beginStream (in ACString table); */
        pub BeginStream: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService, table: *const ::nsstring::nsACString) -> ::nserror::nsresult,

        /* void updateStream (in ACString updateChunk); */
        pub UpdateStream: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService, updateChunk: *const ::nsstring::nsACString) -> ::nserror::nsresult,

        /* void finishStream (); */
        pub FinishStream: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService) -> ::nserror::nsresult,

        /* void finishUpdate (); */
        pub FinishUpdate: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService) -> ::nserror::nsresult,

        /* void cancelUpdate (); */
        pub CancelUpdate: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService) -> ::nserror::nsresult,

        /* void resetDatabase (); */
        pub ResetDatabase: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService) -> ::nserror::nsresult,

        /* void reloadDatabase (); */
        pub ReloadDatabase: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService) -> ::nserror::nsresult,

        /* void clearCache (); */
        pub ClearCache: unsafe extern "system" fn (this: *const nsIUrlClassifierDBService) -> ::nserror::nsresult,
    }


    // The implementations of the function wrappers which are exposed to rust code.
    // Call these methods rather than manually calling through the VTable struct.
    impl nsIUrlClassifierDBService {

        /// ```text
        /// /**
        ///    * Looks up a URI in the specified tables.
        ///    *
        ///    * @param principal: The principal containing the URI to search.
        ///    * @param c: The callback will be called with a comma-separated list
        ///    *        of tables to which the key belongs.
        ///    */
        /// ```
        ///

        /// `void lookup (in nsIPrincipal principal, in ACString tables, in nsIUrlClassifierCallback c);`
        #[inline]
        pub unsafe fn Lookup(&self, principal: *const nsIPrincipal, tables: *const ::nsstring::nsACString, c: *const nsIUrlClassifierCallback) -> ::nserror::nsresult {
            ((*self.vtable).Lookup)(self, principal, tables, c)
        }


        /// ```text
        /// /**
        ///    * Lists the tables along with their meta info in the following format:
        ///    *
        ///    *   tablename;[metadata]\n
        ///    *   tablename2;[metadata]\n
        ///    *
        ///    * For v2 tables, the metadata is the chunks info such as
        ///    *
        ///    *   goog-phish-shavar;a:10,14,30-40s:56,67
        ///    *   goog-unwanted-shavar;a:1-3,5
        ///    *
        ///    * For v4 tables, base64 encoded state is currently the only info in the
        ///    * metadata (can be extended whenever necessary). For exmaple,
        ///    *
        ///    *   goog-phish-proto;Cg0IARAGGAEiAzAwMTABEKqTARoCGAjT1gDD:oCGAjT1gDD\n
        ///    *   goog-malware-proto;Cg0IAhAGGAEiAzAwMTABENCQARoCGAjx5Yty:BENCQARoCGAj\n
        ///    *
        ///    * Note that the metadata is colon-separated.
        ///    *
        ///    */
        /// ```
        ///

        /// `void getTables (in nsIUrlClassifierCallback c);`
        #[inline]
        pub unsafe fn GetTables(&self, c: *const nsIUrlClassifierCallback) -> ::nserror::nsresult {
            ((*self.vtable).GetTables)(self, c)
        }


        /// ```text
        /// /**
        ///    * Set the nsIUrlClassifierCompleter object for a given table.  This
        ///    * object will be used to request complete versions of partial
        ///    * hashes.
        ///    */
        /// ```
        ///

        /// `void setHashCompleter (in ACString tableName, in nsIUrlClassifierHashCompleter completer);`
        #[inline]
        pub unsafe fn SetHashCompleter(&self, tableName: *const ::nsstring::nsACString, completer: *const nsIUrlClassifierHashCompleter) -> ::nserror::nsresult {
            ((*self.vtable).SetHashCompleter)(self, tableName, completer)
        }


        /// ```text
        /// /**
        ///    * Forget the results that were used in the last DB update.
        ///    */
        /// ```
        ///

        /// `void clearLastResults ();`
        #[inline]
        pub unsafe fn ClearLastResults(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).ClearLastResults)(self, )
        }


        /// ```text
        /// /**
        ///    * Begin an update process.  Will throw NS_ERROR_NOT_AVAILABLE if there
        ///    * is already an update in progress.
        ///    *
        ///    * @param updater The update observer tied to this update.
        ///    * @param tables A comma-separated list of tables included in this update.
        ///    */
        /// ```
        ///

        /// `void beginUpdate (in nsIUrlClassifierUpdateObserver updater, in ACString tables);`
        #[inline]
        pub unsafe fn BeginUpdate(&self, updater: *const nsIUrlClassifierUpdateObserver, tables: *const ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).BeginUpdate)(self, updater, tables)
        }


        /// ```text
        /// /**
        ///    * Begin a stream update.  This should be called once per url being
        ///    * fetched.
        ///    *
        ///    * @param table The table the contents of this stream will be associated
        ///    *              with, or empty for the initial stream.
        ///    */
        /// ```
        ///

        /// `void beginStream (in ACString table);`
        #[inline]
        pub unsafe fn BeginStream(&self, table: *const ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).BeginStream)(self, table)
        }


        /// ```text
        /// /**
        ///    * Update the table incrementally.
        ///    */
        /// ```
        ///

        /// `void updateStream (in ACString updateChunk);`
        #[inline]
        pub unsafe fn UpdateStream(&self, updateChunk: *const ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).UpdateStream)(self, updateChunk)
        }


        /// ```text
        /// /**
        ///    * Finish an individual stream update.  Must be called for every
        ///    * beginStream() call, before the next beginStream() or finishUpdate().
        ///    *
        ///    * The update observer's streamFinished will be called once the
        ///    * stream has been processed.
        ///    */
        /// ```
        ///

        /// `void finishStream ();`
        #[inline]
        pub unsafe fn FinishStream(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).FinishStream)(self, )
        }


        /// ```text
        /// /**
        ///    * Finish an incremental update.  This will attempt to commit any
        ///    * pending changes and resets the update interface.
        ///    *
        ///    * The update observer's updateSucceeded or updateError methods
        ///    * will be called when the update has been processed.
        ///    */
        /// ```
        ///

        /// `void finishUpdate ();`
        #[inline]
        pub unsafe fn FinishUpdate(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).FinishUpdate)(self, )
        }


        /// ```text
        /// /**
        ///    * Cancel an incremental update.  This rolls back any pending changes.
        ///    * and resets the update interface.
        ///    *
        ///    * The update observer's updateError method will be called when the
        ///    * update has been rolled back.
        ///    */
        /// ```
        ///

        /// `void cancelUpdate ();`
        #[inline]
        pub unsafe fn CancelUpdate(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).CancelUpdate)(self, )
        }


        /// ```text
        /// /**
        ///    * Reset the url-classifier database.  This call will delete the existing
        ///    * database, emptying all tables.  Mostly intended for use in unit tests.
        ///    */
        /// ```
        ///

        /// `void resetDatabase ();`
        #[inline]
        pub unsafe fn ResetDatabase(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).ResetDatabase)(self, )
        }


        /// ```text
        /// /**
        ///    * Reload he url-classifier database. This will empty all cache for
        ///    * completions from gethash, and reload it from database. Mostly intended
        ///    * for use in tests.
        ///    */
        /// ```
        ///

        /// `void reloadDatabase ();`
        #[inline]
        pub unsafe fn ReloadDatabase(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).ReloadDatabase)(self, )
        }


        /// ```text
        /// /**
        ///    * Empty all the caches.
        ///    */
        /// ```
        ///

        /// `void clearCache ();`
        #[inline]
        pub unsafe fn ClearCache(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).ClearCache)(self, )
        }


    }


    /// `interface nsIUrlClassifierLookupCallback : nsISupports`
    ///

    /// ```text
    /// /**
    ///  * This is an internal helper interface for communication between the
    ///  * main thread and the dbservice worker thread.  It is called for each
    ///  * lookup to provide a set of possible results, which the main thread
    ///  * may need to expand using an nsIUrlClassifierCompleter.
    ///  */
    /// ```
    ///

    // The actual type definition for the interface. This struct has methods
    // declared on it which will call through its vtable. You never want to pass
    // this type around by value, always pass it behind a reference.

    #[repr(C)]
    pub struct nsIUrlClassifierLookupCallback {
        vtable: *const nsIUrlClassifierLookupCallbackVTable,

        /// This field is a phantomdata to ensure that the VTable type and any
        /// struct containing it is not safe to send across threads, as XPCOM is
        /// generally not threadsafe.
        ///
        /// XPCOM interfaces in general are not safe to send across threads.
        __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
    }

    // Implementing XpCom for an interface exposes its IID, which allows for easy
    // use of the `.query_interface<T>` helper method. This also defines that
    // method for nsIUrlClassifierLookupCallback.
    unsafe impl XpCom for nsIUrlClassifierLookupCallback {
        const IID: nsIID = nsID(0xb903dc8f, 0xdff1, 0x42fe,
            [0x89, 0x4b, 0x36, 0xe7, 0xa5, 0x9b, 0xb8, 0x01]);
    }

    // We need to implement the RefCounted trait so we can be used with `RefPtr`.
    // This trait teaches `RefPtr` how to manage our memory.
    unsafe impl RefCounted for nsIUrlClassifierLookupCallback {
        #[inline]
        unsafe fn addref(&self) {
            self.AddRef();
        }
        #[inline]
        unsafe fn release(&self) {
            self.Release();
        }
    }

    // This trait is implemented on all types which can be coerced to from nsIUrlClassifierLookupCallback.
    // It is used in the implementation of `fn coerce<T>`. We hide it from the
    // documentation, because it clutters it up a lot.
    #[doc(hidden)]
    pub trait nsIUrlClassifierLookupCallbackCoerce {
        /// Cheaply cast a value of this type from a `nsIUrlClassifierLookupCallback`.
        fn coerce_from(v: &nsIUrlClassifierLookupCallback) -> &Self;
    }

    // The trivial implementation: We can obviously coerce ourselves to ourselves.
    impl nsIUrlClassifierLookupCallbackCoerce for nsIUrlClassifierLookupCallback {
        #[inline]
        fn coerce_from(v: &nsIUrlClassifierLookupCallback) -> &Self {
            v
        }
    }

    impl nsIUrlClassifierLookupCallback {
        /// Cast this `nsIUrlClassifierLookupCallback` to one of its base interfaces.
        #[inline]
        pub fn coerce<T: nsIUrlClassifierLookupCallbackCoerce>(&self) -> &T {
            T::coerce_from(self)
        }
    }

    // Every interface struct type implements `Deref` to its base interface. This
    // causes methods on the base interfaces to be directly avaliable on the
    // object. For example, you can call `.AddRef` or `.QueryInterface` directly
    // on any interface which inherits from `nsISupports`.
    impl ::std::ops::Deref for nsIUrlClassifierLookupCallback {
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
    impl<T: nsISupportsCoerce> nsIUrlClassifierLookupCallbackCoerce for T {
        #[inline]
        fn coerce_from(v: &nsIUrlClassifierLookupCallback) -> &Self {
            T::coerce_from(v)
        }
    }

    // This struct represents the interface's VTable. A pointer to a statically
    // allocated version of this struct is at the beginning of every nsIUrlClassifierLookupCallback
    // object. It contains one pointer field for each method in the interface. In
    // the case where we can't generate a binding for a method, we include a void
    // pointer.
    #[doc(hidden)]
    #[repr(C)]
    pub struct nsIUrlClassifierLookupCallbackVTable {
        /// We need to include the members from the base interface's vtable at the start
        /// of the VTable definition.
        pub __base: nsISupportsVTable,

        /* void lookupComplete (in ResultArray results); */
        /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
        pub LookupComplete: *const ::libc::c_void,
    }


    // The implementations of the function wrappers which are exposed to rust code.
    // Call these methods rather than manually calling through the VTable struct.
    impl nsIUrlClassifierLookupCallback {

        /// ```text
        /// /**
        ///    * The lookup process is complete.
        ///    *
        ///    * @param results
        ///    *        If this parameter is null, there were no results found.
        ///    *        If not, it contains an array of nsUrlClassifierEntry objects
        ///    *        with possible matches.  The callee is responsible for freeing
        ///    *        this array.
        ///    */
        /// ```
        ///

        /// `void lookupComplete (in ResultArray results);`
        const _LookupComplete: () = ();

    }


    /// `interface nsIUrlClassifierClassifyCallback : nsISupports`
    ///

    /// ```text
    /// /**
    ///  * This is an internal helper interface which is called after each
    ///  * classify completes to provide and handle a set of possible results,
    ///  * which the main thread may need to expand using an nsIURIClassifierCallback.
    ///  */
    /// ```
    ///

    // The actual type definition for the interface. This struct has methods
    // declared on it which will call through its vtable. You never want to pass
    // this type around by value, always pass it behind a reference.

    #[repr(C)]
    pub struct nsIUrlClassifierClassifyCallback {
        vtable: *const nsIUrlClassifierClassifyCallbackVTable,

        /// This field is a phantomdata to ensure that the VTable type and any
        /// struct containing it is not safe to send across threads, as XPCOM is
        /// generally not threadsafe.
        ///
        /// XPCOM interfaces in general are not safe to send across threads.
        __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
    }

    // Implementing XpCom for an interface exposes its IID, which allows for easy
    // use of the `.query_interface<T>` helper method. This also defines that
    // method for nsIUrlClassifierClassifyCallback.
    unsafe impl XpCom for nsIUrlClassifierClassifyCallback {
        const IID: nsIID = nsID(0x091adf98, 0x28a5, 0x473d,
            [0x8d, 0xec, 0x5b, 0x34, 0xb4, 0xe6, 0x24, 0x96]);
    }

    // We need to implement the RefCounted trait so we can be used with `RefPtr`.
    // This trait teaches `RefPtr` how to manage our memory.
    unsafe impl RefCounted for nsIUrlClassifierClassifyCallback {
        #[inline]
        unsafe fn addref(&self) {
            self.AddRef();
        }
        #[inline]
        unsafe fn release(&self) {
            self.Release();
        }
    }

    // This trait is implemented on all types which can be coerced to from nsIUrlClassifierClassifyCallback.
    // It is used in the implementation of `fn coerce<T>`. We hide it from the
    // documentation, because it clutters it up a lot.
    #[doc(hidden)]
    pub trait nsIUrlClassifierClassifyCallbackCoerce {
        /// Cheaply cast a value of this type from a `nsIUrlClassifierClassifyCallback`.
        fn coerce_from(v: &nsIUrlClassifierClassifyCallback) -> &Self;
    }

    // The trivial implementation: We can obviously coerce ourselves to ourselves.
    impl nsIUrlClassifierClassifyCallbackCoerce for nsIUrlClassifierClassifyCallback {
        #[inline]
        fn coerce_from(v: &nsIUrlClassifierClassifyCallback) -> &Self {
            v
        }
    }

    impl nsIUrlClassifierClassifyCallback {
        /// Cast this `nsIUrlClassifierClassifyCallback` to one of its base interfaces.
        #[inline]
        pub fn coerce<T: nsIUrlClassifierClassifyCallbackCoerce>(&self) -> &T {
            T::coerce_from(self)
        }
    }

    // Every interface struct type implements `Deref` to its base interface. This
    // causes methods on the base interfaces to be directly avaliable on the
    // object. For example, you can call `.AddRef` or `.QueryInterface` directly
    // on any interface which inherits from `nsISupports`.
    impl ::std::ops::Deref for nsIUrlClassifierClassifyCallback {
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
    impl<T: nsISupportsCoerce> nsIUrlClassifierClassifyCallbackCoerce for T {
        #[inline]
        fn coerce_from(v: &nsIUrlClassifierClassifyCallback) -> &Self {
            T::coerce_from(v)
        }
    }

    // This struct represents the interface's VTable. A pointer to a statically
    // allocated version of this struct is at the beginning of every nsIUrlClassifierClassifyCallback
    // object. It contains one pointer field for each method in the interface. In
    // the case where we can't generate a binding for a method, we include a void
    // pointer.
    #[doc(hidden)]
    #[repr(C)]
    pub struct nsIUrlClassifierClassifyCallbackVTable {
        /// We need to include the members from the base interface's vtable at the start
        /// of the VTable definition.
        pub __base: nsISupportsVTable,

        /* void handleResult (in ACString aList, in ACString aPrefix); */
        pub HandleResult: unsafe extern "system" fn (this: *const nsIUrlClassifierClassifyCallback, aList: *const ::nsstring::nsACString, aPrefix: *const ::nsstring::nsACString) -> ::nserror::nsresult,
    }


    // The implementations of the function wrappers which are exposed to rust code.
    // Call these methods rather than manually calling through the VTable struct.
    impl nsIUrlClassifierClassifyCallback {

        /// ```text
        /// /**
        ///    * The function is called each time the URL matches a Safe Browsing list
        ///    * The function could be called multiple times if URL matches multiple lists
        ///    *
        ///    */
        /// ```
        ///

        /// `void handleResult (in ACString aList, in ACString aPrefix);`
        #[inline]
        pub unsafe fn HandleResult(&self, aList: *const ::nsstring::nsACString, aPrefix: *const ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).HandleResult)(self, aList, aPrefix)
        }


    }


