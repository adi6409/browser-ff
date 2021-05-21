//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/base/nsIConsoleMessage.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIConsoleMessage",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute uint32_t logLevel; */
                    Method {
                        name: "GetLogLevel",
                        params: &[Param { name: "aLogLevel", ty: "*mut uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute long long timeStamp; */
                    Method {
                        name: "GetTimeStamp",
                        params: &[Param { name: "aTimeStamp", ty: "*mut i64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [binaryname(MessageMoz)] readonly attribute AString message; */
                    Method {
                        name: "GetMessageMoz",
                        params: &[Param { name: "aMessage", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* AUTF8String toString (); */
                    Method {
                        name: "ToString",
                        params: &[Param { name: "_retval", ty: "*mut ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

