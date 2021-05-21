//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache/nsICacheListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheListener",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onCacheEntryAvailable (in nsICacheEntryDescriptor descriptor, in nsCacheAccessMode accessGranted, in nsresult status); */
                    Method {
                        name: "OnCacheEntryAvailable",
                        params: &[Param { name: "descriptor", ty: "*const nsICacheEntryDescriptor" }, Param { name: "accessGranted", ty: "nsCacheAccessMode" }, Param { name: "status", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void onCacheEntryDoomed (in nsresult status); */
                    Method {
                        name: "OnCacheEntryDoomed",
                        params: &[Param { name: "status", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

