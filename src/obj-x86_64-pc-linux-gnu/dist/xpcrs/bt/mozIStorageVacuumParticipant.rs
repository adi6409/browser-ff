//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageVacuumParticipant.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageVacuumParticipant",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long expectedDatabasePageSize; */
                    Method {
                        name: "GetExpectedDatabasePageSize",
                        params: &[Param { name: "aExpectedDatabasePageSize", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute mozIStorageConnection databaseConnection; */
                    Method {
                        name: "GetDatabaseConnection",
                        params: &[Param { name: "aDatabaseConnection", ty: "*mut*const mozIStorageConnection" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean onBeginVacuum (); */
                    Method {
                        name: "OnBeginVacuum",
                        params: &[Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onEndVacuum (in boolean aSucceeded); */
                    Method {
                        name: "OnEndVacuum",
                        params: &[Param { name: "aSucceeded", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

