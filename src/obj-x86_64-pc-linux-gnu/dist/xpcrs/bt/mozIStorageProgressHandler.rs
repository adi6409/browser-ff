//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageProgressHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageProgressHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* boolean onProgress (in mozIStorageConnection aConnection); */
                    Method {
                        name: "OnProgress",
                        params: &[Param { name: "aConnection", ty: "*const mozIStorageConnection" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

