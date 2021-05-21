//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/security/manager/ssl/nsIClientAuthRememberService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIClientAuthRememberRecord",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString asciiHost; */
                    Method {
                        name: "GetAsciiHost",
                        params: &[Param { name: "aAsciiHost", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString fingerprint; */
                    Method {
                        name: "GetFingerprint",
                        params: &[Param { name: "aFingerprint", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString dbKey; */
                    Method {
                        name: "GetDbKey",
                        params: &[Param { name: "aDbKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute ACString entryKey; */
                    Method {
                        name: "GetEntryKey",
                        params: &[Param { name: "aEntryKey", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIClientAuthRememberService",
            base: Some("nsISupports"),
            methods: Err("native type const mozilla::OriginAttributes unsupported"),
        },

        ]; D}

