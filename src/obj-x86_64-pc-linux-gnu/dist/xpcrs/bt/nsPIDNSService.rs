//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/dns/nsPIDNSService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsPIDNSService",
            base: Some("nsIDNSService"),
            methods: Ok(&[
                    /* void init (); */
                    Method {
                        name: "Init",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* void shutdown (); */
                    Method {
                        name: "Shutdown",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    /* attribute boolean prefetchEnabled; */
                    Method {
                        name: "GetPrefetchEnabled",
                        params: &[Param { name: "aPrefetchEnabled", ty: "*mut bool" }],
                        ret: "::nserror::nsresult",
                    },
                    Method {
                        name: "SetPrefetchEnabled",
                        params: &[Param { name: "aPrefetchEnabled", ty: "bool" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

