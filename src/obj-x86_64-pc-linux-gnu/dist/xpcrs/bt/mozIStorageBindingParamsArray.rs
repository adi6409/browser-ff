//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageBindingParamsArray.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageBindingParamsArray",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* mozIStorageBindingParams newBindingParams (); */
                    Method {
                        name: "NewBindingParams",
                        params: &[Param { name: "_retval", ty: "*mut*const mozIStorageBindingParams" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void addParams (in mozIStorageBindingParams aParameters); */
                    Method {
                        name: "AddParams",
                        params: &[Param { name: "aParameters", ty: "*const mozIStorageBindingParams" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long length; */
                    Method {
                        name: "GetLength",
                        params: &[Param { name: "aLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

