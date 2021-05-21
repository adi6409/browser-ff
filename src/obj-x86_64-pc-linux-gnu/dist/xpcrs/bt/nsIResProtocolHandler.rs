//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/res/nsIResProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIResProtocolHandler",
            base: Some("nsISubstitutingProtocolHandler"),
            methods: Ok(&[
                    /* boolean allowContentToAccess (in nsIURI url); */
                    Method {
                        name: "AllowContentToAccess",
                        params: &[Param { name: "url", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

