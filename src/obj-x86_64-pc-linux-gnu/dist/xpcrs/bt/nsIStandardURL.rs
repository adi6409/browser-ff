//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIStandardURL.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIStandardURL",
            base: Some("nsISupports"),
            methods: Ok(&[
                    ]),
        },

        Interface {
            name: "nsIStandardURLMutator",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* nsIURIMutator init (in unsigned long aUrlType, in long aDefaultPort, in AUTF8String aSpec, in string aOriginCharset, in nsIURI aBaseURI); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aUrlType", ty: "u32" }, Param { name: "aDefaultPort", ty: "i32" }, Param { name: "aSpec", ty: "*const ::nsstring::nsACString" }, Param { name: "aOriginCharset", ty: "*const libc::c_char" }, Param { name: "aBaseURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut*const nsIURIMutator" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIURIMutator setDefaultPort (in long aNewDefaultPort); */
                    Method {
                        name: "SetDefaultPort",
                        params: &[Param { name: "aNewDefaultPort", ty: "i32" }, Param { name: "_retval", ty: "*mut*const nsIURIMutator" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

