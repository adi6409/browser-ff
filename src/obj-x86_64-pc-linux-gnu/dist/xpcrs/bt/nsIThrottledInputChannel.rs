//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIThrottledInputChannel.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIInputChannelThrottleQueue",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void init (in unsigned long aMeanBytesPerSecond, in unsigned long aMaxBytesPerSecond); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aMeanBytesPerSecond", ty: "u32" }, Param { name: "aMaxBytesPerSecond", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] readonly attribute unsigned long meanBytesPerSecond; */
                    Method {
                        name: "GetMeanBytesPerSecond",
                        params: &[Param { name: "aMeanBytesPerSecond", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* [noscript] readonly attribute unsigned long maxBytesPerSecond; */
                    Method {
                        name: "GetMaxBytesPerSecond",
                        params: &[Param { name: "aMaxBytesPerSecond", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long available (in unsigned long aRemaining); */
                    Method {
                        name: "Available",
                        params: &[Param { name: "aRemaining", ty: "u32" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void recordRead (in unsigned long aBytesRead); */
                    Method {
                        name: "RecordRead",
                        params: &[Param { name: "aBytesRead", ty: "u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* unsigned long long bytesProcessed (); */
                    Method {
                        name: "BytesProcessed",
                        params: &[Param { name: "_retval", ty: "*mut u64" }],
                        ret: "::nserror::nsresult",
                    },

                    /* nsIAsyncInputStream wrapStream (in nsIInputStream aInputStream); */
                    Method {
                        name: "WrapStream",
                        params: &[Param { name: "aInputStream", ty: "*const nsIInputStream" }, Param { name: "_retval", ty: "*mut*const nsIAsyncInputStream" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIThrottledInputChannel",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute nsIInputChannelThrottleQueue throttleQueue; */
                    Method {
                        name: "GetThrottleQueue",
                        params: &[Param { name: "aThrottleQueue", ty: "*mut *const nsIInputChannelThrottleQueue" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetThrottleQueue",
                        params: &[Param { name: "aThrottleQueue", ty: "*const nsIInputChannelThrottleQueue" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

