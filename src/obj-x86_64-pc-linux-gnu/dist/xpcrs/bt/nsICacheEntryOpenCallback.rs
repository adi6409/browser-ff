//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheEntryOpenCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheEntryOpenCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* unsigned long onCacheEntryCheck (in nsICacheEntry aEntry, in nsIApplicationCache aApplicationCache); */
                    Method {
                        name: "OnCacheEntryCheck",
                        params: &[Param { name: "aEntry", ty: "*const nsICacheEntry" }, Param { name: "aApplicationCache", ty: "*const nsIApplicationCache" }, Param { name: "_retval", ty: "*mut u32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onCacheEntryAvailable (in nsICacheEntry aEntry, in boolean aNew, in nsIApplicationCache aApplicationCache, in nsresult aResult); */
                    Method {
                        name: "OnCacheEntryAvailable",
                        params: &[Param { name: "aEntry", ty: "*const nsICacheEntry" }, Param { name: "aNew", ty: "bool" }, Param { name: "aApplicationCache", ty: "*const nsIApplicationCache" }, Param { name: "aResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

