//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/url-classifier/nsIUrlClassifierExceptionListService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIUrlClassifierExceptionListObserver",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void onExceptionListUpdate (in ACString aList); */
                    Method {
                        name: "OnExceptionListUpdate",
                        params: &[Param { name: "aList", ty: "*const ::nsstring::nsACString" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        Interface {
            name: "nsIUrlClassifierExceptionListService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void registerAndRunExceptionListObserver (in ACString aFeature, in ACString aPrefName, in nsIUrlClassifierExceptionListObserver aObserver); */
                    Method {
                        name: "RegisterAndRunExceptionListObserver",
                        params: &[Param { name: "aFeature", ty: "*const ::nsstring::nsACString" }, Param { name: "aPrefName", ty: "*const ::nsstring::nsACString" }, Param { name: "aObserver", ty: "*const nsIUrlClassifierExceptionListObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterExceptionListObserver (in ACString aFeature, in nsIUrlClassifierExceptionListObserver aObserver); */
                    Method {
                        name: "UnregisterExceptionListObserver",
                        params: &[Param { name: "aFeature", ty: "*const ::nsstring::nsACString" }, Param { name: "aObserver", ty: "*const nsIUrlClassifierExceptionListObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void clear (); */
                    Method {
                        name: "Clear",
                        params: &[],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

