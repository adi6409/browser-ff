//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageStatementCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageStatementCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void handleResult (in mozIStorageResultSet aResultSet); */
                    Method {
                        name: "HandleResult",
                        params: &[Param { name: "aResultSet", ty: "*const mozIStorageResultSet" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleError (in mozIStorageError aError); */
                    Method {
                        name: "HandleError",
                        params: &[Param { name: "aError", ty: "*const mozIStorageError" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void handleCompletion (in unsigned short aReason); */
                    Method {
                        name: "HandleCompletion",
                        params: &[Param { name: "aReason", ty: "u16" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

