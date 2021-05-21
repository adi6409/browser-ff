//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageService.idl
//


/// `interface mozIStorageService : nsISupports`
///

/// ```text
/// /**
///  * The mozIStorageService interface is intended to be implemented by
///  * a service that can create storage connections (mozIStorageConnection)
///  * to either a well-known profile database or to a specific database file.
///  *
///  * This is the only way to open a database connection.
///  *
///  * @note The first reference to mozIStorageService must be made on the main
///  * thread.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageService {
    vtable: *const mozIStorageServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageService.
unsafe impl XpCom for mozIStorageService {
    const IID: nsIID = nsID(0x07b6b2f5, 0x6d97, 0x47b4,
        [0x95, 0x84, 0xe6, 0x5b, 0xc4, 0x67, 0xfe, 0x9e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageServiceCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageService`.
    fn coerce_from(v: &mozIStorageService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageServiceCoerce for mozIStorageService {
    #[inline]
    fn coerce_from(v: &mozIStorageService) -> &Self {
        v
    }
}

impl mozIStorageService {
    /// Cast this `mozIStorageService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageService {
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
impl<T: nsISupportsCoerce> mozIStorageServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void openAsyncDatabase (in nsIVariant aDatabaseStore, [optional] in nsIPropertyBag2 aOptions, in mozIStorageCompletionCallback aCallback); */
    pub OpenAsyncDatabase: unsafe extern "system" fn (this: *const mozIStorageService, aDatabaseStore: *const nsIVariant, aOptions: *const nsIPropertyBag2, aCallback: *const mozIStorageCompletionCallback) -> ::nserror::nsresult,

    /* mozIStorageConnection openSpecialDatabase (in ACString aStorageKey, [optional] in ACString aName); */
    pub OpenSpecialDatabase: unsafe extern "system" fn (this: *const mozIStorageService, aStorageKey: *const ::nsstring::nsACString, aName: *const ::nsstring::nsACString, _retval: *mut*const mozIStorageConnection) -> ::nserror::nsresult,

    /* mozIStorageConnection openDatabase (in nsIFile aDatabaseFile); */
    pub OpenDatabase: unsafe extern "system" fn (this: *const mozIStorageService, aDatabaseFile: *const nsIFile, _retval: *mut*const mozIStorageConnection) -> ::nserror::nsresult,

    /* mozIStorageConnection openUnsharedDatabase (in nsIFile aDatabaseFile); */
    pub OpenUnsharedDatabase: unsafe extern "system" fn (this: *const mozIStorageService, aDatabaseFile: *const nsIFile, _retval: *mut*const mozIStorageConnection) -> ::nserror::nsresult,

    /* mozIStorageConnection openDatabaseWithFileURL (in nsIFileURL aFileURL, [optional] in ACString aTelemetryFilename); */
    pub OpenDatabaseWithFileURL: unsafe extern "system" fn (this: *const mozIStorageService, aFileURL: *const nsIFileURL, aTelemetryFilename: *const ::nsstring::nsACString, _retval: *mut*const mozIStorageConnection) -> ::nserror::nsresult,

    /* nsIFile backupDatabaseFile (in nsIFile aDBFile, in AString aBackupFileName, [optional] in nsIFile aBackupParentDirectory); */
    pub BackupDatabaseFile: unsafe extern "system" fn (this: *const mozIStorageService, aDBFile: *const nsIFile, aBackupFileName: *const ::nsstring::nsAString, aBackupParentDirectory: *const nsIFile, _retval: *mut*const nsIFile) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageService {

    /// ```text
    /// /**
    ///    * Open an asynchronous connection to a database.
    ///    *
    ///    * This method MUST be called from the main thread. The connection object
    ///    * returned by this function is not threadsafe. You MUST use it only from
    ///    * the main thread.
    ///    *
    ///    * If you have more than one connection to a file, you MUST use the EXACT
    ///    * SAME NAME for the file each time, including case. The sqlite code uses
    ///    * a simple string compare to see if there is already a connection. Opening
    ///    * a connection to "Foo.sqlite" and "foo.sqlite" will CORRUPT YOUR DATABASE.
    ///    *
    ///    * @param aDatabaseStore Either a nsIFile representing the file that contains
    ///    * the database or a special string to open a special database. The special
    ///    * string may be:
    ///    * - "memory" to open an in-memory database.
    ///    *
    ///    * @param aOptions A set of options (may be null). Options may contain:
    ///    * - bool shared (defaults to |false|).
    ///    *   -- If |true|, opens the database with a shared-cache. The
    ///    *     shared-cache mode is more memory-efficient when many
    ///    *     connections to the same database are expected, though, the
    ///    *     connections will contend the cache resource. In any cases
    ///    *     where performance matter, working without a shared-cache will
    ///    *     improve concurrency.  @see openUnsharedDatabase
    ///    *
    ///    * - int growthIncrement (defaults to none).
    ///    *   -- Set the growth increment for the main database.  This hints SQLite to
    ///    *      grow the database file by a given chunk size and may reduce
    ///    *      filesystem fragmentation on large databases.
    ///    *      @see mozIStorageConnection::setGrowthIncrement
    ///    *
    ///    * @param aCallback A callback that will receive the result of the operation.
    ///    *  In case of error, it may receive as status:
    ///    *  - NS_ERROR_OUT_OF_MEMORY if allocating a new storage object fails.
    ///    *  - NS_ERROR_FILE_CORRUPTED if the database file is corrupted.
    ///    *  In case of success, it receives as argument the new database
    ///    *  connection, as an instance of |mozIStorageAsyncConnection|.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG if |aDatabaseStore| is neither a file nor
    ///    *         one of the special strings understood by this method, or if one of
    ///    *         the options passed through |aOptions| does not have the right type.
    ///    * @throws NS_ERROR_NOT_SAME_THREAD if called from a thread other than the
    ///    *         main thread.
    ///    */
    /// ```
    ///

    /// `void openAsyncDatabase (in nsIVariant aDatabaseStore, [optional] in nsIPropertyBag2 aOptions, in mozIStorageCompletionCallback aCallback);`
    #[inline]
    pub unsafe fn OpenAsyncDatabase(&self, aDatabaseStore: *const nsIVariant, aOptions: *const nsIPropertyBag2, aCallback: *const mozIStorageCompletionCallback) -> ::nserror::nsresult {
        ((*self.vtable).OpenAsyncDatabase)(self, aDatabaseStore, aOptions, aCallback)
    }


    /// ```text
    /// /**
    ///    * Get a connection to a named special database storage.
    ///    *
    ///    * @param aStorageKey a string key identifying the type of storage
    ///    * requested.  Valid values include: "memory".
    ///    *
    ///    * @param aName an optional string identifying the name of the database.
    ///    * If omitted, a filename of ":memory:" will be used which results in a
    ///    * private in-memory database specific to this connection, making it
    ///    * impossible to clone the in-memory database. If you want to be able to
    ///    * clone the connection (or otherwise connect to the in-memory database from
        ///    * a connection), then you must pick a name that's sufficiently unique within
    ///    * the process to not collide with other mozStorage users.
    ///    *
    ///    * @see openDatabase for restrictions on how database connections may be
    ///    * used. For the profile database, you should only access it from the main
    ///    * thread since other callers may also have connections.
    ///    *
    ///    * @returns a new mozIStorageConnection for the requested
    ///    * storage database.
    ///    *
    ///    * @throws NS_ERROR_INVALID_ARG if aStorageKey is invalid.
    ///    */
    /// ```
    ///

    /// `mozIStorageConnection openSpecialDatabase (in ACString aStorageKey, [optional] in ACString aName);`
    #[inline]
    pub unsafe fn OpenSpecialDatabase(&self, aStorageKey: *const ::nsstring::nsACString, aName: *const ::nsstring::nsACString, _retval: *mut*const mozIStorageConnection) -> ::nserror::nsresult {
        ((*self.vtable).OpenSpecialDatabase)(self, aStorageKey, aName, _retval)
    }


    /// ```text
    /// /**
    ///    * Open a connection to the specified file.
    ///    *
    ///    * Consumers should check mozIStorageConnection::connectionReady to ensure
    ///    * that they can use the database.  If this value is false, it is strongly
    ///    * recommended that the database be backed up with
    ///    * mozIStorageConnection::backupDB so user data is not lost.
    ///    *
    ///    * ==========
    ///    *   DANGER
    ///    * ==========
    ///    *
    ///    * If you have more than one connection to a file, you MUST use the EXACT
    ///    * SAME NAME for the file each time, including case. The sqlite code uses
    ///    * a simple string compare to see if there is already a connection. Opening
    ///    * a connection to "Foo.sqlite" and "foo.sqlite" will CORRUPT YOUR DATABASE.
    ///    *
    ///    * The connection object returned by this function is not threadsafe. You must
    ///    * use it only from the thread you created it from.
    ///    *
    ///    * @param aDatabaseFile
    ///    *        A nsIFile that represents the database that is to be opened..
    ///    *
    ///    * @returns a mozIStorageConnection for the requested database file.
    ///    *
    ///    * @throws NS_ERROR_OUT_OF_MEMORY
    ///    *         If allocating a new storage object fails.
    ///    * @throws NS_ERROR_FILE_CORRUPTED
    ///    *         If the database file is corrupted.
    ///    */
    /// ```
    ///

    /// `mozIStorageConnection openDatabase (in nsIFile aDatabaseFile);`
    #[inline]
    pub unsafe fn OpenDatabase(&self, aDatabaseFile: *const nsIFile, _retval: *mut*const mozIStorageConnection) -> ::nserror::nsresult {
        ((*self.vtable).OpenDatabase)(self, aDatabaseFile, _retval)
    }


    /// ```text
    /// /**
    ///    * Open a connection to the specified file that doesn't share a sqlite cache.
    ///    *
    ///    * Without a shared-cache, each connection uses its own pages cache, which
    ///    * may be memory inefficient with a large number of connections, in such a
    ///    * case so you should use openDatabase instead.  On the other side, if cache
    ///    * contention may be an issue, for instance when concurrency is important to
    ///    * ensure responsiveness, using unshared connections may be a performance win.
    ///    *
    ///    * ==========
    ///    *   DANGER
    ///    * ==========
    ///    *
    ///    * If you have more than one connection to a file, you MUST use the EXACT
    ///    * SAME NAME for the file each time, including case. The sqlite code uses
    ///    * a simple string compare to see if there is already a connection. Opening
    ///    * a connection to "Foo.sqlite" and "foo.sqlite" will CORRUPT YOUR DATABASE.
    ///    *
    ///    * The connection object returned by this function is not threadsafe. You must
    ///    * use it only from the thread you created it from.
    ///    *
    ///    * @param aDatabaseFile
    ///    *        A nsIFile that represents the database that is to be opened.
    ///    *
    ///    * @returns a mozIStorageConnection for the requested database file.
    ///    *
    ///    * @throws NS_ERROR_OUT_OF_MEMORY
    ///    *         If allocating a new storage object fails.
    ///    * @throws NS_ERROR_FILE_CORRUPTED
    ///    *         If the database file is corrupted.
    ///    */
    /// ```
    ///

    /// `mozIStorageConnection openUnsharedDatabase (in nsIFile aDatabaseFile);`
    #[inline]
    pub unsafe fn OpenUnsharedDatabase(&self, aDatabaseFile: *const nsIFile, _retval: *mut*const mozIStorageConnection) -> ::nserror::nsresult {
        ((*self.vtable).OpenUnsharedDatabase)(self, aDatabaseFile, _retval)
    }


    /// ```text
    /// /**
    ///    * See openDatabase(). Exactly the same only initialized with a file URL.
    ///    * Custom parameters can be passed to SQLite and VFS implementations through
    ///    * the query part of the URL.
    ///    *
    ///    * @param aURL
    ///    *        A nsIFileURL that represents the database that is to be opened.
    ///    * @param [optional] aTelemetryFilename
    ///    *        The name to use for the database in telemetry. Only needed if the
    ///    *        actual filename can contain sensitive information.
    ///    */
    /// ```
    ///

    /// `mozIStorageConnection openDatabaseWithFileURL (in nsIFileURL aFileURL, [optional] in ACString aTelemetryFilename);`
    #[inline]
    pub unsafe fn OpenDatabaseWithFileURL(&self, aFileURL: *const nsIFileURL, aTelemetryFilename: *const ::nsstring::nsACString, _retval: *mut*const mozIStorageConnection) -> ::nserror::nsresult {
        ((*self.vtable).OpenDatabaseWithFileURL)(self, aFileURL, aTelemetryFilename, _retval)
    }


    /// ```text
    /// /**
    ///    * Copies the specified database file to the specified parent directory with
    ///    * the specified file name.  If the parent directory is not specified, it
    ///    * places the backup in the same directory as the current file.  This function
    ///    * ensures that the file being created is unique.
    ///    *
    ///    * @param aDBFile
    ///    *        The database file that will be backed up.
    ///    * @param aBackupFileName
    ///    *        The name of the new backup file to create.
    ///    * @param [optional] aBackupParentDirectory
    ///    *        The directory you'd like the backup file to be placed.
    ///    * @return The nsIFile representing the backup file.
    ///    */
    /// ```
    ///

    /// `nsIFile backupDatabaseFile (in nsIFile aDBFile, in AString aBackupFileName, [optional] in nsIFile aBackupParentDirectory);`
    #[inline]
    pub unsafe fn BackupDatabaseFile(&self, aDBFile: *const nsIFile, aBackupFileName: *const ::nsstring::nsAString, aBackupParentDirectory: *const nsIFile, _retval: *mut*const nsIFile) -> ::nserror::nsresult {
        ((*self.vtable).BackupDatabaseFile)(self, aDBFile, aBackupFileName, aBackupParentDirectory, _retval)
    }


}


