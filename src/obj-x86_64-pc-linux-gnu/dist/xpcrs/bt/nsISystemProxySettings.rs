//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISystemProxySettings.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISystemProxySettings",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute bool mainThreadOnly; */
                    Method {
                        name: "GetMainThreadOnly",
                        params: &[Param { name: "aMainThreadOnly", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute AUTF8String PACURI; */
                    Method {
                        name: "GetPACURI",
                        params: &[Param { name: "aPACURI", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String getProxyForURI (in AUTF8String testSpec, in AUTF8String testScheme, in AUTF8String testHost, in int32_t testPort); */
                    Method {
                        name: "GetProxyForURI",
                        params: &[Param { name: "testSpec", ty: "*const ::nsstring::nsACString" }, Param { name: "testScheme", ty: "*const ::nsstring::nsACString" }, Param { name: "testHost", ty: "*const ::nsstring::nsACString" }, Param { name: "testPort", ty: "int32_t" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

