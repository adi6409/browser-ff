//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIExternalProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIExternalProtocolHandler",
            base: Some("nsIProtocolHandler"),
            methods: Ok(&[
                    /* boolean externalAppExistsForScheme (in ACString scheme); */
                    Method {
                        name: "ExternalAppExistsForScheme",
                        params: &[Param { name: "scheme", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

