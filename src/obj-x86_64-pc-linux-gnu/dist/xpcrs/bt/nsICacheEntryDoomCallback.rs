//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cache2/nsICacheEntryDoomCallback.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsICacheEntryDoomCallback",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onCacheEntryDoomed (in nsresult aResult); */
                    Method {
                        name: "OnCacheEntryDoomed",
                        params: &[Param { name: "aResult", ty: "::nserror::nsresult" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

