//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/storage/mozIStorageFunction.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "mozIStorageFunction",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIVariant onFunctionCall (in mozIStorageValueArray aFunctionArguments); */
                    Method {
                        name: "OnFunctionCall",
                        params: &[Param { name: "aFunctionArguments", ty: "*const mozIStorageValueArray" }, Param { name: "_retval", ty: "*mut*const nsIVariant" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

