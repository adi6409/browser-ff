//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageResultSet.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageResultSet",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* mozIStorageRow getNextRow (); */
                    Method {
                        name: "GetNextRow",
                        params: &[Param { name: "_retval", ty: "*mut*const mozIStorageRow" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

