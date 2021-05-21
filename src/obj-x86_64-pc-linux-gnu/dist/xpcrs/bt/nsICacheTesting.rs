//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheTesting.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheTesting",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void suspendCacheIOThread (in uint32_t aLevel); */
                    Method {
                        name: "SuspendCacheIOThread",
                        params: &[Param { name: "aLevel", ty: "uint32_t" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void resumeCacheIOThread (); */
                    Method {
                        name: "ResumeCacheIOThread",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void flush (in nsIObserver aObserver); */
                    Method {
                        name: "Flush",
                        params: &[Param { name: "aObserver", ty: "*const nsIObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

