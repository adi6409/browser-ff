//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISimpleStreamListener.idl
//


{static D: &'static [Interface] = &[

        Interface {
            name: "nsISimpleStreamListener",
            base: Some("nsIStreamListener"),
            methods: Ok(&[
                    /* void init (in nsIOutputStream aSink, in nsIRequestObserver aObserver); */
                    Method {
                        name: "Init",
                        params: &[Param { name: "aSink", ty: "*const nsIOutputStream" }, Param { name: "aObserver", ty: "*const nsIRequestObserver" }],
                        ret: "::nserror::nsresult",
                    },

                    ]),
        },

        ]; D}

