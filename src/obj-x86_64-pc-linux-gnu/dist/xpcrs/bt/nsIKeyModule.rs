//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIKeyModule.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIKeyObject",
            base: Some("nsISupports"),
            methods: Err("native type PK11SymKey unsupported"),
        },

        Interface {
            name: "nsIKeyObjectFactory",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] nsIKeyObject keyFromString (in short aAlgorithm, in ACString aKey); */
                    Method {
                        name: "KeyFromString",
                        params: &[Param { name: "aAlgorithm", ty: "i16" }, Param { name: "aKey", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut *const nsIKeyObject" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

