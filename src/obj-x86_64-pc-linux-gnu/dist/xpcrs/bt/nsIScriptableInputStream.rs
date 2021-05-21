//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIScriptableInputStream.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIScriptableInputStream",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void close (); */
                    Method {
                        name: "Close",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void init (in nsIInputStream aInputStream); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aInputStream", ty: "*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long long available (); */
                    Method {
                        name: "Available",
                        params: &[Param { name: "_retval", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* string read (in unsigned long aCount); */
                    Method {
                        name: "Read",
                        params: &[Param { name: "aCount", ty: "u32" }, Param { name: "_retval", ty: "*mut *const libc::c_char" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString readBytes (in unsigned long aCount); */
                    Method {
                        name: "ReadBytes",
                        params: &[Param { name: "aCount", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

