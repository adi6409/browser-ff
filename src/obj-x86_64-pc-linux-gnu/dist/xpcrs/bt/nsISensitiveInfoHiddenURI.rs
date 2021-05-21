//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISensitiveInfoHiddenURI.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISensitiveInfoHiddenURI",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AUTF8String getSensitiveInfoHiddenSpec (); */
                    Method {
                        name: "GetSensitiveInfoHiddenSpec",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

