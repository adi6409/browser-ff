//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/io/nsIInputStreamTee.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputStreamTee",
            base: Some("nsIInputStream"),
            methods: Ok(&[
                    /* attribute nsIInputStream source; */
                    Method {
                        name: "GetSource",
                        params: &[Param { name: "aSource", ty: "*mut *const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSource",
                        params: &[Param { name: "aSource", ty: "*const nsIInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIOutputStream sink; */
                    Method {
                        name: "GetSink",
                        params: &[Param { name: "aSink", ty: "*mut*const nsIOutputStream" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetSink",
                        params: &[Param { name: "aSink", ty: "*const nsIOutputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute nsIEventTarget eventTarget; */
                    Method {
                        name: "GetEventTarget",
                        params: &[Param { name: "aEventTarget", ty: "*mut*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetEventTarget",
                        params: &[Param { name: "aEventTarget", ty: "*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

