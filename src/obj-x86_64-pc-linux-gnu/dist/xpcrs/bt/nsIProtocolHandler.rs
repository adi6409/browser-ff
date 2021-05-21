//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIProtocolHandler.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIProtocolHandlerWithDynamicFlags",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* unsigned long getFlagsForURI (in nsIURI aURI); */
                    Method {
                        name: "GetFlagsForURI",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIProtocolHandler",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute ACString scheme; */
                    Method {
                        name: "GetScheme",
                        params: &[Param { name: "aScheme", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long defaultPort; */
                    Method {
                        name: "GetDefaultPort",
                        params: &[Param { name: "aDefaultPort", ty: "*mut i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long protocolFlags; */
                    Method {
                        name: "GetProtocolFlags",
                        params: &[Param { name: "aProtocolFlags", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIChannel newChannel (in nsIURI aURI, in nsILoadInfo aLoadinfo); */
                    Method {
                        name: "NewChannel",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aLoadinfo", ty: "*const nsILoadInfo" }, Param { name: "_retval", ty: "*mut*const nsIChannel" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean allowPort (in long port, in string scheme); */
                    Method {
                        name: "AllowPort",
                        params: &[Param { name: "port", ty: "i32" }, Param { name: "scheme", ty: "*const libc::c_char" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

