//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetUtil.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsINetUtil",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* AUTF8String parseRequestContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
                    Method {
                        name: "ParseRequestContentType",
                        params: &[Param { name: "aTypeHeader", ty: "*const ::nsstring::nsACString" }, Param { name: "aCharset", ty: "*mut ::nsstring::nsACString" }, Param { name: "aHadCharset", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String parseResponseContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out boolean aHadCharset); */
                    Method {
                        name: "ParseResponseContentType",
                        params: &[Param { name: "aTypeHeader", ty: "*const ::nsstring::nsACString" }, Param { name: "aCharset", ty: "*mut ::nsstring::nsACString" }, Param { name: "aHadCharset", ty: "*mut bool" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean protocolHasFlags (in nsIURI aURI, in unsigned long aFlag); */
                    Method {
                        name: "ProtocolHasFlags",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aFlag", ty: "u32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean URIChainHasFlags (in nsIURI aURI, in unsigned long aFlags); */
                    Method {
                        name: "URIChainHasFlags",
                        params: &[Param { name: "aURI", ty: "*const nsIURI" }, Param { name: "aFlags", ty: "u32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString escapeString (in ACString aString, in unsigned long aEscapeType); */
                    Method {
                        name: "EscapeString",
                        params: &[Param { name: "aString", ty: "*const ::nsstring::nsACString" }, Param { name: "aEscapeType", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString escapeURL (in ACString aStr, in unsigned long aFlags); */
                    Method {
                        name: "EscapeURL",
                        params: &[Param { name: "aStr", ty: "*const ::nsstring::nsACString" }, Param { name: "aFlags", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* ACString unescapeString (in AUTF8String aStr, in unsigned long aFlags); */
                    Method {
                        name: "UnescapeString",
                        params: &[Param { name: "aStr", ty: "*const ::nsstring::nsACString" }, Param { name: "aFlags", ty: "u32" }, Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* boolean extractCharsetFromContentType (in AUTF8String aTypeHeader, out AUTF8String aCharset, out long aCharsetStart, out long aCharsetEnd); */
                    Method {
                        name: "ExtractCharsetFromContentType",
                        params: &[Param { name: "aTypeHeader", ty: "*const ::nsstring::nsACString" }, Param { name: "aCharset", ty: "*mut ::nsstring::nsACString" }, Param { name: "aCharsetStart", ty: "*mut i32" }, Param { name: "aCharsetEnd", ty: "*mut i32" }, Param { name: "_retval", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void socketProcessTelemetryPing (); */
                    Method {
                        name: "SocketProcessTelemetryPing",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void notImplemented (); */
                    Method {
                        name: "NotImplemented",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

