//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/remote/components/nsIRemoteAgent.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRemoteAgent",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* readonly attribute AString debuggerAddress; */
                    Method {
                        name: "GetDebuggerAddress",
                        params: &[Param { name: "aDebuggerAddress", ty: "*mut ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute boolean listening; */
                    Method {
                        name: "GetListening",
                        params: &[Param { name: "aListening", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void listen (in AString aURL); */
                    Method {
                        name: "Listen",
                        params: &[Param { name: "aURL", ty: "*const ::nsstring::nsAString" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void close (); */
                    Method {
                        name: "Close",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

