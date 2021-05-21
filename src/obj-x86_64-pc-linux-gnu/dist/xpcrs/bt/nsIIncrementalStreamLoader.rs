//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIIncrementalStreamLoader.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIIncrementalStreamLoaderObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onIncrementalData (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in unsigned long dataLength, [array, size_is (dataLength), const] in octet data, inout unsigned long consumedLength); */
                    Method {
                        name: "OnIncrementalData",
                        params: &[Param { name: "loader", ty: "*const nsIIncrementalStreamLoader" }, Param { name: "ctxt", ty: "*const nsISupports" }, Param { name: "dataLength", ty: "u32" }, Param { name: "data", ty: "*const u8" }, Param { name: "consumedLength", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onStreamComplete (in nsIIncrementalStreamLoader loader, in nsISupports ctxt, in nsresult status, in unsigned long resultLength, [array, size_is (resultLength), const] in octet result); */
                    Method {
                        name: "OnStreamComplete",
                        params: &[Param { name: "loader", ty: "*const nsIIncrementalStreamLoader" }, Param { name: "ctxt", ty: "*const nsISupports" }, Param { name: "status", ty: "::nserror::nsresult" }, Param { name: "resultLength", ty: "u32" }, Param { name: "result", ty: "*const u8" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIIncrementalStreamLoader",
            base: Some("nsIStreamListener"),
            methods: Ok(&[
                    /* void init (in nsIIncrementalStreamLoaderObserver aObserver); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aObserver", ty: "*const nsIIncrementalStreamLoaderObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute unsigned long numBytesRead; */
                    Method {
                        name: "GetNumBytesRead",
                        params: &[Param { name: "aNumBytesRead", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* readonly attribute nsIRequest request; */
                    Method {
                        name: "GetRequest",
                        params: &[Param { name: "aRequest", ty: "*mut*const nsIRequest" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

