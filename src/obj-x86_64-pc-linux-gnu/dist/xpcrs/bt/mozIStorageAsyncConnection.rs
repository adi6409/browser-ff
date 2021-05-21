//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageAsyncConnection.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageAsyncConnection",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute int32_t defaultTransactionType; */
                    Method {
                        name: "GetDefaultTransactionType",
                        params: &[Param { name: "aDefaultTransactionType", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetDefaultTransactionType",
                        params: &[Param { name: "aDefaultTransactionType", ty: "int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute int32_t variableLimit; */
                    Method {
                        name: "GetVariableLimit",
                        params: &[Param { name: "aVariableLimit", ty: "*mut int32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean transactionInProgress; */
                    Method {
                        name: "GetTransactionInProgress",
                        params: &[Param { name: "aTransactionInProgress", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncClose ([optional] in mozIStorageCompletionCallback aCallback); */
                    Method {
                        name: "AsyncClose",
                        params: &[Param { name: "aCallback", ty: "*const mozIStorageCompletionCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] void spinningSynchronousClose (); */
                    Method {
                        name: "SpinningSynchronousClose",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncClone (in boolean aReadOnly, in mozIStorageCompletionCallback aCallback); */
                    Method {
                        name: "AsyncClone",
                        params: &[Param { name: "aReadOnly", ty: "bool" }, Param { name: "aCallback", ty: "*const mozIStorageCompletionCallback" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIFile databaseFile; */
                    Method {
                        name: "GetDatabaseFile",
                        params: &[Param { name: "aDatabaseFile", ty: "*mut*const nsIFile" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void interrupt (); */
                    Method {
                        name: "Interrupt",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStorageAsyncStatement createAsyncStatement (in AUTF8String aSQLStatement); */
                    Method {
                        name: "CreateAsyncStatement",
                        params: &[Param { name: "aSQLStatement", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const mozIStorageAsyncStatement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStoragePendingStatement executeAsync (in Array<mozIStorageBaseStatement> aStatements, [optional] in mozIStorageStatementCallback aCallback); */
                    Method {
                        name: "ExecuteAsync",
                        params: &[Param { name: "aStatements", ty: "*const thin_vec::ThinVec<RefPtr<mozIStorageBaseStatement>>" }, Param { name: "aCallback", ty: "*const mozIStorageStatementCallback" }, Param { name: "_retval", ty: "*mut*const mozIStoragePendingStatement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStoragePendingStatement executeSimpleSQLAsync (in AUTF8String aSQLStatement, [optional] in mozIStorageStatementCallback aCallback); */
                    Method {
                        name: "ExecuteSimpleSQLAsync",
                        params: &[Param { name: "aSQLStatement", ty: "*const ::nsstring::nsACString" }, Param { name: "aCallback", ty: "*const mozIStorageStatementCallback" }, Param { name: "_retval", ty: "*mut*const mozIStoragePendingStatement" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void createFunction (in AUTF8String aFunctionName, in long aNumArguments, in mozIStorageFunction aFunction); */
                    Method {
                        name: "CreateFunction",
                        params: &[Param { name: "aFunctionName", ty: "*const ::nsstring::nsACString" }, Param { name: "aNumArguments", ty: "i32" }, Param { name: "aFunction", ty: "*const mozIStorageFunction" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void removeFunction (in AUTF8String aFunctionName); */
                    Method {
                        name: "RemoveFunction",
                        params: &[Param { name: "aFunctionName", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStorageProgressHandler setProgressHandler (in int32_t aGranularity, in mozIStorageProgressHandler aHandler); */
                    Method {
                        name: "SetProgressHandler",
                        params: &[Param { name: "aGranularity", ty: "int32_t" }, Param { name: "aHandler", ty: "*const mozIStorageProgressHandler" }, Param { name: "_retval", ty: "*mut*const mozIStorageProgressHandler" }],
                        ret: "::nserror::nsresult",
                    },

                    /* mozIStorageProgressHandler removeProgressHandler (); */
                    Method {
                        name: "RemoveProgressHandler",
                        params: &[Param { name: "_retval", ty: "*mut*const mozIStorageProgressHandler" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

