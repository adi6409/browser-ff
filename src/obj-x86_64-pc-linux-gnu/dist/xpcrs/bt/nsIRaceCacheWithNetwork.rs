//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIRaceCacheWithNetwork.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIRaceCacheWithNetwork",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void test_triggerNetwork (in long timeout); */
                    Method {
                        name: "Test_triggerNetwork",
                        params: &[Param { name: "timeout", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void test_delayCacheEntryOpeningBy (in long timeout); */
                    Method {
                        name: "Test_delayCacheEntryOpeningBy",
                        params: &[Param { name: "timeout", ty: "i32" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void test_triggerDelayedOpenCacheEntry (); */
                    Method {
                        name: "Test_triggerDelayedOpenCacheEntry",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

