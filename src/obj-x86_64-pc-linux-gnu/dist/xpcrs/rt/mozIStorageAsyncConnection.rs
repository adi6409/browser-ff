//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageAsyncConnection.idl
//


/// `interface mozIStorageAsyncConnection : nsISupports`
///

/// ```text
/// /**
///  * mozIStorageAsyncConnection represents an asynchronous database
///  * connection attached to a specific file or to an in-memory data
///  * storage.  It is the primary interface for interacting with a
///  * database from the main thread, including creating prepared
///  * statements, executing SQL, and examining database errors.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIStorageAsyncConnection {
    vtable: *const mozIStorageAsyncConnectionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIStorageAsyncConnection.
unsafe impl XpCom for mozIStorageAsyncConnection {
    const IID: nsIID = nsID(0x8bfd34d5, 0x4ddf, 0x4e4b,
        [0x89, 0xdd, 0x9b, 0x14, 0xf3, 0x35, 0x34, 0xc6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIStorageAsyncConnection {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIStorageAsyncConnection.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIStorageAsyncConnectionCoerce {
    /// Cheaply cast a value of this type from a `mozIStorageAsyncConnection`.
    fn coerce_from(v: &mozIStorageAsyncConnection) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIStorageAsyncConnectionCoerce for mozIStorageAsyncConnection {
    #[inline]
    fn coerce_from(v: &mozIStorageAsyncConnection) -> &Self {
        v
    }
}

impl mozIStorageAsyncConnection {
    /// Cast this `mozIStorageAsyncConnection` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIStorageAsyncConnectionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIStorageAsyncConnection {
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
impl<T: nsISupportsCoerce> mozIStorageAsyncConnectionCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIStorageAsyncConnection) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIStorageAsyncConnection
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIStorageAsyncConnectionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute int32_t defaultTransactionType; */
    pub GetDefaultTransactionType: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aDefaultTransactionType: *mut int32_t) -> ::nserror::nsresult,

    /* attribute int32_t defaultTransactionType; */
    pub SetDefaultTransactionType: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aDefaultTransactionType: int32_t) -> ::nserror::nsresult,

    /* readonly attribute int32_t variableLimit; */
    pub GetVariableLimit: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aVariableLimit: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute boolean transactionInProgress; */
    pub GetTransactionInProgress: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aTransactionInProgress: *mut bool) -> ::nserror::nsresult,

    /* void asyncClose ([optional] in mozIStorageCompletionCallback aCallback); */
    pub AsyncClose: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aCallback: *const mozIStorageCompletionCallback) -> ::nserror::nsresult,

