//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageError.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageError",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute long result; */
                    Method {
                        name: "GetResult",
                        params: &[Param { name: "aResult", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String message; */
                    Method {
                        name: "GetMessage",
                        params: &[Param { name: "aMessage", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

