//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIInputStreamPump.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputStreamPump",
            base: Some("nsIRequest"),
            methods: Ok(&[
                    /* void init (in nsIInputStream aStream, in unsigned long aSegmentSize, in unsigned long aSegmentCount, in boolean aCloseWhenDone, [optional] in nsIEventTarget aMainThreadTarget); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aStream", ty: "*const nsIInputStream" }, Param { name: "aSegmentSize", ty: "u32" }, Param { name: "aSegmentCount", ty: "u32" }, Param { name: "aCloseWhenDone", ty: "bool" }, Param { name: "aMainThreadTarget", ty: "*const nsIEventTarget" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncRead (in nsIStreamListener aListener); */
                    Method {
                        name: "AsyncRead",
                        params: &[Param { name: "aListener", ty: "*const nsIStreamListener" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

