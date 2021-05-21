//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIDHCPClient.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIDHCPClient",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* ACString getOption (in uint8_t option); */
                    Method {
                        name: "GetOption",
                        params: &[Param { name: "option", ty: "uint8_t" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

