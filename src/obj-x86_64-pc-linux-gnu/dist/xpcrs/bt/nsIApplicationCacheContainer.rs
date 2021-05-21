//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsIApplicationCacheContainer.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIApplicationCacheContainer",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* attribute nsIApplicationCache applicationCache; */
                    Method {
                        name: "GetApplicationCache",
                        params: &[Param { name: "aApplicationCache", ty: "*mut*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetApplicationCache",
                        params: &[Param { name: "aApplicationCache", ty: "*const nsIApplicationCache" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

