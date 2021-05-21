//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIAsyncStreamCopier2.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIAsyncStreamCopier2",
            base: Some("nsIRequest"),
            methods: Ok(&[
                    /* void init (in nsIInputStream aSource, in nsIOutputStream aSink, in nsIEventTarget aTarget, in unsigned long aChunkSize, in boolean aCloseSource, in boolean aCloseSink); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aSource", ty: "*const nsIInputStream" }, Param { name: "aSink", ty: "*const nsIOutputStream" }, Param { name: "aTarget", ty: "*const nsIEventTarget" }, Param { name: "aChunkSize", ty: "u32" }, Param { name: "aCloseSource", ty: "bool" }, Param { name: "aCloseSink", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void asyncCopy (in nsIRequestObserver aObserver, in nsISupports aObserverContext); */
                    Method {
                        name: "AsyncCopy",
                        params: &[Param { name: "aObserver", ty: "*const nsIRequestObserver" }, Param { name: "aObserverContext", ty: "*const nsISupports" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