    /* [noscript] void spinningSynchronousClose (); */
    pub SpinningSynchronousClose: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection) -> ::nserror::nsresult,

    /* void asyncClone (in boolean aReadOnly, in mozIStorageCompletionCallback aCallback); */
    pub AsyncClone: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aReadOnly: bool, aCallback: *const mozIStorageCompletionCallback) -> ::nserror::nsresult,

    /* readonly attribute nsIFile databaseFile; */
    pub GetDatabaseFile: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aDatabaseFile: *mut*const nsIFile) -> ::nserror::nsresult,

    /* void interrupt (); */
    pub Interrupt: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection) -> ::nserror::nsresult,

    /* mozIStorageAsyncStatement createAsyncStatement (in AUTF8String aSQLStatement); */
    pub CreateAsyncStatement: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aSQLStatement: *const ::nsstring::nsACString, _retval: *mut*const mozIStorageAsyncStatement) -> ::nserror::nsresult,

    /* mozIStoragePendingStatement executeAsync (in Array<mozIStorageBaseStatement> aStatements, [optional] in mozIStorageStatementCallback aCallback); */
    pub ExecuteAsync: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aStatements: *const thin_vec::ThinVec<RefPtr<mozIStorageBaseStatement>>, aCallback: *const mozIStorageStatementCallback, _retval: *mut*const mozIStoragePendingStatement) -> ::nserror::nsresult,

    /* mozIStoragePendingStatement executeSimpleSQLAsync (in AUTF8String aSQLStatement, [optional] in mozIStorageStatementCallback aCallback); */
    pub ExecuteSimpleSQLAsync: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aSQLStatement: *const ::nsstring::nsACString, aCallback: *const mozIStorageStatementCallback, _retval: *mut*const mozIStoragePendingStatement) -> ::nserror::nsresult,

    /* void createFunction (in AUTF8String aFunctionName, in long aNumArguments, in mozIStorageFunction aFunction); */
    pub CreateFunction: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aFunctionName: *const ::nsstring::nsACString, aNumArguments: i32, aFunction: *const mozIStorageFunction) -> ::nserror::nsresult,

    /* void removeFunction (in AUTF8String aFunctionName); */
    pub RemoveFunction: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aFunctionName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* mozIStorageProgressHandler setProgressHandler (in int32_t aGranularity, in mozIStorageProgressHandler aHandler); */
    pub SetProgressHandler: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, aGranularity: int32_t, aHandler: *const mozIStorageProgressHandler, _retval: *mut*const mozIStorageProgressHandler) -> ::nserror::nsresult,

    /* mozIStorageProgressHandler removeProgressHandler (); */
    pub RemoveProgressHandler: unsafe extern "system" fn (this: *const mozIStorageAsyncConnection, _retval: *mut*const mozIStorageProgressHandler) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIStorageAsyncConnection {
    /// ```text
    /// /**
    ///    * Transaction behavior constants.
    ///    */
    /// ```
    ///

    pub const TRANSACTION_DEFAULT: i64 = -1;


    pub const TRANSACTION_DEFERRED: i64 = 0;


    pub const TRANSACTION_IMMEDIATE: i64 = 1;


    pub const TRANSACTION_EXCLUSIVE: i64 = 2;

    /// ```text
    /// /**
    ///    * The default behavior for all transactions run on this connection. Defaults
    ///    * to `TRANSACTION_DEFERRED`, and can be overridden for individual
    ///    * transactions.
    ///    */
    /// ```
    ///

    /// `attribute int32_t defaultTransactionType;`
    #[inline]
    pub unsafe fn GetDefaultTransactionType(&self, aDefaultTransactionType: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetDefaultTransactionType)(self, aDefaultTransactionType)
    }


    /// ```text
    /// /**
    ///    * The default behavior for all transactions run on this connection. Defaults
    ///    * to `TRANSACTION_DEFERRED`, and can be overridden for individual
    ///    * transactions.
    ///    */
    /// ```
    ///

    /// `attribute int32_t defaultTransactionType;`
    #[inline]
    pub unsafe fn SetDefaultTransactionType(&self, aDefaultTransactionType: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultTransactionType)(self, aDefaultTransactionType)
    }


    /// ```text
    /// /**
    ///    * The maximum number of bound parameters for statements executed on this
    ///    * connection. If your statement has more params than this limit, you'll
    ///    * need to chunk them into multiple statements. See `PlacesUtils.chunkArray`
    ///    * and its callers in Places for examples of how to do this, or read on for
    ///    * an overview.
    ///    *
    ///    * Keep in mind that the variable limit is for the _total_ number of
    ///    * parameters, including ones bound by name (using the `:VVV`, `@VVV`, or
        ///    * `?VVV` syntax) and index (`?` and `?NNN`).
    ///    *
    ///    * This means, when chunking:
    ///    *
    ///    * - If you're binding 1 param per 1 value per chunk (for example, if you
        ///    *   have a list of GUIDs and a clause like `WHERE guid IN (?, ?, ?, ...)`,
        ///    *   your chunk length is just `variableLimit`.
        ///    * - If you're binding 1 param per 1 value per chunk, but using that
        ///    *   param in multiple positions in the query (for example, `WHERE url_hash
            ///    *   IN (hash(?1), hash(?2), ...) AND url IN (?1, ?2, ...)`), you can use the
        ///    *   `?NNN` syntax with a chunk length of `variableLimit`.
        ///    * - If you're binding N params per 1 value per chunk (for example, if you
            ///    *   have a list of items with GUIDs and parent GUIDs, and you want to bind
            ///    *   both), your chunk length is `variableLimit / N`, since you're binding
        ///    *   two params for each element.
        ///    * - If you're binding K params per L values per chunk, plus M fixed ones
        ///    *   (for example, `WHERE parentGuid = :parentGuid AND guid IN (?, ?, ...)`),
        ///    *   your chunk length is `variableLimit - M`, to ensure there's space for the
        ///    *   fixed variables.
        ///    *
        ///    * If you bind more params than this limit, `create{Async}Statement` will
        ///    * fail with a "too many SQL variables" error.
        ///    */
        /// ```
        ///

        /// `readonly attribute int32_t variableLimit;`
        #[inline]
        pub unsafe fn GetVariableLimit(&self, aVariableLimit: *mut int32_t) -> ::nserror::nsresult {
            ((*self.vtable).GetVariableLimit)(self, aVariableLimit)
        }


        /// ```text
        /// /**
        ///    * Returns true if a transaction is active on this connection.
        ///    *
        ///    * Note that this is true if a transaction is active on the connection,
        ///    * regardless of how it was opened. There are several ways to open one:
        ///    *
        ///    * 1. Explicitly calling `beginTransaction` on a `mozIStorageConnection`.
        ///    * 2. Calling `executeSimpleSQL("BEGIN")` or
        ///    *    `createStatement("BEGIN").execute()` on a `mozIStorageConnection`.
        ///    * 3. Executing an async statement, like
        ///    *    `createAsyncStatement("BEGIN").executeAsync(...)`. This is what
        ///    *    `Sqlite.jsm` does under the hood.
        ///    *
        ///    * Because of this, it's important *not* to use this attribute to decide
        ///    * whether to *commit* the active transaction, because the caller that opened
        ///    * it may not expect that. This is why both `mozStorageTransaction` and
        ///    * `Sqlite.jsm` use an internal variable (`mHasTransaction` for the former;
            ///    * `_hasInProgressTransaction` for the latter) to check if their transaction
        ///    * is already in progress, instead of just checking this attribute before
        ///    * committing. Otherwise, mozStorage might accidentally commit (or roll back!)
        ///    * a transaction started by `Sqlite.jsm`, and vice versa.
        ///    */
        /// ```
        ///

        /// `readonly attribute boolean transactionInProgress;`
        #[inline]
        pub unsafe fn GetTransactionInProgress(&self, aTransactionInProgress: *mut bool) -> ::nserror::nsresult {
            ((*self.vtable).GetTransactionInProgress)(self, aTransactionInProgress)
        }


        /// ```text
        /// /**
        ///    * Close this database connection, allowing all pending statements
        ///    * to complete first.
        ///    *
        ///    * @param aCallback [optional]
        ///    *        A callback that will be notified when the close is completed,
        ///    *        with the following arguments:
        ///    *        - status: the status of the call
        ///    *        - value: |null|
        ///    *
        ///    * @throws NS_ERROR_NOT_SAME_THREAD
        ///    *         If called on a thread other than the one that opened it.  The
        ///    *         callback will not be dispatched.
        ///    * @throws NS_ERROR_NOT_INITIALIZED
        ///    *         If called on a connection that has already been closed or was
        ///    *         never properly opened.  The callback will still be dispatched
        ///    *         to the main thread despite the returned error.
        ///    * @note If this call should fail, the callback won't be invoked.
        ///    */
        /// ```
        ///

        /// `void asyncClose ([optional] in mozIStorageCompletionCallback aCallback);`
        #[inline]
        pub unsafe fn AsyncClose(&self, aCallback: *const mozIStorageCompletionCallback) -> ::nserror::nsresult {
            ((*self.vtable).AsyncClose)(self, aCallback)
        }


        /// ```text
        /// /**
        ///    * Forcibly closes a database connection synchronously.
        ///    * This should only be used when it's required to close and replace the
        ///    * database synchronously to return control to the consumer, for example in
        ///    * case of a detected corruption on database opening.
        ///    * Since this spins the events loop, it should be used only in very particular
        ///    * and rare situations, or it may cause unexpected consequences (crashes).
        ///    *
        ///    * @throws NS_ERROR_NOT_SAME_THREAD
        ///    *         If called on a thread other than the one that opened it.
        ///    */
        /// ```
        ///

        /// `[noscript] void spinningSynchronousClose ();`
        #[inline]
        pub unsafe fn SpinningSynchronousClose(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).SpinningSynchronousClose)(self, )
        }


        /// ```text
        /// /**
        ///    * Clone a database and make the clone read only if needed.
        ///    * SQL Functions and attached on-disk databases are applied to the new clone.
        ///    *
        ///    * @param aReadOnly
        ///    *        If true, the returned database should be put into read-only mode.
        ///    *
        ///    * @param aCallback
        ///    *        A callback that will be notified when the operation is complete,
        ///    *        with the following arguments:
        ///    *        - status: the status of the operation
        ///    *        - value: in case of success, an intance of
        ///    *             mozIStorageAsyncConnection cloned from this one.
        ///    *
        ///    * @throws NS_ERROR_NOT_SAME_THREAD
        ///    *         If is called on a thread other than the one that opened it.
        ///    * @throws NS_ERROR_UNEXPECTED
        ///    *         If this connection is a memory database.
        ///    *
        ///    * @note If your connection is already read-only, you will get a read-only
        ///    *       clone.
        ///    * @note The resulting connection will implement `mozIStorageConnection`, but
        ///    *       all synchronous methods will throw if called from the main thread.
        ///    * @note Due to a bug in SQLite, if you use the shared cache
        ///    *       (see mozIStorageService), you end up with the same privileges as the
        ///    *       first connection opened regardless of what is specified in aReadOnly.
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
        ///    */
        /// ```
        ///

        /// `void asyncClone (in boolean aReadOnly, in mozIStorageCompletionCallback aCallback);`
        #[inline]
        pub unsafe fn AsyncClone(&self, aReadOnly: bool, aCallback: *const mozIStorageCompletionCallback) -> ::nserror::nsresult {
            ((*self.vtable).AsyncClone)(self, aReadOnly, aCallback)
        }


        /// ```text
        /// /**
        ///    * The current database nsIFile.  Null if the database
        ///    * connection refers to an in-memory database.
        ///    */
        /// ```
        ///

        /// `readonly attribute nsIFile databaseFile;`
        #[inline]
        pub unsafe fn GetDatabaseFile(&self, aDatabaseFile: *mut*const nsIFile) -> ::nserror::nsresult {
            ((*self.vtable).GetDatabaseFile)(self, aDatabaseFile)
        }


        /// ```text
        /// /**
        ///    * Causes any pending database operation to abort and return at the first
        ///    * opportunity.
        ///    * This can only be used on read-only connections that don't implement
        ///    * the mozIStorageConnection interface.
        ///    * @note operations that are nearly complete may still be able to complete.
        ///    * @throws if used on an unsupported connection type, or a closed connection.
        ///    */
        /// ```
        ///

        /// `void interrupt ();`
        #[inline]
        pub unsafe fn Interrupt(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).Interrupt)(self, )
        }


        /// ```text
        /// /**
        ///    * Create an asynchronous statement for the given SQL. An
        ///    * asynchronous statement can only be used to dispatch asynchronous
        ///    * requests to the asynchronous execution thread and cannot be used
        ///    * to take any synchronous actions on the database.
        ///    *
        ///    * The expression may use ? to indicate sequential numbered arguments,
        ///    * ?1, ?2 etc. to indicate specific numbered arguments or :name and
        ///    * $var to indicate named arguments.
        ///    *
        ///    * @param aSQLStatement
        ///    *        The SQL statement to execute.
        ///    * @return a new mozIStorageAsyncStatement
        ///    * @note The statement is created lazily on first execution.
        ///    */
        /// ```
        ///

        /// `mozIStorageAsyncStatement createAsyncStatement (in AUTF8String aSQLStatement);`
        #[inline]
        pub unsafe fn CreateAsyncStatement(&self, aSQLStatement: *const ::nsstring::nsACString, _retval: *mut*const mozIStorageAsyncStatement) -> ::nserror::nsresult {
            ((*self.vtable).CreateAsyncStatement)(self, aSQLStatement, _retval)
        }


        /// ```text
        /// /**
        ///    * Execute an array of statements created with this connection using
        ///    * any currently bound parameters. When the array contains multiple
        ///    * statements, the execution is wrapped in a single
        ///    * transaction. These statements can be reused immediately, and
        ///    * reset does not need to be called.
        ///    *
        ///    * @param aStatements
        ///    *        The array of statements to execute asynchronously, in the order they
        ///    *        are given in the array.
        ///    * @param aCallback [optional]
        ///    *        The callback object that will be notified of progress, errors, and
        ///    *        completion.
        ///    * @return an object that can be used to cancel the statements execution.
        ///    *
        ///    * @note If you have any custom defined functions, they must be
        ///    *        re-entrant since they can be called on multiple threads.
        ///    */
        /// ```
        ///

        /// `mozIStoragePendingStatement executeAsync (in Array<mozIStorageBaseStatement> aStatements, [optional] in mozIStorageStatementCallback aCallback);`
        #[inline]
        pub unsafe fn ExecuteAsync(&self, aStatements: *const thin_vec::ThinVec<RefPtr<mozIStorageBaseStatement>>, aCallback: *const mozIStorageStatementCallback, _retval: *mut*const mozIStoragePendingStatement) -> ::nserror::nsresult {
            ((*self.vtable).ExecuteAsync)(self, aStatements, aCallback, _retval)
        }


        /// ```text
        /// /**
        ///    * Execute asynchronously an SQL expression, expecting no arguments.
        ///    *
        ///    * @param aSQLStatement
        ///    *        The SQL statement to execute
        ///    * @param aCallback [optional]
        ///    *        The callback object that will be notified of progress, errors, and
        ///    *        completion.
        ///    * @return an object that can be used to cancel the statement execution.
        ///    */
        /// ```
        ///

        /// `mozIStoragePendingStatement executeSimpleSQLAsync (in AUTF8String aSQLStatement, [optional] in mozIStorageStatementCallback aCallback);`
        #[inline]
        pub unsafe fn ExecuteSimpleSQLAsync(&self, aSQLStatement: *const ::nsstring::nsACString, aCallback: *const mozIStorageStatementCallback, _retval: *mut*const mozIStoragePendingStatement) -> ::nserror::nsresult {
            ((*self.vtable).ExecuteSimpleSQLAsync)(self, aSQLStatement, aCallback, _retval)
        }


        /// ```text
        /// /**
        ///    * Create a new SQL function.  If you use your connection on multiple threads,
        ///    * your function needs to be threadsafe, or it should only be called on one
        ///    * thread.
        ///    *
        ///    * @param aFunctionName
        ///    *        The name of function to create, as seen in SQL.
        ///    * @param aNumArguments
        ///    *        The number of arguments the function takes. Pass -1 for
        ///    *        variable-argument functions.
        ///    * @param aFunction
        ///    *        The instance of mozIStorageFunction, which implements the function
        ///    *        in question.
        ///    */
        /// ```
        ///

        /// `void createFunction (in AUTF8String aFunctionName, in long aNumArguments, in mozIStorageFunction aFunction);`
        #[inline]
        pub unsafe fn CreateFunction(&self, aFunctionName: *const ::nsstring::nsACString, aNumArguments: i32, aFunction: *const mozIStorageFunction) -> ::nserror::nsresult {
            ((*self.vtable).CreateFunction)(self, aFunctionName, aNumArguments, aFunction)
        }


        /// ```text
        /// /**
        ///    * Delete custom SQL function.
        ///    *
        ///    * @param aFunctionName
        ///    *        The name of function to remove.
        ///    */
        /// ```
        ///

        /// `void removeFunction (in AUTF8String aFunctionName);`
        #[inline]
        pub unsafe fn RemoveFunction(&self, aFunctionName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).RemoveFunction)(self, aFunctionName)
        }


        /// ```text
        /// /**
        ///    * Sets a progress handler. Only one handler can be registered at a time.
        ///    * If you need more than one, you need to chain them yourself.  This progress
        ///    * handler should be threadsafe if you use this connection object on more than
        ///    * one thread.
        ///    *
        ///    * @param aGranularity
        ///    *        The number of SQL virtual machine steps between progress handler
        ///    *        callbacks.
        ///    * @param aHandler
        ///    *        The instance of mozIStorageProgressHandler.
        ///    * @return previous registered handler.
        ///    */
        /// ```
        ///

        /// `mozIStorageProgressHandler setProgressHandler (in int32_t aGranularity, in mozIStorageProgressHandler aHandler);`
        #[inline]
        pub unsafe fn SetProgressHandler(&self, aGranularity: int32_t, aHandler: *const mozIStorageProgressHandler, _retval: *mut*const mozIStorageProgressHandler) -> ::nserror::nsresult {
            ((*self.vtable).SetProgressHandler)(self, aGranularity, aHandler, _retval)
        }


        /// ```text
        /// /**
        ///    * Remove a progress handler.
        ///    *
        ///    * @return previous registered handler.
        ///    */
        /// ```
        ///

        /// `mozIStorageProgressHandler removeProgressHandler ();`
        #[inline]
        pub unsafe fn RemoveProgressHandler(&self, _retval: *mut*const mozIStorageProgressHandler) -> ::nserror::nsresult {
            ((*self.vtable).RemoveProgressHandler)(self, _retval)
        }


    }


