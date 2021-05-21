//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/antitracking/nsIPartitioningExceptionListService.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsIPartitioningExceptionListObserver",
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
            name: "nsIPartitioningExceptionListService",
            base: Some("nsISupports"),
            methods: Ok(&[
                    /* void registerAndRunExceptionListObserver (in nsIPartitioningExceptionListObserver aObserver); */
                    Method {
                        name: "RegisterAndRunExceptionListObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsIPartitioningExceptionListObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    /* void unregisterExceptionListObserver (in nsIPartitioningExceptionListObserver aObserver); */
                    Method {
                        name: "UnregisterExceptionListObserver",
                        params: &[Param { name: "aObserver", ty: "*const nsIPartitioningExceptionListObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

