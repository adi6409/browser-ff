//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIURIMutator.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIURISetSpec",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* [must_use] nsIURIMutator setSpec (in AUTF8String aSpec); */
                    Method {
                        name: "SetSpec",
                        params: &[Param { name: "aSpec", ty: "*const ::nsstring::nsACString" }, Param { name: "_retval", ty: "*mut*const nsIURIMutator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIURISetters",
            base: Some("nsIURISetSpec"),
            methods: Err("native type const mozilla::Encoding unsupported"),
        },

        Interface {
            name: "nsIURIMutator",
            base: Some("nsIURISetters"),
            methods: Err("native type const mozilla::ipc::URIParams unsupported"),
        },

        ]; D}

