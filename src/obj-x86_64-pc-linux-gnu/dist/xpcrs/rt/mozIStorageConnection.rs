//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageConnection.idl
//


/// `interface mozIStorageConnection : mozIStorageAsyncConnection`
///

/// ```text
/// /**
///  * mozIStorageConnection represents a database connection attached to
///  * a specific file or to the in-memory data storage.  It is the
///  * primary interface for interacting with a database, including
///  * creating prepared statements, executing SQL, and examining database
///  * errors.
///  *
///  * @note From the main thread, you should rather use mozIStorageAsyncConnection.
///  *
///  * @threadsafe
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageConnection {
    vtable: *const mozIStorageConnectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageConnection.
unsafe impl XpCom for mozIStorageConnection {
    const IID: nsIID = nsID(0x4aa2ac47, 0x8d24, 0x4004,
        [0x9b, 0x31, 0xec, 0x0b, 0xd8, 0x5f, 0x0c, 0xc3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageConnection {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageConnection.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageConnectionCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageConnection`.
    fn coerce_from(v: &mozIStorageConnection) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageConnectionCoerce for mozIStorageConnection {
    #[inline]
    fn coerce_from(v: &mozIStorageConnection) -> &Self {
        v
    }
}

impl mozIStorageConnection {
    /// Cast this `mozIStorageConnection` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageConnectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageConnection {
    type Target = mozIStorageAsyncConnection;
    #[inline]
    fn deref(&self) -> &mozIStorageAsyncConnection {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: mozIStorageAsyncConnectionCoerce> mozIStorageConnectionCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageConnection) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageConnection
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageConnectionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: mozIStorageAsyncConnectionVTable,

    /* void close (); */
    pub Close: unsafe extern "system" fn (this: *const mozIStorageConnection) -> ::nserror::nsresult,

    /* mozIStorageConnection clone ([optional] in boolean aReadOnly); */
    pub Clone: unsafe extern "system" fn (this: *const mozIStorageConnection, aReadOnly: bool, _retval: *mut *const mozIStorageConnection) -> ::nserror::nsresult,

    /* readonly attribute long defaultPageSize; */
    pub GetDefaultPageSize: unsafe extern "system" fn (this: *const mozIStorageConnection, aDefaultPageSize: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute boolean connectionReady; */
    pub GetConnectionReady: unsafe extern "system" fn (this: *const mozIStorageConnection, aConnectionReady: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long long lastInsertRowID; */
    pub GetLastInsertRowID: unsafe extern "system" fn (this: *const mozIStorageConnection, aLastInsertRowID: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long affectedRows; */
    pub GetAffectedRows: unsafe extern "system" fn (this: *const mozIStorageConnection, aAffectedRows: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long lastError; */
    pub GetLastError: unsafe extern "system" fn (this: *const mozIStorageConnection, aLastError: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String lastErrorString; */
    pub GetLastErrorString: unsafe extern "system" fn (this: *const mozIStorageConnection, aLastErrorString: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute long schemaVersion; */
    pub GetSchemaVersion: unsafe extern "system" fn (this: *const mozIStorageConnection, aSchemaVersion: *mut i32) -> ::nserror::nsresult,

    /* attribute long schemaVersion; */
    pub SetSchemaVersion: unsafe extern "system" fn (this: *const mozIStorageConnection, aSchemaVersion: i32) -> ::nserror::nsresult,

    /* mozIStorageStatement createStatement (in AUTF8String aSQLStatement); */
    pub CreateStatement: unsafe extern "system" fn (this: *const mozIStorageConnection, aSQLStatement: *const ::nsstring::nsACString, _retval: *mut*const mozIStorageStatement) -> ::nserror::nsresult,

    /* void executeSimpleSQL (in AUTF8String aSQLStatement); */
    pub ExecuteSimpleSQL: unsafe extern "system" fn (this: *const mozIStorageConnection, aSQLStatement: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* boolean tableExists (in AUTF8String aTableName); */
    pub TableExists: unsafe extern "system" fn (this: *const mozIStorageConnection, aTableName: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* boolean indexExists (in AUTF8String aIndexName); */
    pub IndexExists: unsafe extern "system" fn (this: *const mozIStorageConnection, aIndexName: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult,

    /* void beginTransaction (); */
    pub BeginTransaction: unsafe extern "system" fn (this: *const mozIStorageConnection) -> ::nserror::nsresult,

    /* void commitTransaction (); */
    pub CommitTransaction: unsafe extern "system" fn (this: *const mozIStorageConnection) -> ::nserror::nsresult,

    /* void rollbackTransaction (); */
    pub RollbackTransaction: unsafe extern "system" fn (this: *const mozIStorageConnection) -> ::nserror::nsresult,

    /* void createTable (in string aTableName, in string aTableSchema); */
    pub CreateTable: unsafe extern "system" fn (this: *const mozIStorageConnection, aTableName: *const libc::c_char, aTableSchema: *const libc::c_char) -> ::nserror::nsresult,

    /* void setGrowthIncrement (in int32_t aIncrement, in AUTF8String aDatabaseName); */
    pub SetGrowthIncrement: unsafe extern "system" fn (this: *const mozIStorageConnection, aIncrement: int32_t, aDatabaseName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void enableModule (in ACString aModuleName); */
    pub EnableModule: unsafe extern "system" fn (this: *const mozIStorageConnection, aModuleName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] void getQuotaObjects (out QuotaObject aDatabaseQuotaObject, out QuotaObject aJournalQuotaObject); */
    /// Unable to generate binding because `native type mozilla::dom::quota::QuotaObject unsupported`
    pub GetQuotaObjects: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageConnection {

    /// ```text
    /// /**
    ///    * Closes a database connection.  Callers must finalize all statements created
    ///    * for this connection prior to calling this method.  It is illegal to use
    ///    * call this method if any asynchronous statements have been executed on this
    ///    * connection.
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *         If any statement has been executed asynchronously on this object.
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *         If is called on a thread other than the one that opened it.
    ///    */
    /// ```
    ///

    /// `void close ();`
    #[inline]
    pub unsafe fn Close(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, )
    }


    /// ```text
    /// /**
    ///    * Clones a database connection and makes the clone read only if needed.
    ///    * SQL Functions and attached on-disk databases are applied to the new clone.
    ///    *
    ///    * @param aReadOnly
    ///    *        If true, the returned database should be put into read-only mode.
    ///    *        Defaults to false.
    ///    * @return the cloned database connection.
    ///    *
    ///    * @throws NS_ERROR_UNEXPECTED
    ///    *         If this connection is a memory database.
    ///    * @note If your connection is already read-only, you will get a read-only
    ///    *       clone.
    ///    * @note Due to a bug in SQLite, if you use the shared cache (openDatabase),
    ///    *       you end up with the same privileges as the first connection opened
    ///    *       regardless of what is specified in aReadOnly.
    ///    * @note The following pragmas are copied over to a read-only clone:
    ///    *        - cache_size
    ///    *        - temp_store
    ///    *       The following pragmas are copied over to a writeable clone:
    ///    *        - cache_size
    ///    *        - temp_store
    ///    *        - foreign_keys
    ///    *        - journal_size_limit
    ///    *        - synchronous
    ///    *        - wal_autocheckpoint
    ///    *       All SQL functions are copied over to read-only and writeable clones.
    ///    *       Additionally, all temporary tables, triggers, and views, as well as
    ///    *       any indexes on temporary tables, are copied over to writeable clones.
    ///    *       For temporary tables, only the schemas are copied, not their
    ///    *       contents.
    ///    *
    ///    */
    /// ```
    ///

    /// `mozIStorageConnection clone ([optional] in boolean aReadOnly);`
    #[inline]
    pub unsafe fn Clone(&self, aReadOnly: bool, _retval: *mut *const mozIStorageConnection) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, aReadOnly, _retval)
    }


    /// ```text
    /// /**
    ///    * The default size for SQLite database pages used by mozStorage for new
    ///    * databases.
    ///    */
    /// ```
    ///

    /// `readonly attribute long defaultPageSize;`
    #[inline]
    pub unsafe fn GetDefaultPageSize(&self, aDefaultPageSize: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultPageSize)(self, aDefaultPageSize)
    }


    /// ```text
    /// /**
    ///    * Indicates if the connection is open and ready to use.  This will be false
    ///    * if the connection failed to open, or it has been closed.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean connectionReady;`
    #[inline]
    pub unsafe fn GetConnectionReady(&self, aConnectionReady: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetConnectionReady)(self, aConnectionReady)
    }


    /// ```text
    /// /**
    ///    * lastInsertRowID returns the row ID from the last INSERT
    ///    * operation.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long lastInsertRowID;`
    #[inline]
    pub unsafe fn GetLastInsertRowID(&self, aLastInsertRowID: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetLastInsertRowID)(self, aLastInsertRowID)
    }


    /// ```text
    /// /**
    ///    * affectedRows returns the number of database rows that were changed or
    ///    * inserted or deleted by last operation.
    ///    */
    /// ```
    ///

    /// `readonly attribute long affectedRows;`
    #[inline]
    pub unsafe fn GetAffectedRows(&self, aAffectedRows: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetAffectedRows)(self, aAffectedRows)
    }


    /// ```text
    /// /**
    ///    * The last error SQLite error code.
    ///    */
    /// ```
    ///

    /// `readonly attribute long lastError;`
    #[inline]
    pub unsafe fn GetLastError(&self, aLastError: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetLastError)(self, aLastError)
    }


    /// ```text
    /// /**
    ///    * The last SQLite error as a string (in english, straight from the
        ///    * sqlite library).
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String lastErrorString;`
    #[inline]
    pub unsafe fn GetLastErrorString(&self, aLastErrorString: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetLastErrorString)(self, aLastErrorString)
    }


    /// ```text
    /// /**
    ///    * The schema version of the database.  This should not be used until the
    ///    * database is ready.  The schema will be reported as zero if it is not set.
    ///    */
    /// ```
    ///

    /// `attribute long schemaVersion;`
    #[inline]
    pub unsafe fn GetSchemaVersion(&self, aSchemaVersion: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetSchemaVersion)(self, aSchemaVersion)
    }


    /// ```text
    /// /**
    ///    * The schema version of the database.  This should not be used until the
    ///    * database is ready.  The schema will be reported as zero if it is not set.
    ///    */
    /// ```
    ///

    /// `attribute long schemaVersion;`
    #[inline]
    pub unsafe fn SetSchemaVersion(&self, aSchemaVersion: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetSchemaVersion)(self, aSchemaVersion)
    }


    /// ```text
    /// /**
    ///    * Create a mozIStorageStatement for the given SQL expression.  The
    ///    * expression may use ? to indicate sequential numbered arguments,
    ///    * ?1, ?2 etc. to indicate specific numbered arguments or :name and
    ///    * $var to indicate named arguments.
    ///    *
    ///    * @param aSQLStatement
    ///    *        The SQL statement to execute.
    ///    * @return a new mozIStorageStatement
    ///    */
    /// ```
    ///

    /// `mozIStorageStatement createStatement (in AUTF8String aSQLStatement);`
    #[inline]
    pub unsafe fn CreateStatement(&self, aSQLStatement: *const ::nsstring::nsACString, _retval: *mut*const mozIStorageStatement) -> ::nserror::nsresult {
        ((*self.vtable).CreateStatement)(self, aSQLStatement, _retval)
    }


    /// ```text
    /// /**
    ///    * Execute a SQL expression, expecting no arguments.
    ///    *
    ///    * @param aSQLStatement  The SQL statement to execute
    ///    */
    /// ```
    ///

    /// `void executeSimpleSQL (in AUTF8String aSQLStatement);`
    #[inline]
    pub unsafe fn ExecuteSimpleSQL(&self, aSQLStatement: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).ExecuteSimpleSQL)(self, aSQLStatement)
    }


    /// ```text
    /// /**
    ///    * Check if the given table exists.
    ///    *
    ///    * @param aTableName
    ///    *        The table to check
    ///    * @return TRUE if table exists, FALSE otherwise.
    ///    */
    /// ```
    ///

    /// `boolean tableExists (in AUTF8String aTableName);`
    #[inline]
    pub unsafe fn TableExists(&self, aTableName: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).TableExists)(self, aTableName, _retval)
    }


    /// ```text
    /// /**
    ///    * Check if the given index exists.
    ///    *
    ///    * @param aIndexName   The index to check
    ///    * @return TRUE if the index exists, FALSE otherwise.
    ///    */
    /// ```
    ///

    /// `boolean indexExists (in AUTF8String aIndexName);`
    #[inline]
    pub unsafe fn IndexExists(&self, aIndexName: *const ::nsstring::nsACString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IndexExists)(self, aIndexName, _retval)
    }


    /// ```text
    /// /**
    ///    * Begin a new transaction. If a transaction is active, throws an error.
    ///    */
    /// ```
    ///

    /// `void beginTransaction ();`
    #[inline]
    pub unsafe fn BeginTransaction(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BeginTransaction)(self, )
    }


    /// ```text
    /// /**
    ///    * Commits the current transaction.  If no transaction is active,
    ///    * @throws NS_ERROR_UNEXPECTED.
    ///    * @throws NS_ERROR_NOT_INITIALIZED.
    ///    */
    /// ```
    ///

    /// `void commitTransaction ();`
    #[inline]
    pub unsafe fn CommitTransaction(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CommitTransaction)(self, )
    }


    /// ```text
    /// /**
    ///    * Rolls back the current transaction.  If no transaction is active,
    ///    * @throws NS_ERROR_UNEXPECTED.
    ///    * @throws NS_ERROR_NOT_INITIALIZED.
    ///    */
    /// ```
    ///

    /// `void rollbackTransaction ();`
    #[inline]
    pub unsafe fn RollbackTransaction(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RollbackTransaction)(self, )
    }


    /// ```text
    /// /**
    ///    * Create the table with the given name and schema.
    ///    *
    ///    * If the table already exists, NS_ERROR_FAILURE is thrown.
    ///    * (XXX at some point in the future it will check if the schema is
        ///    * the same as what is specified, but that doesn't happen currently.)
    ///    *
    ///    * @param aTableName
    ///    *        The table name to be created, consisting of [A-Za-z0-9_], and
    ///    *        beginning with a letter.
    ///    * @param aTableSchema
    ///    *        The schema of the table; what would normally go between the parens
    ///    *        in a CREATE TABLE statement: e.g., "foo  INTEGER, bar STRING".
    ///    *
    ///    * @throws NS_ERROR_FAILURE
    ///    *         If the table already exists or could not be created for any other
    ///    *         reason.
    ///    */
    /// ```
    ///

    /// `void createTable (in string aTableName, in string aTableSchema);`
    #[inline]
    pub unsafe fn CreateTable(&self, aTableName: *const libc::c_char, aTableSchema: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).CreateTable)(self, aTableName, aTableSchema)
    }


    /// ```text
    /// /**
    ///    * Controls SQLITE_FCNTL_CHUNK_SIZE setting in sqlite. This helps avoid fragmentation
    ///    * by growing/shrinking the database file in SQLITE_FCNTL_CHUNK_SIZE increments. To
    ///    * conserve memory on systems short on storage space, this function will have no effect
    ///    * on mobile devices or if less than 500MiB of space is left available.
    ///    *
    ///    * @param aIncrement
    ///    *        The database file will grow in multiples of chunkSize.
    ///    * @param aDatabaseName
    ///    *        Sqlite database name. "" means pass NULL for zDbName to sqlite3_file_control.
    ///    *        See http://sqlite.org/c3ref/file_control.html for more details.
    ///    * @throws NS_ERROR_FILE_TOO_BIG
    ///    *         If the system is short on storage space.
    ///    */
    /// ```
    ///

    /// `void setGrowthIncrement (in int32_t aIncrement, in AUTF8String aDatabaseName);`
    #[inline]
    pub unsafe fn SetGrowthIncrement(&self, aIncrement: int32_t, aDatabaseName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetGrowthIncrement)(self, aIncrement, aDatabaseName)
    }


    /// ```text
    /// /**
    ///    * Enable a predefined virtual table implementation.
    ///    *
    ///    * @param aModuleName
    ///    *        The module to enable. Only "filesystem" is currently supported.
    ///    *
    ///    * @throws NS_ERROR_FAILURE
    ///    *         For unknown module names.
    ///    */
    /// ```
    ///

    /// `[noscript] void enableModule (in ACString aModuleName);`
    #[inline]
    pub unsafe fn EnableModule(&self, aModuleName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).EnableModule)(self, aModuleName)
    }


    /// ```text
    /// /**
    ///    * Get quota objects.
    ///    *
    ///    * @param[out] aDatabaseQuotaObject
    ///    *             The QuotaObject associated with the database file.
    ///    * @param[out] aJournalQuotaObject
    ///    *             The QuotaObject associated with the journal file.
    ///    *
    ///    * @throws NS_ERROR_NOT_INITIALIZED.
    ///    */
    /// ```
    ///

    /// `[noscript] void getQuotaObjects (out QuotaObject aDatabaseQuotaObject, out QuotaObject aJournalQuotaObject);`
    const _GetQuotaObjects: () = ();

}


